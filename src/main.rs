use cli::PrintWriterHelpers;
use std::io::Write;

mod cli;
mod lex;
mod longpeekable;
mod scan;

fn main() -> Result<(), CompilerError> {
    handle_args(cli::Cli::parse())
}

fn handle_args(args: cli::Cli) -> Result<(), CompilerError> {
    let _ = scan::Scanner::new(&args.filename)?
        .preprocess()
        .maybe_add_writer(args.print.get_preprocessor_writer()?)
        .handle_stopping(args.stop_after == cli::StopAfter::Preprocessing)
        .is_some();

    Ok(())
}
struct MaybeWriter<I: Iterator + 'static>(Box<dyn Iterator<Item = I::Item>>, bool);
impl<I: Iterator + 'static> MaybeWriter<I> {
    fn handle_stopping(self, stop: bool) -> Option<Box<dyn Iterator<Item = I::Item>>> {
        if stop {
            if self.1 {
                self.0.for_each(drop);
            }
            None
        } else {
            Some(self.0)
        }
    }
}
trait MaybeAddWriterExt<I: Iterator + 'static>
where
    I::Item: std::fmt::Display,
{
    fn maybe_add_writer(self, writer: Option<impl Write + 'static>) -> MaybeWriter<I>;
}
impl<I: Iterator + 'static> MaybeAddWriterExt<I> for I
where
    I::Item: std::fmt::Display,
{
    fn maybe_add_writer(self, writer: Option<impl Write + 'static>) -> MaybeWriter<I> {
        match writer {
            Some(mut writer) => MaybeWriter(
                Box::new(self.map(move |item| {
                    let _ = write!(writer, "{}", item);
                    item
                })),
                true,
            ),
            None => MaybeWriter(Box::new(self), false),
        }
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
