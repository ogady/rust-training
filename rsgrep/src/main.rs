// use std::env::args; <- stuctoptを使うと不要
use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rsgrep")]
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "PATH")]
    path: String,
}

// impl GrepArgs { // stuctoptを使うと不要
//     fn new(pattern: String, path: String) -> Self {
//         GrepArgs { pattern, path }
//     }
// }

fn run(state: GrepArgs) {
    // &で借用する
    match read_to_string(&state.path) {
        Ok(content) => grep(content, &state),
        Err(reason) => println!("{}", reason),
    }
}

fn grep(content: String, state: &GrepArgs) {
    for line in content.lines() {
        if line.contains(state.pattern.as_str()) {
            println!("{}", line)
        }
    }
}

fn main() {
    // let pattern = args().nth(1);
    // let path = args().nth(2);
    // match (path, pattern) {
    //     (Some(path), Some(pattern)) => run(GrepArgs::new(pattern, path)), // Result型
    //     _ => println!("No path is specified!"),
    // }

    // stuctoptを使うと↑が不要で、↓だけで済む
    run(GrepArgs::from_args())
}
