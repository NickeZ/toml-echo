use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use toml::Value;

#[derive(StructOpt, Debug)]
#[structopt(name = "toml-echo")]
struct Opt {
    #[structopt(name = "TOMLFILE", parse(from_os_str))]
    tomlfile: PathBuf,
    #[structopt(name = "QUERY")]
    query: String,
    #[structopt(
        short = "q",
        long = "quiet",
        help = "Don't print output but exit with 0 if the query is non-empty"
    )]
    is_quiet: bool,
}

fn find_nearest_file<F: AsRef<OsStr>>(path: &Path, filename: F) -> Option<PathBuf> {
    for p in path.ancestors() {
        let mut candidate = p.to_path_buf();
        candidate.push(filename.as_ref());
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code);
}

fn real_main() -> i32 {
    let opt = Opt::from_args();

    // Will only panic if tomlfile ends in ".."
    let has_ancestors = opt.tomlfile != opt.tomlfile.file_name().unwrap();

    let mut path = opt.tomlfile;
    if !has_ancestors {
        path = if let Some(file) = find_nearest_file(&std::env::current_dir().unwrap(), path) {
            file
        } else {
            eprintln!("No tomfile found");
            return 2;
        };
    }

    let mut file_content = String::new();
    match File::open(path) {
        Ok(mut f) => {
            if let Err(e) = f.read_to_string(&mut file_content) {
                eprintln!("Couldn't read file: {}", e);
                return 4;
            }
        }
        Err(e) => {
            eprintln!("Couldn't open file: {}", e);
            return 8;
        }
    };

    let value = file_content.parse::<Value>().unwrap();
    let mut inner_value = None;
    for path in opt.query.split('.') {
        if inner_value.is_none() {
            inner_value = value.get(path);
            continue;
        }
        // Unwrap never panics because of check above
        inner_value = inner_value.unwrap().get(path);
    }

    match inner_value {
        Some(toml_value) => {
            if !opt.is_quiet {
                // Default printer prints strings quoted which we don't want
                if let Some(value) = toml_value.as_str() {
                    println!("{}", value);
                } else {
                    println!("{}", toml_value);
                }
            }

            return 0;
        }
        None => {
            if !opt.is_quiet {
                eprintln!("No matches in the toml file")
            }

            return 1;
        }
    };
}
