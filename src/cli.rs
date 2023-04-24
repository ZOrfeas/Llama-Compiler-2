use std::{fs::File, io::BufWriter, io::Write, path::Path};

use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};
use colored::Colorize;
use log::{error, warn};
use thiserror::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to source file
    pub filename: String,

    /// Run and stop after specified step
    #[arg(long, short, value_enum, value_name = "step", default_value_t = StopAfter::IrGen)]
    pub stop_after: StopAfter,

    /// Specify binary output filename
    #[arg(long, short, value_name = "file", default_value = "a.out")]
    pub out: String,

    // TODO: Use this, or remove it.
    #[arg(long, short, default_value_t = false)]
    pub verbose: bool,

    #[command(subcommand)]
    pub print: Option<Printer>,
}

#[derive(Subcommand)]
pub enum Printer {
    /// Use to request intermediate output from the compilation process
    Print(PrintCalls),
}

#[derive(Args)]
#[command(group(
    ArgGroup::new("print")
        .args(["preprocessed", "tokens", "ast", "types", "ir", "asm"])
        .required(true).multiple(true),
))]
pub struct PrintCalls {
    #[arg(long, value_name = "file")]
    preprocessed: Option<Option<String>>,

    #[arg(long, value_name = "file")]
    tokens: Option<Option<String>>,

    #[arg(long, value_name = "file")]
    ast: Option<Option<String>>,

    #[arg(long, value_name = "file")]
    types: Option<Option<String>>,

    #[arg(long, value_name = "file")]
    ir: Option<Option<String>>,

    #[arg(long, value_name = "file")]
    asm: Option<Option<String>>,
}
impl Printer {
    pub fn to_print_calls(&self) -> &PrintCalls {
        match self {
            Printer::Print(calls) => calls,
        }
    }
}

#[derive(PartialEq, PartialOrd, ValueEnum, Clone, Copy)]
pub enum StopAfter {
    Preprocessing,
    Lexing,
    Parsing,
    Sem,
    IrGen,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse().validate()
    }
    /// Warns on some errors, exits on unrecoverable ones.
    fn validate(self) -> Self {
        if !Path::new(&self.filename).exists() {
            let filename_str = self.filename.underline();
            error!("File {} not found", filename_str);
            std::process::exit(1);
        }
        if self.stop_after != StopAfter::IrGen && self.out != "a.out" {
            warn!("no executable produced when --stop-after is not 'ir-gen'");
        }
        if let Some(Printer::Print(PrintCalls {
            preprocessed: _,
            tokens,
            ast,
            types,
            ir,
            asm,
        })) = &self.print
        {
            // if self.stop_after < StopAfter::Preprocessing && preprocessed.is_some() {
            //     eprintln!("Warning: will stop before preprocessing, 'print --preprocessed' ignored");
            // }
            if self.stop_after < StopAfter::Lexing && tokens.is_some() {
                warn!("Warning: will stop before producing tokens, print --tokens ignored");
            }
            if self.stop_after < StopAfter::Parsing && ast.is_some() {
                warn!("Warning: will stop before producing AST, print --ast ignored");
            }
            if self.stop_after < StopAfter::Sem && types.is_some() {
                warn!("Warning: will stop before semantic analysis, print --types ignored");
            }
            if self.stop_after < StopAfter::IrGen && ir.is_some() {
                warn!("Warning: will stop before producing IR, print --ir ignored");
            }
            if self.stop_after < StopAfter::IrGen && asm.is_some() {
                warn!("Warning: will stop before producing IR, print --asm ignored");
            }
        }
        self
    }
}

pub trait PrintWriterHelpers {
    fn get_preprocessor_writer(&self) -> CliResult<Option<Box<dyn Write>>>;
    fn get_token_writer(&self) -> CliResult<Option<Box<dyn Write>>>;
    fn get_ast_writer(&self) -> CliResult<Option<Box<dyn Write>>>;
    fn get_types_writer(&self) -> CliResult<Option<Box<dyn Write>>>;
    fn get_ir_writer(&self) -> CliResult<Option<Box<dyn Write>>>;
    fn get_asm_writer(&self) -> CliResult<Option<Box<dyn Write>>>;
    fn get_any_writer(
        &self,
        mapper: impl FnOnce(&Printer) -> CliResult<Option<Box<dyn Write>>>,
    ) -> CliResult<Option<Box<dyn Write>>>;
}
impl PrintWriterHelpers for Option<Printer> {
    fn get_preprocessor_writer(&self) -> CliResult<Option<Box<dyn Write>>> {
        self.get_any_writer(|p| PrintCalls::out_target_helper(&p.to_print_calls().preprocessed))
    }
    fn get_token_writer(&self) -> CliResult<Option<Box<dyn Write>>> {
        self.get_any_writer(|p| PrintCalls::out_target_helper(&p.to_print_calls().tokens))
    }
    fn get_ast_writer(&self) -> CliResult<Option<Box<dyn Write>>> {
        self.get_any_writer(|p| PrintCalls::out_target_helper(&p.to_print_calls().ast))
    }
    fn get_types_writer(&self) -> CliResult<Option<Box<dyn Write>>> {
        self.get_any_writer(|p| PrintCalls::out_target_helper(&p.to_print_calls().types))
    }
    fn get_ir_writer(&self) -> CliResult<Option<Box<dyn Write>>> {
        self.get_any_writer(|p| PrintCalls::out_target_helper(&p.to_print_calls().ir))
    }
    fn get_asm_writer(&self) -> CliResult<Option<Box<dyn Write>>> {
        self.get_any_writer(|p| PrintCalls::out_target_helper(&p.to_print_calls().asm))
    }
    fn get_any_writer(
        &self,
        mapper: impl FnOnce(&Printer) -> CliResult<Option<Box<dyn Write>>>,
    ) -> CliResult<Option<Box<dyn Write>>> {
        Ok(self.as_ref().map(mapper).transpose()?.flatten())
    }
}
impl PrintCalls {
    fn out_target_helper(p: &Option<Option<String>>) -> CliResult<Option<Box<dyn Write>>> {
        Ok(match p {
            None => None,
            Some(None) => Some(Box::new(BufWriter::new(std::io::stdout()))),
            Some(Some(path)) => Some(Box::new(BufWriter::new(File::create(&path)?))),
        })
    }
}

type CliResult<T> = Result<T, CliErr>;
#[derive(Error, Debug)]
pub enum CliErr {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}
