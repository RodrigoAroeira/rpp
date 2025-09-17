use std::{
    hash::{DefaultHasher, Hasher},
    path::Path,
    time::SystemTime,
};

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
    if !Path::new(out_file).exists() {
        return true;
    }

    let curr_time = std::fs::metadata(curr_file)
        .and_then(|m| m.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH);

    let out_time = std::fs::metadata(out_file)
        .and_then(|m| m.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH);

    curr_time > out_time
}
