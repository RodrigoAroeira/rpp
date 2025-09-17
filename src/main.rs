mod cli;
mod helpers;

use std::{io, process::Command};

use clap::Parser;

use crate::{
    cli::Cli,
    helpers::{gen_and_push_out_name, should_rebuild},
};

const COMPILER: &str = "g++";

fn main() -> io::Result<()> {
    let mut args = Cli::parse();
    args.sanitize();

    let out_file = gen_and_push_out_name(&args.src_file, &mut args.compile_args);

    let mut compile_command = Command::new(COMPILER);
    let build = args.force || should_rebuild(&args.src_file, &out_file);

    if args.verbose && build {
        println!("[INFO] Building {} -> {}", args.src_file, &out_file);
    }

    if build {
        run(compile_command.args(args.compile_args))?;
    }

    if args.verbose && !build {
        eprintln!("[INFO] Rebuild not needed for {}", args.src_file);
    }
    let mut run_command = Command::new(&out_file);
    run(run_command.args(args.runtime_args))?;

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
