use std::fs::File;
use std::io::Read;
use std::ffi::OsStr;
use std::path::{Path,PathBuf};
use structopt::StructOpt;
use toml::Value;

#[derive(StructOpt, Debug)]
#[structopt(name = "toml-echo")]
struct Opt {
    #[structopt(long="tomlfile", short="f", parse(from_os_str))]
    tomlfile: Option<PathBuf>,
    #[structopt(name = "QUERY")]
    query: String,
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
    let opt = Opt::from_args();

    let tomlfile = match opt.tomlfile {
        Some(file) => {
            // Will only panic if file ends in ".."
            let has_ancestors = file != file.file_name().unwrap();
            if has_ancestors {
                file
            } else if let Some(file) = find_nearest_file(&std::env::current_dir().unwrap(), file) {
                file
            } else {
                eprintln!("No tomfile found");
                return
            }
        },
        None => {
            eprintln!("No tomfile found");
            return
        },
    };

    let mut file_content = String::new();
    match File::open(tomlfile) {
        Ok(mut f) => {
            if let Err(e) = f.read_to_string(&mut file_content) {
                eprintln!("Couldn't read file: {}", e);
                return;
            }
        },
        Err(e) => {
            eprintln!("Couldn't open file: {}", e);
            return;
        },
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
    // Unwrap never panics because of check above
    let inner_value = inner_value.unwrap();

    // Default printer prints strings quoted which we don't want
    if let Some(value) = inner_value.as_str() {
        println!("{}", value);
        return;
    }
    println!("{}", inner_value);
}
