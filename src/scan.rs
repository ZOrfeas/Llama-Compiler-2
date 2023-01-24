use core::fmt;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::{FusedIterator, Peekable};
use std::rc::Rc;
use std::str::SplitWhitespace;

use log::error;
use thiserror::Error;
// !Consider Scanners that can read from stdin.
/// Simple file scanner that can read line by line and preprocess files.
pub struct Scanner {
    buffers: Vec<Buffer>,
    preprocess: bool,
    included_files: HashMap<Rc<String>, Option<Rc<String>>>,

    first_call: bool,
}
impl Scanner {
    pub fn new(filename: &str) -> ScanResult<Self> {
        let filename = Rc::new(filename.to_string());
        let mut scanner = Scanner {
            buffers: vec![Buffer::new(Rc::clone(&filename))?],
            preprocess: false,
            included_files: HashMap::new(), // error: None,
            first_call: true,
        };
        scanner.included_files.insert(Rc::clone(&filename), None);
        Ok(scanner)
    }
    // TODO: Consider removing ability to *not* preprocess.
    pub fn preprocess(mut self) -> Self {
        self.preprocess = true;
        self
    }

    fn read_line(&mut self) -> ScanResult<Option<Line>> {
        if self.first_call {
            // Possibly a bit hacky, since it incurs a cost on every call.
            self.first_call = false;
            return Ok(Some(Line::change_file(Rc::clone(
                &self
                    .buffers
                    .last()
                    .expect("should have exactly one buffer")
                    .filename,
            ))));
        }
        let mut line = String::new();
        while let Some(buf) = self.buffers.last_mut() {
            if let 0 = buf.read_line(&mut line)? {
                self.buffers.pop();
                if let Some(filename) = self.buffers.last().map(|b| &b.filename) {
                    return Ok(Some(Line::change_file(Rc::clone(filename))));
                }
                continue;
            }
            match (self.preprocess, to_directive(&line)?) {
                (true, Some(directive)) => {
                    buf.set_ows_empty();
                    if let Some(line) = self.handle_directive(directive)? {
                        return Ok(Some(line));
                    };
                }
                _ => return Ok(Some(buf.finalize_line(line))),
            }
            line.clear()
        }
        Ok(None)
    }
    fn handle_directive(&mut self, directive: Directive) -> ScanResult<Option<Line>> {
        match directive {
            Directive::Include(path) => {
                let path = Rc::new(path);
                self.push_file(Rc::clone(&path))?;
                return Ok(Some(Line::change_file(Rc::clone(&path))));
            }
        }
    }
    fn push_file(&mut self, to_include: Rc<String>) -> ScanResult<()> {
        if self.included_files.contains_key(&to_include) {
            let current_buf = self.get_current_buf();
            return Err(ScanErr::IncludeCycle {
                included_file: to_include.to_string(),
                in_file: current_buf.filename.to_string(),
                at_line: current_buf.lineno,
                prev_included_at: self.included_files[&to_include]
                    .as_ref()
                    .map(|s| s.to_string()),
            });
        }
        self.included_files.insert(
            Rc::clone(&to_include),
            Some(Rc::clone(&self.get_current_buf().filename)),
        );
        self.buffers.push(Buffer::new(Rc::clone(&to_include))?);
        Ok(())
    }
    fn get_current_buf(&self) -> &Buffer {
        self.buffers
            .last()
            .expect("If scanner is still running, buffer stack should not be empty")
    }
}
impl Iterator for Scanner {
    type Item = Line;
    fn next(&mut self) -> Option<Self::Item> {
        match self.read_line() {
            Ok(any) => any,
            Err(e) => {
                error!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
impl FusedIterator for Scanner {}

#[derive(Debug)]
pub enum Line {
    ChangeFile(Rc<String>),
    Line { text: Vec<u8>, lineno: usize },
}
impl Line {
    pub fn line(text: Vec<u8>, lineno: usize) -> Self {
        Line::Line { text, lineno }
    }
    pub fn change_file(path: Rc<String>) -> Self {
        Line::ChangeFile(path)
    }
}
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Line::ChangeFile(path) => write!(f, "{}", path),
            Line::Line { text, lineno: _ } => {
                write!(f, "{}", unsafe { std::str::from_utf8_unchecked(text) })
            }
            _ => Ok(()),
        }
    }
}
#[derive(Debug)]
struct Buffer {
    inner: BufReader<File>,
    filename: Rc<String>,
    lineno: usize,
    ows_empty: bool,
}
impl Buffer {
    fn new(filename: Rc<String>) -> ScanResult<Self> {
        let file = File::open(filename.as_ref())?;
        Ok(Self {
            inner: BufReader::new(file),
            filename,
            lineno: 0,
            ows_empty: false,
        })
    }
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        if self.ows_empty {
            self.ows_empty = false;
            buf.clear();
            buf.push('\n');
            Ok(1)
        } else {
            self.inner.read_line(buf)
        }
    }
    fn set_ows_empty(&mut self) {
        self.ows_empty = true;
    }
    fn finalize_line(&mut self, line: String) -> Line {
        self.lineno += 1;
        Line::line(line.into_bytes(), self.lineno)
    }
}

#[derive(Debug)]
enum Directive {
    Include(String),
}

fn to_directive(s: &String) -> ScanResult<Option<Directive>> {
    let mut words = s.split_whitespace().peekable();
    match words.next() {
        Some(word) if !word.starts_with("#") => Ok(None),
        Some(directive) => Ok(Some(decide_directive(directive, words)?)),
        None => Ok(None),
    }
}
fn decide_directive(first: &str, mut rest: Peekable<SplitWhitespace>) -> ScanResult<Directive> {
    match first {
        "#include" => match rest.next() {
            Some(filename) if rest.peek().is_none() => Ok(Directive::Include(
                filename[1..filename.len() - 1].to_string(),
            )),
            Some(_) => Err(ScanErr::IncludeTrailingArgs),
            None => Err(ScanErr::IncludeEmpty),
        },
        _ => Err(ScanErr::UnknownDirective(first.to_string())),
    }
}

type ScanResult<T> = Result<T, ScanErr>;
#[derive(Error, Debug)]
pub enum ScanErr {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Unknown directive: {0}")]
    UnknownDirective(String),

    #[error("Include directive has trailing arguments")]
    IncludeTrailingArgs,
    #[error("Include directive has no arguments")]
    IncludeEmpty,
    #[error("Include cycle found: {included_file} included again in {in_file} at line {at_line} {}",
        .prev_included_at.as_ref()
        .map(|x| format!("(previously included in {})", x))
        .unwrap_or("(it's possibly also the source file given to the compiler)".to_string())
    )]
    IncludeCycle {
        included_file: String,
        in_file: String,
        at_line: usize,
        prev_included_at: Option<String>,
    },
}
