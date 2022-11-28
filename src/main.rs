use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Debug)]
enum ProgError {
    NoFile,
    Io(Error),
}

impl From<Error> for ProgError {
    fn from(err: Error) -> ProgError {
        ProgError::Io(err)
    }
}

const METADATA_REPO: &str = "/Users/chris.mcdevitt/code/scratch/nix-shells/";

fn map_file(file: &OsStr) -> &OsStr {
    match file.to_str().unwrap() {
        "envrc" => OsStr::new(".envrc"),
        _other => file,
    }
}

fn confirm(text: &str) -> bool {
    println!("{:?}? [y/N]", text);
    let mut confirm = String::new();
    io::stdin()
        .read_line(&mut confirm)
        .expect("Failed to read input");
    println!("Got {}", confirm);
    confirm == "y\n"
}

fn filter_dir(path: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let names = HashSet::from(["envrc", "shell.nix", "justfile", "Makefile"]);
    Ok(fs::read_dir(path)?
        .into_iter()
        .filter_map(Result::ok)
        .map(|r| r.path())
        .filter(|p| match p.file_name() {
            Some(n) => names.contains(n.to_str().unwrap()),
            None => false,
        })
        .collect())
}

fn link_dir() -> Result<(), ProgError> {
    let pwd = Path::new(".");
    let somepwd = fs::canonicalize(pwd)?;
    let thisdir = somepwd.file_name().ok_or(ProgError::NoFile)?;
    let mut pwd_buf = PathBuf::from(METADATA_REPO);
    pwd_buf.push(thisdir);
    if pwd_buf.exists() {
        let filtered = filter_dir(&pwd_buf)?;
        let mut targets = vec![];
        for f in &filtered {
            let fname = f.file_name();
            if let Some(fname) = fname {
                let mut targf = PathBuf::from(&somepwd);
                targf.push(map_file(fname));
                targets.push([fs::canonicalize(&f)?, targf]);
            };
        }
        println!("Will link files {:?} to {:?}", filtered, somepwd);
        for [path, target] in targets {
            if target.exists() {
                println!("Already exist: {:?}", target);
            } else {
                println!("Will create symlink from {:?} to {:?}", path, target);
                std::os::unix::fs::symlink(path, target)?;
            };
        }
    } else {
        println!("No metadata exists for this dir");
        if confirm(&format!("Create new metadata dir {:?}", pwd_buf)) {
            fs::create_dir(&pwd_buf)?;
            println!("Created dir");
        } else {
            println!("Did nothing");
            process::exit(1);
        }
    }
    Result::Ok(())
}

fn main() {
    match link_dir() {
        Ok(()) => (),
        Result::Err(e) => {
            print!("error: {:?}", e);
            process::exit(1);
        }
    }
}
