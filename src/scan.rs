use core::fmt;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::Peekable;
use std::path::{Path, PathBuf};
use std::str::SplitWhitespace;

use log::error;
// !Consider Scanners that can read from stdin.

/// Simple file scanner that can read line by line and preprocess files.
pub struct Scanner {
    buffers: Vec<Buffer>,
    preprocess: bool,
    included_files: HashSet<PathBuf>,
    // error: Option<ScanErr>,
}
impl Scanner {
    pub fn new<S: AsRef<Path>>(path: S) -> ScanResult<Self> {
        let mut scanner = Scanner {
            buffers: vec![Buffer::new(PathBuf::from(path.as_ref()))?],
            preprocess: false,
            included_files: HashSet::new(),
            // error: None,
        };
        scanner.included_files.insert(PathBuf::from(path.as_ref()));
        Ok(scanner)
    }
    // TODO: Consider removing ability to *not* preprocess.
    pub fn preprocess(mut self) -> Self {
        self.preprocess = true;
        self
    }

    fn read_line(&mut self) -> ScanResult<Option<LineType>> {
        let mut line = String::new();
        while let Some(buf) = self.buffers.last_mut() {
            if let 0 = buf.read_line(&mut line)? {
                self.buffers.pop();
                if let Some(buf) = self.buffers.last().map(|b| &b.path) {
                    return Ok(Some(LineType::change_file(buf)));
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
    fn handle_directive(&mut self, directive: Directive) -> ScanResult<Option<LineType>> {
        match directive {
            Directive::Include(path) => {
                self.push_file(path.clone())?;
                return Ok(Some(LineType::change_file(&path)));
            }
        }
    }
    fn push_file(&mut self, path: PathBuf) -> ScanResult<()> {
        if self.included_files.contains(&path) {
            let included_file = path.to_string_lossy().to_string();
            let current_buf = self.get_current_buf();
            let in_file = current_buf.path.to_string_lossy().to_string();
            let at_line = current_buf.lineno;
            return Err(ScanErr::IncludeCycle {
                included_file,
                in_file,
                at_line,
                previously_included_in: self.get_current_buf().included_from.clone(),
            });
        }
        self.included_files.insert(path.clone());

        self.buffers.push(Buffer::with_included_from(
            path,
            self.get_current_buf().get_filename_to_string(),
        )?);
        Ok(())
    }
    fn get_current_buf(&self) -> &Buffer {
        self.buffers
            .last()
            .expect("If scanner is still running, buffer stack should not be empty")
    }
}
impl Iterator for Scanner {
    type Item = LineType;
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
#[derive(Debug)]
pub enum LineType {
    ChangeFile(PathBuf),
    Line { text: Vec<u8>, lineno: usize },
}
impl LineType {
    pub fn line(text: Vec<u8>, lineno: usize) -> Self {
        LineType::Line { text, lineno }
    }
    pub fn change_file(path: &PathBuf) -> Self {
        LineType::ChangeFile(path.to_owned())
    }
}
impl fmt::Display for LineType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // LineType::ChangeFile(path) => write!(f, "{}", path.display()),
            LineType::Line { text, lineno: _ } => {
                write!(f, "{}", unsafe { std::str::from_utf8_unchecked(text) })
            }
            _ => Ok(()),
        }
    }
}
#[derive(Debug)]
struct Buffer {
    inner: BufReader<File>,
    path: PathBuf,
    lineno: usize,
    ows_empty: bool,
    included_from: Option<String>,
}
impl Buffer {
    fn new(path: PathBuf) -> ScanResult<Self> {
        Ok(Self {
            inner: BufReader::new(File::open(path.as_path())?),
            path,
            lineno: 0,
            ows_empty: false,
            included_from: None,
        })
    }
    fn with_included_from(path: PathBuf, included_from: String) -> ScanResult<Self> {
        let mut buf = Self::new(path)?;
        buf.included_from = Some(included_from);
        Ok(buf)
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
    fn finalize_line(&mut self, line: String) -> LineType {
        self.lineno += 1;
        LineType::line(line.into_bytes(), self.lineno)
    }
    fn get_filename_to_string(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}

#[derive(Debug)]
enum Directive {
    Include(PathBuf),
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
            Some(filename) if rest.peek().is_none() => Ok(Directive::Include(PathBuf::from(
                filename[1..filename.len() - 1].to_string(),
            ))),
            Some(_) => Err(ScanErr::IncludeTrailingArgs),
            None => Err(ScanErr::IncludeEmpty),
        },
        _ => Err(ScanErr::UnknownDirective(first.to_string())),
    }
}

type ScanResult<T> = Result<T, ScanErr>;
#[derive(Debug)]
pub enum ScanErr {
    IO(std::io::Error),
    UnknownDirective(String),

    IncludeTrailingArgs,
    IncludeEmpty,
    IncludeCycle {
        included_file: String,
        in_file: String,
        at_line: usize,
        previously_included_in: Option<String>,
    },
}
impl std::fmt::Display for ScanErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScanErr::IncludeCycle {
                included_file,
                in_file,
                at_line,
                previously_included_in,
            } => {
                let previously_included_in = if let Some(prev) = previously_included_in {
                    format!("(previously included in {})", prev)
                } else {
                    "(it's possibly also the source file given to the compiler)".to_string()
                };

                write!(
                    f,
                    "Include cycle detected: {} included again in {} at line {} {}",
                    included_file,
                    in_file,
                    at_line + 1,
                    previously_included_in
                )
            }
            ScanErr::IncludeEmpty => write!(f, "Include directive without filename"),
            ScanErr::IncludeTrailingArgs => write!(f, "Include directive with trailing arguments"),
            ScanErr::UnknownDirective(s) => write!(f, "Unknown directive: {}", s),
            ScanErr::IO(e) => write!(f, "IO error: {}", e),
        }
    }
}
impl From<std::io::Error> for ScanErr {
    fn from(e: std::io::Error) -> Self {
        ScanErr::IO(e)
    }
}
