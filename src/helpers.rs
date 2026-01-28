use std::{
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::{BufRead, BufReader},
};

pub fn gen_and_push_out_name(
    src_filename: &str,
    compile_args: &mut Vec<String>,
    verbose: bool,
) -> String {
    let mut hasher = DefaultHasher::new();
    src_filename.hash(&mut hasher);

    let hash_lines = |filename: &str, hasher: &mut DefaultHasher| {
        let file = File::open(filename).expect("Unable to open file");
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            if verbose {
                eprintln!("[INFO] Hashing: {line:?}");
            }
            line.hash(hasher);
        }
    };

    hash_lines(src_filename, &mut hasher);
    for arg in compile_args.iter() {
        for ext in [".cpp", ".h", ".hpp"] {
            if arg.contains(ext) {
                hash_lines(arg, &mut hasher);
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
    name
}
