mod cli;
mod helpers;

use std::{io, path::Path, process::Command};

use clap::Parser;

use crate::{cli::Cli, helpers::gen_and_push_out_name};

const COMPILER: &str = "g++";

fn main() -> io::Result<()> {
    let mut args = Cli::parse();
    args.sanitize();

    let out_file = gen_and_push_out_name(&args.src_file, &mut args.compile_args);

    let should_build = args.force || !Path::new(&out_file).exists();

    if args.verbose && should_build {
        println!("[INFO] Building {} -> {}", args.src_file, &out_file);
    }

    if should_build {
        run(Command::new(COMPILER).args(args.compile_args))?;
    }

    if args.verbose && !should_build {
        eprintln!("[INFO] Rebuild not needed for {}", args.src_file);
    }

    run(Command::new(&out_file).args(args.runtime_args))?;

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
