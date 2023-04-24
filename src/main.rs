use llamac::cli;
use llamac::init_logger;
use llamac::run_compiler;
use llamac::CompilerError;
use log::error;
use log::info;
use std::process::ExitCode;

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
