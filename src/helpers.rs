use std::{
    fs,
    hash::{DefaultHasher, Hash, Hasher},
    time::UNIX_EPOCH,
};

pub fn gen_and_push_out_name(src_filename: &str, compile_args: &mut Vec<String>) -> String {
    let mut hasher = DefaultHasher::new();
    src_filename.hash(&mut hasher);

    let ntime = fs::metadata(src_filename)
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
