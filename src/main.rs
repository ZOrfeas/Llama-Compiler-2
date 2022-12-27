mod cli;
mod lex;
mod long_peekable;
mod parse;
mod scan;
mod writer_iter;

use cli::PrintWriterHelpers;
use cli::StopAfter;
use colored::Colorize;
use env_logger::Env;
use lex::IntoLexer;
use log::info;
use parse::IntoParser;
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
    // let scanner = make_scanner(args)?;
    // let _lexer = make_lexer(args, scanner)?;
    let ast = scan::Scanner::new(&args.filename)?
        .preprocess()
        .make_step(
            args,
            args.print.get_preprocessor_writer()?,
            StopAfter::Preprocessing,
            "Stopping... (--stop-after preprocessing)",
        )?
        .into_lexer(true)
        .make_step(
            args,
            args.print.get_token_writer()?,
            StopAfter::Lexing,
            "Stopping... (--stop-after lexing)",
        )?
        .into_parser()
        .program()?;
    println!("{:#?}", ast);
    // TODO: Implement parser
    // TODO: Implement sem
    // TODO: Implement irgen
    // TODO: Implement codegen/binary-gen
    Ok(())
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
    fn make_step(
        self,
        args: &cli::Cli,
        writer: Option<Box<dyn Write>>,
        step: StopAfter,
        msg: &'static str,
    ) -> CompilerResult<Box<dyn Iterator<Item = Self::Item>>> {
        self.maybe_stop(writer, args.stop_after == step)
            .ok_or(CompilerError::EarlyExit(msg))
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

    ParserError(parse::ParseErr),
    ScannerError(scan::ScanErr),
    CliError(cli::CliErr),
}
impl From<parse::ParseErr> for CompilerError {
    fn from(err: parse::ParseErr) -> Self {
        CompilerError::ParserError(err)
    }
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
