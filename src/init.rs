extern crate clap;

use clap::{Arg, App, ArgMatches};
use std::path::Path;
use std::process;
use std::fs::File;
use std::io::{Read, Write};

fn get_match() -> ArgMatches<'static> {
  let matches = App::new("kt")
    .version("0.0.1")
    .author("xi gua <tomato.stao@gmail.com>")
    .about("A drop in cat replacement written in Rust")
    .arg(
      Arg::with_name("FILE")
      .help("FILE to print.")
      .empty_values(false)
    )
    .get_matches();
  matches
}

pub fn app() {
  let matches = get_match();

  if let Some(_file) = matches.value_of("FILE") {
    if Path::new(&_file).exists() {
      match File::open(_file) {
        Ok(mut item) => {
          let mut data = String::from("");
          item.read_to_string(&mut data).expect("[kt Error] Unable to read the file.");
          let stdout = std::io::stdout();
          let mut handle = std::io::BufWriter::new(stdout); // 可选项：将 handle 包装在缓冲区中
          match writeln!(handle, "{}", data) {
            Ok(_res) => {},
            Err(err) => {
              eprintln!("[kt Error] Unable to display the file contents. {:?}", err);
              process::exit(1);
            }
          }
        },
        Err(err) => {
          eprintln!("[kt Error] Unable to read the file. {:?}", err);
          process::exit(1);
        }
      }
    } else {
      eprintln!("[kt Error] Np such file or directory.");
      process::exit(1); // 程序错误终止时的标准退出码
    }
  }
}