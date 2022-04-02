use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt)] // => 「StructOptとレイトを継承する」という意味
#[structopt(name = "rsgrep")]
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: Vec<String>,
}

fn grep(content: String, pattern: &String) {
    for line in content.lines() {
        if line.contains(pattern.as_str()) {
            println!("{}", line)
        }
    }
}

fn run(state: GrepArgs) {
    // 実はforループは1回ごとに確保したリソースを解放する => 実質的にmoveが発生する
    for path in state.path.iter() {
        match read_to_string(path) {
            Ok(content) => grep(content, &state.pattern),
            Err(reason) => println!("{}", reason),
        }
    }
}

fn main() {
    run(GrepArgs::from_args());
}
