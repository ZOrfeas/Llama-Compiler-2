use core::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::Peekable;
use std::path::{Path, PathBuf};
use std::str::SplitWhitespace;
// !Consider Scanners that can read from stdin.

/// Simple file scanner that can read line by line and preprocess files.
pub struct Scanner {
    buffers: Vec<Buffer>,
    preprocess: bool,
}
impl Scanner {
    pub fn new<S: AsRef<Path>>(path: S) -> Result<Self, ScanErr> {
        Ok(Scanner {
            buffers: vec![Buffer::new(PathBuf::from(path.as_ref()))?],
            preprocess: false,
        })
    }
    pub fn preprocess(mut self) -> Self {
        self.preprocess = true;
        self
    }

    fn read_line(&mut self) -> Result<Option<LineType>, ScanErr> {
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

    fn handle_directive(&mut self, directive: Directive) -> Result<Option<LineType>, ScanErr> {
        match directive {
            Directive::Include(path) => {
                self.push_file(path.clone())?;
                return Ok(Some(LineType::change_file(&path)));
            }
        }
    }
    fn push_file(&mut self, path: PathBuf) -> Result<(), ScanErr> {
        self.buffers.push(Buffer::new(path)?);
        Ok(())
    }
}
impl Iterator for Scanner {
    type Item = LineType;
    fn next(&mut self) -> Option<Self::Item> {
        self.read_line().unwrap()
    }
}
#[derive(Debug)]
pub enum LineType {
    ChangeFile(PathBuf),
    Line { text: String, lineno: usize },
}
impl LineType {
    pub fn line(text: String, lineno: usize) -> Self {
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
            LineType::Line { text, lineno: _ } => write!(f, "{}", text),
            _ => Ok(()),
        }
    }
}
struct Buffer {
    inner: BufReader<File>,
    path: PathBuf,
    lineno: usize,
    ows_empty: bool,
}
impl Buffer {
    fn new(path: PathBuf) -> Result<Self, ScanErr> {
        Ok(Self {
            inner: BufReader::new(File::open(path.as_path())?),
            path,
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
    fn finalize_line(&mut self, line: String) -> LineType {
        self.lineno += 1;
        LineType::line(line, self.lineno)
    }
}

#[derive(Debug)]
enum Directive {
    Include(PathBuf),
}

fn to_directive(s: &String) -> Result<Option<Directive>, ScanErr> {
    let mut words = s.split_whitespace().peekable();
    match words.next() {
        Some(word) if !word.starts_with("#") => Ok(None),
        Some(directive) => Ok(Some(decide_directive(directive, words)?)),
        None => Ok(None),
    }
}
fn decide_directive(
    first: &str,
    mut rest: Peekable<SplitWhitespace>,
) -> Result<Directive, ScanErr> {
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

#[derive(Debug)]
pub enum ScanErr {
    IO(std::io::Error),
    UnknownDirective(String),

    IncludeTrailingArgs,
    IncludeEmpty,
}
impl From<std::io::Error> for ScanErr {
    fn from(e: std::io::Error) -> Self {
        ScanErr::IO(e)
    }
}
