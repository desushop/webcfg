use std::path::{Path, PathBuf};

use xshell::cmd;

#[cfg(target_os = "linux")]
const BIN: &'static str = "webcfg";
#[cfg(target_os = "windows")]
const BIN: &'static str = "webcfg.exe";

fn main() {
    let mut sh = xshell::Shell::new().unwrap();
    let task = std::env::args().nth(1);
    sh.set_current_dir(workspace_root());
    match task.as_deref() {
        Some("dependencies") => dependencies(&mut sh),
        Some("build") => run_build(&mut sh),
        Some("move") => run_move(&mut sh),
        Some("test") => run_test(&mut sh, std::env::args().nth(2).is_some()),
        Some("exec") => run_exec(&mut sh, ),
        _ => println!("commands:\n- dependencies\n- build\n- move\n- test\n- exec"),
    }
}

fn dependencies(sh: &mut xshell::Shell) {
}

fn run_build(sh: &mut xshell::Shell) {
    cmd!(sh, "cargo build --release --color always").run().unwrap();
}

fn run_move(sh: &mut xshell::Shell) {
    sh.copy_file_to_dir(
        sh.current_dir().join(format!(".output/release/{BIN}")),
        sh.current_dir().join(".build/")
    ).unwrap();
}

fn run_exec(sh: &mut xshell::Shell) {
    cmd!(sh, "./.build/{BIN}").run().unwrap()
}

fn run_test(sh: &mut xshell::Shell, verbose: bool) {
    match verbose {
        false => cmd!(sh, "cargo test -p webcfg -- --test-threads=1").run().unwrap(),
        true => cmd!(sh, "cargo nextest run -rv -j 1 --nff --workspace --color always --cargo-quiet --cargo-message-format human --show-progress bar").run().unwrap(),
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}