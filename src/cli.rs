use std::{fs::File, io::BufWriter, io::Write, path::PathBuf};

use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    /// Path to source file
    pub filename: PathBuf,

    /// Run and stop after specified step
    #[arg(long, short, value_enum, value_name = "step", default_value_t = StopAfter::IrGen)]
    pub stop_after: StopAfter,

    /// Specify binary output filename
    #[arg(long, short, value_name = "file", default_value = "a.out")]
    pub out: String,

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
    preprocessed: Option<Option<PathBuf>>,

    #[arg(long, value_name = "file")]
    tokens: Option<Option<PathBuf>>,

    #[arg(long, value_name = "file")]
    ast: Option<Option<PathBuf>>,

    #[arg(long, value_name = "file")]
    types: Option<Option<PathBuf>>,

    #[arg(long, value_name = "file")]
    ir: Option<Option<PathBuf>>,

    #[arg(long, value_name = "file")]
    asm: Option<Option<PathBuf>>,
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
        if !self.filename.exists() {
            eprintln!("File not found: {}", self.filename.display());
            std::process::exit(1);
        }
        if self.stop_after != StopAfter::IrGen && self.out != "a.out" {
            eprintln!("Warning: no executable produced when --stop-after is not 'ir-gen'");
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
                eprintln!("Warning: will stop before producing tokens, print --tokens ignored");
            }
            if self.stop_after < StopAfter::Parsing && ast.is_some() {
                eprintln!("Warning: will stop before producing AST, print --ast ignored");
            }
            if self.stop_after < StopAfter::Sem && types.is_some() {
                eprintln!("Warning: will stop before semantic analysis, print --types ignored");
            }
            if self.stop_after < StopAfter::IrGen && ir.is_some() {
                eprintln!("Warning: will stop before producing IR, print --ir ignored");
            }
            if self.stop_after < StopAfter::IrGen && asm.is_some() {
                eprintln!("Warning: will stop before producing IR, print --asm ignored");
            }
        }
        self
    }
}

pub trait PrintWriterHelpers {
    fn get_preprocessor_writer(&self) -> Result<Option<BufWriter<Box<dyn Write>>>, CliErr>;
    fn get_token_writer(&self) -> Result<Option<BufWriter<Box<dyn Write>>>, CliErr>;
    fn get_ast_writer(&self) -> Result<Option<BufWriter<Box<dyn Write>>>, CliErr>;
    fn get_types_writer(&self) -> Result<Option<BufWriter<Box<dyn Write>>>, CliErr>;
    fn get_ir_writer(&self) -> Result<Option<BufWriter<Box<dyn Write>>>, CliErr>;
    fn get_asm_writer(&self) -> Result<Option<BufWriter<Box<dyn Write>>>, CliErr>;
}
impl PrintWriterHelpers for Option<Printer> {
    fn get_preprocessor_writer(&self) -> Result<Option<BufWriter<Box<dyn Write>>>, CliErr> {
        Ok(self
            .as_ref()
            .map(|p| PrintCalls::out_target_helper(&p.to_print_calls().preprocessed))
            .transpose()?
            .flatten())
    }
    fn get_token_writer(&self) -> Result<Option<BufWriter<Box<dyn Write>>>, CliErr> {
        Ok(self
            .as_ref()
            .map(|p| PrintCalls::out_target_helper(&p.to_print_calls().tokens))
            .transpose()?
            .flatten())
    }
    fn get_ast_writer(&self) -> Result<Option<BufWriter<Box<dyn Write>>>, CliErr> {
        Ok(self
            .as_ref()
            .map(|p| PrintCalls::out_target_helper(&p.to_print_calls().ast))
            .transpose()?
            .flatten())
    }
    fn get_types_writer(&self) -> Result<Option<BufWriter<Box<dyn Write>>>, CliErr> {
        Ok(self
            .as_ref()
            .map(|p| PrintCalls::out_target_helper(&p.to_print_calls().types))
            .transpose()?
            .flatten())
    }
    fn get_ir_writer(&self) -> Result<Option<BufWriter<Box<dyn Write>>>, CliErr> {
        Ok(self
            .as_ref()
            .map(|p| PrintCalls::out_target_helper(&p.to_print_calls().ir))
            .transpose()?
            .flatten())
    }
    fn get_asm_writer(&self) -> Result<Option<BufWriter<Box<dyn Write>>>, CliErr> {
        Ok(self
            .as_ref()
            .map(|p| PrintCalls::out_target_helper(&p.to_print_calls().asm))
            .transpose()?
            .flatten())
    }
}
impl PrintCalls {
    fn out_target_helper(
        p: &Option<Option<PathBuf>>,
    ) -> Result<Option<BufWriter<Box<dyn Write>>>, CliErr> {
        Ok(match p {
            None => None,
            Some(None) => Some(BufWriter::new(Box::new(std::io::stdout()))),
            Some(Some(path)) => Some(BufWriter::new(Box::new(File::create(&path)?))),
        })
    }
}

#[derive(Debug)]
pub enum CliErr {
    IO(std::io::Error),
}
impl From<std::io::Error> for CliErr {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}
