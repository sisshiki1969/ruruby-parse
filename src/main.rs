extern crate clap;
//extern crate dirs;
extern crate ruruby_parse;

use clap::*;
use std::fs::*;
use std::io::Read;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, trailing_var_arg = true)]
struct Cli {
    /// one line of script. Several -e's allowed. Omit [programfile]
    #[clap(short, multiple_occurrences = true)]
    exec: Option<String>,

    /// print the version number, then turn on verbose mode
    #[clap(short)]
    verbose: bool,

    /// program file and arguments
    args: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    if cli.verbose {
        println!("{} {}", crate_name!(), crate_version!());
    }
    match cli.exec {
        Some(command) => {
            parse_and_output(command);
            return;
        }
        None => {}
    }

    let file = if cli.args.is_empty() {
        parse_and_output(include_str!("../quine/yamanote.rb").to_string());
        return;
    } else {
        &cli.args[0]
    };

    let absolute_path = match std::path::Path::new(file).canonicalize() {
        Ok(path) => path,
        Err(ioerr) => {
            eprintln!("{}: {}.", file, ioerr);
            return;
        }
    };

    let program = match load_file(&absolute_path) {
        Ok(program) => program,
        Err(err) => {
            eprintln!("{}: {}.", file, err);
            return;
        }
    };

    parse_and_output(program);
}

fn parse_and_output(program: String) {
    match ruruby_parse::Parser::parse_program(program, Path::new(""), "main") {
        Ok(res) => println!("{:#?}", res),
        Err(err) => panic!("{:?}\n{}", err.kind, err.source_info.get_location(&err.loc)),
    };
}

fn load_file(path: &Path) -> Result<String, String> {
    let mut file_body = String::new();
    match OpenOptions::new().read(true).open(path) {
        Ok(mut file) => match file.read_to_string(&mut file_body) {
            Ok(_) => {}
            Err(ioerr) => return Err(format!("{}", ioerr)),
        },
        Err(ioerr) => return Err(format!("{}", ioerr)),
    };
    Ok(file_body)
}

#[test]
fn yamanote() {
    parse_and_output(include_str!("../quine/yamanote.rb").to_string());
}
