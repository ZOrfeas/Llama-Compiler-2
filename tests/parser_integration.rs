use std::path::Path;

use llamac::cli::{Cli, StopAfter};

fn make_args_struct(input_filename: String) -> Cli {
    Cli {
        filename: input_filename,
        stop_after: StopAfter::Parsing,
        out: "".to_string(),
        verbose: false,
        print: None,
    }
}
fn parse_fully(path: &Path) -> datatest_stable::Result<()> {
    let path = path.to_str().unwrap().to_string();
    if let Err(err) = llamac::run_compiler(&make_args_struct(path)) {
        if !matches!(err, llamac::CompilerError::EarlyExit(_)) {
            return Err(Box::new(err) as _);
        }
    }
    Ok(())
}
datatest_stable::harness!(parse_fully, "./testfiles/syntax", r".*\.lla");
