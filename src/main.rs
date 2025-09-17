mod helpers;

use std::{io, process::Command};

use crate::helpers::{gen_and_push_out_name, sanitize_args, should_rebuild};

const COMPILER: &str = "g++";

fn main() -> io::Result<()> {
    let args = std::env::args().skip(1);
    let filename = std::env::args().nth(1).unwrap();

    let (mut compile_args, runtime_args) = sanitize_args(args);

    let out_file = gen_and_push_out_name(&filename, &mut compile_args);

    let force = compile_args.iter().any(|arg| arg == "--force");
    compile_args.retain(|arg| arg != "--force");

    let mut compile_command = Command::new(COMPILER);
    if force || should_rebuild(&compile_args[0], &out_file) {
        run(compile_command.args(compile_args))?;
    }

    let mut run_command = Command::new(&out_file);
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
