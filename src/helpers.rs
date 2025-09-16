use std::time::SystemTime;

use crate::{OUT_FILE, PREFERRED_VERSION};

pub fn sanitize_args(args: impl Iterator<Item = String>) -> (Vec<String>, Vec<String>) {
    let mut compile_args: Vec<String> = args.collect();
    for invalid_arg in ["-fsyntax-only", "-E", "-S", "-c"] {
        // Remove from args
        compile_args.retain(|s| s != invalid_arg);
    }

    while let Some(o_index) = compile_args.iter().position(|arg| arg.starts_with("-o")) {
        // Remove -o
        compile_args.remove(o_index);

        // Remove arg following -o
        if o_index < compile_args.len() {
            compile_args.remove(o_index);
        }
    }

    let runtime_args: Vec<String> =
        if let Some(index) = compile_args.iter().position(|arg| arg == "--") {
            let vec = compile_args.split_off(index + 1);
            let _popped = compile_args.pop();

            vec
        } else {
            vec![]
        };

    for command in ["-o", OUT_FILE].map(String::from) {
        compile_args.push(command);
    }

    if compile_args.iter().all(|arg| !arg.starts_with("-std=")) {
        compile_args.push(String::from(PREFERRED_VERSION));
    }

    (compile_args, runtime_args)
}

pub fn should_rebuild(curr_file: &str, out_file: &str) -> bool {
    let Ok(curr_metadata) = std::fs::metadata(curr_file) else {
        return true;
    };
    let Ok(out_metadata) = std::fs::metadata(out_file) else {
        return true;
    };

    let curr_time = curr_metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let out_time = out_metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);

    if curr_time > out_time {
        return true;
    }

    false
}

