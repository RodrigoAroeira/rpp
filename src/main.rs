mod helpers;

use std::{io, process::Command};

use crate::helpers::{sanitize_args, should_rebuild};

const PREFERRED_VERSION: &str = "-std=c++23";
const OUT_FILE: &str = "/tmp/rpp.out";
const COMPILER: &str = "g++";

fn main() -> io::Result<()> {
    let (mut compile_args, runtime_args) = sanitize_args(std::env::args().skip(1));

    let force = compile_args.iter().any(|arg| arg == "--force");
    compile_args.retain(|arg| arg != "--force");

    let mut compile_command = Command::new(COMPILER);
    if force || should_rebuild(&compile_args[0], OUT_FILE) {
        run(compile_command.args(compile_args))?;
    }

    let mut run_command = Command::new(OUT_FILE);
    run(run_command.args(runtime_args))?;

    Ok(())
}

fn run(command: &mut Command) -> io::Result<()> {
    if let Some(status_code) = command.spawn()?.wait()?.code()
        && status_code != 0
    {
        std::process::exit(status_code);
    };
    Ok(())
}
