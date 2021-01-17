use std::{env, process};
use std::path::Path;
use dll_injector;

static RED: &str = "\x1b[91m";
static RESET: &str = "\x1b[0m";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("{}Usage: {} <dll> <pid>{}", RED, args[0], RESET);
        process::exit(0);
    }

    let dll_path = Path::new(&args[1]);
    let pid: u32 = match args[2].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("{}引数「PID」は正の整数でなければなりません{}", RED, RESET);
            process::exit(0);
        }
    };

    if ! dll_path.exists() {
        eprintln!("{}指定されたDLLパスは存在しません{}", RED, RESET);
        process::exit(0);
    }

    let canon_path = dll_path.canonicalize().unwrap();
    let dll_path = canon_path.into_os_string().into_string().unwrap();    

    if let Err(e) = dll_injector::inject_dll(pid, &dll_path) {
        eprintln!("{} の呼び出しがうまくいかず、実行が途中に止まりました", e);
        process::exit(1);
    }
}
