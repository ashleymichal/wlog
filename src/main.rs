use std::fs;
use std::io::prelude::*;
use std::path::Path;

use clap::{App, Arg};
use time;

fn main() {
    let matches = App::new("📝 wlog: one-line work logs")
        .version("1.0")
        .author("Ashley M. Lewis")
        .about("Track decisions in a project and document design progress")
        .arg(Arg::with_name("message").help("the message you want to log"))
        .arg(Arg::with_name("tag"))
        .get_matches();

    let message = matches.value_of("message").unwrap();
    let tag = matches.value_of("tag").unwrap();

    let log = format!("{} - {}\n", tag, message);
    println!("log: {:?}", log);
    let path = &Path::new(".wlog");
    let today = time::strftime("%Y-%m-%d", &time::now()).unwrap();
    fs::create_dir_all(path).unwrap();
    let filename = path.join(today);
    println!("{:?}", filename);
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();

    write!(file, "{}", &log).unwrap()
}
