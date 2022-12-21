mod cli;
mod lex;
mod long_peekable;
mod scan;
mod writer_iter;

use cli::PrintWriterHelpers;
use cli::StopAfter;
use colored::Colorize;
use env_logger::Env;
use lex::token::Token;
use lex::IntoLexer;
use log::info;
use scan::Line;
use std::io::Write;
use writer_iter::WriterIter;

fn main() -> CompilerResult<()> {
    init_logger();
    let args = cli::Cli::parse();
    let res = run_compiler(&args);
    if let Err(CompilerError::EarlyExit(msg)) = res {
        info!("{msg}");
        return Ok(());
    }
    res
}
fn run_compiler(args: &cli::Cli) -> CompilerResult<()> {
    let scanner = make_scanner(args)?;
    // TODO: Implement lexer
    let _lexer = make_lexer(args, scanner)?;
    // TODO: Implement parser
    // TODO: Implement sem
    // TODO: Implement irgen
    // TODO: Implement codegen/binary-gen
    Ok(())
}
fn make_lexer(
    args: &cli::Cli,
    scanner: impl Iterator<Item = Line> + 'static,
) -> CompilerResult<Box<dyn Iterator<Item = Token>>> {
    scanner
        .into_lexer(true)
        .maybe_stop(
            args.print.get_token_writer()?,
            args.stop_after <= StopAfter::Lexing,
        )
        .ok_or(CompilerError::EarlyExit(
            "Stopping... (--stop-after lexing)",
        ))
}
fn make_scanner(args: &cli::Cli) -> CompilerResult<Box<dyn Iterator<Item = Line>>> {
    // let mut err = Ok(());
    scan::Scanner::new(&args.filename)?
        .preprocess()
        .maybe_stop(
            args.print.get_preprocessor_writer()?,
            args.stop_after == StopAfter::Preprocessing,
        )
        .ok_or(CompilerError::EarlyExit(
            "Stopping... (--stop-after preprocessing)",
        ))
}

trait MaybeStop<Item: std::fmt::Display + 'static>: WriterIter<Item> + 'static {
    fn maybe_stop(
        self,
        writer: Option<impl Write + 'static>,
        stop: bool,
    ) -> Option<Box<dyn Iterator<Item = Self::Item>>> {
        let iter: Box<dyn Iterator<Item = Self::Item>> = match writer {
            Some(writer) => Box::new(self.writer_iter(writer)),
            None => Box::new(self),
        };
        if !stop {
            Some(iter)
        } else {
            iter.for_each(drop);
            None
        }
    }
}
impl<Item: std::fmt::Display + 'static, I: Iterator<Item = Item> + 'static> MaybeStop<Item> for I {}

fn init_logger() {
    // let log_level = if args.verbose { "info" } else { "warn" };
    env_logger::Builder::from_env(Env::default().default_filter_or("warn"))
        .format(|f, record| {
            let level = match record.level() {
                log::Level::Error => "error".red(),
                log::Level::Warn => "warning".yellow(),
                log::Level::Info => "info".green(),
                log::Level::Debug => "debug".blue(),
                log::Level::Trace => "trace".purple(),
            }
            .bold();
            writeln!(f, "{}: {}", level, record.args())
        })
        .init();
}

type CompilerResult<T> = Result<T, CompilerError>;
#[derive(Debug)]
enum CompilerError {
    EarlyExit(&'static str),

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
