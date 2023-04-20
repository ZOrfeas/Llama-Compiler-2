mod cli;
mod lex;
mod long_peekable;
pub mod parse;
mod pass;
mod scan;
mod writer_iter;

use cli::PrintWriterHelpers;
use cli::StopAfter;
use colored::Colorize;
use env_logger::Env;
use lex::IntoLexer;
use log::error;
use log::info;
use parse::IntoParser;
use pass::sem::sem;
use std::io::Write;
use std::process::ExitCode;
use thiserror::Error;
use writer_iter::WriterIter;

fn main() -> ExitCode {
    init_logger();
    let args = cli::Cli::parse();
    let res = run_compiler(&args);
    match res {
        Ok(_) => ExitCode::SUCCESS,
        Err(CompilerError::EarlyExit(msg)) => {
            info!("{}", msg);
            ExitCode::SUCCESS
        }
        Err(err) => {
            error!("{}", err);
            ExitCode::FAILURE
        }
    }
}
fn run_compiler(args: &cli::Cli) -> CompilerResult<()> {
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
    args.print
        .get_ast_writer()?
        .map(|w| ast.print(w).expect("Failed to print AST"));
    // println!("{:#?}", ast);
    if args.stop_after == StopAfter::Parsing {
        return Err(CompilerError::EarlyExit(
            "Stopping... (--stop-after parsing)",
        ));
    }
    // TODO: Implement sem
    let mut _sem_results = sem(&ast)?;
    args.print.get_types_writer()?.map(|w| {
        _sem_results
            .types
            .print_node_types(w)
            .expect("Failed to print types")
    });
    if args.stop_after == StopAfter::Sem {
        return Err(CompilerError::EarlyExit("Stopping... (--stop-after sem)"));
    }
    // _sem_results.types.print_node_types();
    // println!("{:?}", _sem_results.types);
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
#[derive(Error, Debug)]
#[error("{0}")]
enum CompilerError {
    EarlyExit(&'static str),

    SemanticError(#[from] pass::sem::SemanticError),
    ParserError(#[from] parse::ParseErr),
    ScannerError(#[from] scan::ScanErr),
    CliError(#[from] cli::CliErr),
}
