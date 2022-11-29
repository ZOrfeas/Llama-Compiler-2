use cli::PrintWriterHelpers;
use scan::LineType;
use std::io::{BufWriter, Write};

mod cli;
mod lex;
mod longpeekable;
mod scan;

fn main() -> Result<(), CompilerError> {
    handle_args(cli::Cli::parse())?;
    Ok(())
}

fn handle_args(args: cli::Cli) -> Result<(), CompilerError> {
    let (text_iter, print_preprocessed) = args.handle_preprocessor()?;
    match (args.stop_after, print_preprocessed) {
        (cli::StopAfter::Preprocessing, true) => return Ok(text_iter.for_each(drop)),
        (cli::StopAfter::Preprocessing, false) => return Ok(()),
        _ => (),
    }

    Ok(())
}
trait CliHelpers {
    fn handle_preprocessor(
        &self,
    ) -> Result<(Box<dyn Iterator<Item = LineType>>, bool), CompilerError>;
}
impl CliHelpers for cli::Cli {
    fn handle_preprocessor(
        &self,
    ) -> Result<(Box<dyn Iterator<Item = LineType>>, bool), CompilerError> {
        type ReturnType = Result<(Box<dyn Iterator<Item = LineType>>, bool), CompilerError>;
        let make_scanner =
            || -> Result<_, CompilerError> { Ok(scan::Scanner::new(&self.filename)?.preprocess()) };
        let make_writer = |mut writer: BufWriter<Box<dyn Write>>| {
            move |line: LineType| {
                let _ = write!(writer, "{}", line);
                line
            }
        };
        self.print.get_preprocessor_writer()?.map_or_else(
            || -> ReturnType { Ok((Box::new(make_scanner()?), false)) },
            |writer| Ok(((Box::new(make_scanner()?.map(make_writer(writer)))), true)),
        )
    }
}

#[derive(Debug)]
enum CompilerError {
    ScannerError(scan::ScanErr),
    CliError(cli::CliErr),
}
impl From<scan::ScanErr> for CompilerError {
    fn from(err: scan::ScanErr) -> Self {
        CompilerError::ScannerError(err)
    }
}
impl From<cli::CliErr> for CompilerError {
    fn from(err: cli::CliErr) -> Self {
        CompilerError::CliError(err)
    }
}
