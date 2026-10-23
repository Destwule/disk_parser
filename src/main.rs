use std::{path::PathBuf, io::{self}};
use parse_clargs::{Arg, Parser, ValueKind};
use disk_parser::{
    mbr_parser::MbrParser, gpt_parser::GptParser, error_helpers::general_errors
};


fn run(disk: PathBuf) -> io::Result<()> {
    if disk.exists() {
        match GptParser::new(disk.clone()) {
            Err(err) if (err.kind() == io::ErrorKind::InvalidData) || (err.kind() == io::ErrorKind::UnexpectedEof) => {
                match MbrParser::new(disk.clone()) {
                    Err(err) => {
                        return Err(general_errors(err, &disk));
                    }
                    Ok(parser) => parser.run()?
                }
            }
            Err(err) => return Err(err),
            Ok(parser) => parser.run()?
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound, format!("'{}' file does not exist", disk.display())
        ))
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let arg = Arg::positional("disk_path")
        .help("The location of the disk to be read from")
        .value_kind(ValueKind::Str).required();
    let parser = Parser::new("disk_parser")
        .description("Display a partition table of a disk")
        .arg(arg).unwrap();
    
    let args: Vec<String> = std::env::args().skip(1).collect();
    let matches = match parser.parse(&args) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let disk_path = matches.get_str("disk_path").unwrap();
    let disk = PathBuf::from(disk_path);

    if let Err(err) = run(disk.clone()) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
    Ok(())
}
