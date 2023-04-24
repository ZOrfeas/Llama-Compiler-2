use std::path::Path;

use llamac::{
    cli::{Cli, StopAfter},
    pass::sem::SemanticError,
};

fn make_args_struct(input_filename: String) -> Cli {
    Cli {
        filename: input_filename,
        stop_after: StopAfter::Sem,
        out: "".to_string(),
        verbose: false,
        print: None,
    }
}

fn sem_fully(path: &Path) -> datatest_stable::Result<()> {
    let path_str = path.to_str().unwrap().to_string();
    let compilation_result = llamac::run_compiler(&make_args_struct(path_str));
    match compilation_result {
        Ok(_) | Err(llamac::CompilerError::EarlyExit(_)) => Ok(()),
        Err(llamac::CompilerError::SemanticError(SemanticError::InferenceError {
            msg, ..
        })) if msg.contains("Occurs")
            && path
                .as_os_str()
                .to_string_lossy()
                .as_ref()
                .contains("occurs") =>
        {
            Ok(())
        }
        Err(err) => Err(Box::new(err) as _),
    }
}

datatest_stable::harness!(sem_fully, "./testfiles/end-to-end", r".*\.lla");
