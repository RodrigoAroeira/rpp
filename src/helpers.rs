use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io,
};

pub fn gen_and_push_out_name(
    src_filename: &str,
    compile_args: &mut Vec<String>,
) -> io::Result<String> {
    let mut hasher = DefaultHasher::new();
    src_filename.hash(&mut hasher);

    let hash_lines = |filename: &str, hasher: &mut DefaultHasher| -> std::io::Result<()> {
        log::info!("Hashing '{filename}'");
        let s = std::fs::read_to_string(filename)?;
        s.hash(hasher);

        Ok(())
    };

    hash_lines(src_filename, &mut hasher)?;
    for arg in compile_args.iter() {
        for ext in [".cpp", ".h", ".hpp"] {
            if arg.contains(ext) {
                hash_lines(arg, &mut hasher)?;
                continue;
            }
        }
        arg.hash(&mut hasher);
    }

    let hash = hasher.finish();
    let name = format!("/tmp/rpp-{hash}.out");

    for arg in ["-o", &name].map(String::from) {
        compile_args.push(arg);
    }
    Ok(name)
}
