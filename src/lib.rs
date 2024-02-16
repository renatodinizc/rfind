use clap::{builder::PossibleValuesParser, command, Arg, ArgAction};
use regex::RegexSet;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Input {
    pub names: Option<RegexSet>,
    pub paths: Vec<String>,
    pub types: Vec<EntryType>,
}

#[derive(Debug)]
pub enum EntryType {
    File,
    Dir,
    Link,
}

pub fn get_args() -> Input {
    let matches = command!()
        .arg(
            Arg::new("types")
                .help("select the type of files to find")
                .short('t')
                .long("type")
                .action(ArgAction::Append)
                .value_parser(PossibleValuesParser::new(["f", "d", "l"])),
        )
        .arg(
            Arg::new("names")
                .help("filter by a regex pattern")
                .short('n')
                .long("name")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("paths")
                .help("search paths")
                .action(ArgAction::Append)
                .default_value("."),
        )
        .get_matches();

    Input {
        names: matches.get_many::<String>("names").map(|inputs| RegexSet::new(inputs).expect("invalid regex pattern")),
        paths: matches
            .get_many::<String>("paths")
            .unwrap()
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
        types: match matches.get_many::<String>("types") {
            None => vec![],
            Some(values) => values
                .map(|value| {
                    if value == "f" {
                        EntryType::File
                    } else if value == "d" {
                        EntryType::Dir
                    } else {
                        EntryType::Link
                    }
                })
                .collect::<Vec<EntryType>>(),
        },
    }
}

pub fn execute(path: &String, input: &Input) {
    for file in WalkDir::new(path) {
        match file {
            Err(e) => eprintln!("rfind: {}", e),
            Ok(entry) => {
                if input.names.is_some() {
                    if input
                        .names
                        .as_ref()
                        .unwrap()
                        .is_match(entry.file_name().to_str().unwrap())
                    {
                        println!("{}", entry.path().display());
                    }
                } else {
                    println!("{}", entry.path().display());
                }
            }
        }
    }
}
