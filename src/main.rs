use std::{
    error::Error,
    io,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    result::Result,
};

use clap::Parser;
use once_cell::sync::Lazy;

// Parse command-line arguments
static OPTS: Lazy<Opt> = Lazy::new(Opt::parse);

fn main() -> Result<(), Box<dyn Error>> {
    let path = run_fuzzy()?;
    run_editor(&path)?;

    Ok(())
}

fn run_fuzzy() -> io::Result<PathBuf> {
    let mut cmd = Command::new(&OPTS.finder);
    if OPTS.finder == "fzf" {
        cmd.arg("--preview");
        cmd.arg("bat --color=always --style=numbers --line-range=:500 {}");
    }

    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());

    let out = cmd.output()?;

    match out.status.code() {
        Some(0) => Ok(Path::new(std::str::from_utf8(&out.stdout).unwrap().trim()).to_owned()),

        Some(x) => std::process::exit(x),
        None => std::process::exit(-1),
    }
}

fn run_editor(path: &Path) -> io::Result<()> {
    let mut cmd = Command::new(&OPTS.editor);

    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg(path);

    cmd.spawn()?.wait()?;

    Ok(())
}

#[derive(Debug, Parser)]
struct Opt {
    /// The editor to use
    #[clap(long, env = "EDITOR", default_value = "nvim")]
    editor: String,

    /// The fuzzy finder to use
    /// If this is `fzf`, then colour preview will be enabled.
    #[clap(long, env = "SEARCH_EDIT_FINDER", default_value = "fzf")]
    finder: String,
}
