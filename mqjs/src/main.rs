use std::{env::args, process::ExitCode};

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::process::ExitCode {
    let exit_code = mqjs::realmain(args()).await;
    return ExitCode::from(exit_code);
}
