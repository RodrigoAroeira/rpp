use std::{
    hash::{DefaultHasher, Hasher},
    time::SystemTime,
};

const PREFERRED_VERSION: &str = "-std=c++23";

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

    // for command in ["-o", OUT_FILE].map(String::from) {
    //     compile_args.push(command);
    // }

    if compile_args.iter().all(|arg| !arg.starts_with("-std=")) {
        compile_args.push(String::from(PREFERRED_VERSION));
    }

    (compile_args, runtime_args)
}

pub fn gen_and_push_out_name(src_filename: &str, compile_args: &mut Vec<String>) -> String {
    use std::hash::Hash;
    use std::time::UNIX_EPOCH;

    let mut hasher = DefaultHasher::new();
    src_filename.hash(&mut hasher);

    let ntime = std::fs::metadata(src_filename)
        .and_then(|m| m.modified())
        .unwrap_or(UNIX_EPOCH);
    let secs = ntime
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    secs.hash(&mut hasher);

    for cmd in compile_args.iter() {
        hasher.write(cmd.as_bytes());
    }

    let hash = hasher.finish();
    let name = format!("/tmp/rpp-{hash}.out");

    for arg in [String::from("-o"), name.clone()] {
        compile_args.push(arg);
    }
    name
}

pub fn should_rebuild(curr_file: &str, out_file: &str) -> bool {
    let curr_time = std::fs::metadata(curr_file)
        .and_then(|m| m.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH);

    let out_time = std::fs::metadata(out_file)
        .and_then(|m| m.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH);

    let needs_rebuild = curr_time > out_time;

    if needs_rebuild && std::path::Path::new(out_file).exists() {
        eprintln!("[INFO] Rebuilding");
    }

    needs_rebuild
}
