use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::Peekable;
use std::path::PathBuf;
use std::str::SplitWhitespace;

pub struct Scanner {
    buffers: Vec<Buffer>,
    preprocess: bool,
}
impl Scanner {
    pub fn new() -> Self {
        Scanner {
            buffers: Vec::new(),
            preprocess: false,
        }
    }
    pub fn preprocess(mut self) -> Self {
        self.preprocess = true;
        self
    }
    pub fn add_file<S: AsRef<str>>(mut self, path: S) -> Result<Self, ScanErr> {
        self.push_file(PathBuf::from(path.as_ref()))?;
        Ok(self)
    }

    pub fn get_cur_lineno(&self) -> Option<usize> {
        self.buffers.last().map(|b| b.lineno)
    }
    pub fn get_cur_filename(&self) -> Option<&str> {
        self.buffers
            .last()
            .and_then(|b| b.path.as_path().as_os_str().to_str())
    }
    pub fn read_line(&mut self) -> Result<Option<String>, ScanErr> {
        let mut line = String::new();
        while let Some(buf) = self.buffers.last_mut() {
            if let 0 = buf.read_line(&mut line)? {
                self.buffers.pop();
                continue;
            }
            match (self.preprocess, line.to_directive()?) {
                (true, Some(directive)) => {
                    buf.set_ows_empty();
                    self.handle_directive(directive)?;
                }
                _ => return Ok(Some(buf.finalize_line(line))),
            }
            line.clear()
        }
        Ok(None)
    }

    fn handle_directive(&mut self, directive: Directive) -> Result<(), ScanErr> {
        match directive {
            Directive::Include(path) => self.push_file(path),
        }
    }
    fn push_file(&mut self, path: PathBuf) -> Result<(), ScanErr> {
        self.buffers.push(Buffer::new(path)?);
        Ok(())
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
    fn finalize_line(&mut self, line: String) -> String {
        self.lineno += 1;
        line
    }
}

#[derive(Debug)]
enum Directive {
    Include(PathBuf),
}

trait Line {
    fn to_directive(&self) -> Result<Option<Directive>, ScanErr>;
    fn decide_directive(
        &self,
        first: &str,
        rest: Peekable<SplitWhitespace>,
    ) -> Result<Directive, ScanErr>;
}
impl Line for String {
    fn to_directive(&self) -> Result<Option<Directive>, ScanErr> {
        let mut words = self.split_whitespace().peekable();
        match words.next() {
            Some(word) if !word.starts_with("#") => Ok(None),
            Some(directive) => Ok(Some(self.decide_directive(directive, words)?)),
            None => Ok(None),
        }
    }
    fn decide_directive(
        &self,
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
