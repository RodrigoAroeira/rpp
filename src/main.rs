use std::{io, process::Command};

fn main() -> io::Result<()> {
    let mut compile_args: Vec<String> = std::env::args().skip(1).collect();

    let invalid_args = ["-fsyntax-only", "-E", "-S", "-c"];

    for invalid_arg in invalid_args {
        // Remove from args
        compile_args.retain(|s| s != invalid_arg);
    }

    while let Some(o_index) = compile_args.iter().position(|arg| arg == "-o") {
        compile_args.remove(o_index);
        if o_index < compile_args.len() {
            compile_args.remove(o_index);
        }
    }

    let runtime_args: Vec<String> =
        if let Some(index) = compile_args.iter().position(|arg| arg == "--") {
            let vec = compile_args.split_off(index + 1);
            let _popped = compile_args.pop();
            #[cfg(debug_assertions)]
            {
                println!("runtime args: {vec:?}");
                println!("Old args: {compile_args:?}");
            }
            vec
        } else {
            vec![]
        };

    const OUT_FILE: &str = "/tmp/a.out";
    for command in ["-o", OUT_FILE, "-std=c++23"].map(String::from) {
        compile_args.push(command);
    }

    const COMPILER: &str = "g++";

    let mut compile_command = Command::new(COMPILER);
    run(compile_command.args(compile_args))?;

    let mut run_command = Command::new(OUT_FILE);
    run(run_command.args(runtime_args))?; // If fails may not delete out_file

    let mut remove_command = Command::new("rm");
    run(remove_command.arg(OUT_FILE))?;

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
