use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;
use toml::Value;

#[derive(StructOpt, Debug)]
#[structopt(name = "toml-echo")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    tomlfile: PathBuf,
    #[structopt(name = "QUERY")]
    query: String,
}

fn main() {
    let opt = Opt::from_args();

    let mut file_content = String::new();
    File::open(opt.tomlfile)
        .unwrap()
        .read_to_string(&mut file_content)
        .unwrap();

    let value = file_content.parse::<Value>().unwrap();
    let mut inner_value = None;
    for path in opt.query.split(".") {
        if inner_value.is_none() {
            inner_value = value.get(path);
            continue;
        }
        inner_value = inner_value.unwrap().get(path);
    }

    // Default printer prints strings qouted which we don't want
    if let Some(value) = inner_value.unwrap().as_str() {
        println!("{}", value);
        return;
    }
    println!("{}", inner_value.unwrap());
}
