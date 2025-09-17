use clap::Parser;

const PREFERRED_VERSION: &str = "-std=c++23";
/// rpp: Rust PreProcessor
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Force rebuild even if output is up to date
    #[arg(short, long)]
    pub force: bool,

    /// Displays extra information while running
    #[arg(short, long)]
    pub verbose: bool,

    /// Source file to compile
    pub src_file: String,

    /// Extra compiler arguments (before `--`)
    #[arg(allow_hyphen_values = true, short, long)]
    pub compile_args: Vec<String>,

    /// Runtime arguments (after `--`)
    #[arg(short, long)]
    pub runtime_args: Vec<String>,
}

impl Cli {
    pub fn sanitize(&mut self) {
        for invalid_arg in ["-fsyntax-only", "-E", "-S", "-c"] {
            // Remove from args
            self.compile_args.retain(|s| s != invalid_arg);
        }
        while let Some(idx) = self
            .compile_args
            .iter()
            .position(|arg| arg.starts_with("-o"))
        {
            // Remove -o
            self.compile_args.remove(idx);

            // Remove arg following -o
            if idx < self.compile_args.len() {
                self.compile_args.remove(idx);
            }
        }

        if !self.compile_args.iter().any(|arg| arg.starts_with("-std=")) {
            self.compile_args.push(String::from(PREFERRED_VERSION));
        }

        self.compile_args.push(self.src_file.clone());
    }
}
