use clap::{builder::PossibleValuesParser, command, Arg, ArgAction};
use regex::RegexSet;
use walkdir::WalkDir;

pub struct Input {
    pub names: Option<RegexSet>,
    pub paths: Vec<String>,
    pub types: Vec<EntryType>,
}

#[derive(PartialEq)]
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
        names: matches
            .get_many::<String>("names")
            .map(|inputs| RegexSet::new(inputs).expect("invalid regex pattern")),
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
    let name_closure = |entry: &walkdir::DirEntry| {
        input.names.is_none() || {
            input
                .names
                .as_ref()
                .unwrap()
                .is_match(entry.file_name().to_str().unwrap())
        }
    };

    let type_closure = |entry: &walkdir::DirEntry| {
        input.types.is_empty()
            || entry.file_type().is_dir() && input.types.contains(&EntryType::Dir)
            || entry.file_type().is_file() && input.types.contains(&EntryType::File)
            || entry.file_type().is_symlink() && input.types.contains(&EntryType::Link)
    };

    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| match e {
            Err(e) => {
                eprintln!("rfind: {}", e);
                None
            }
            Ok(entry) => Some(entry),
        })
        .filter(name_closure)
        .filter(type_closure)
        .for_each(|item| println!("{}", item.path().display()));
}
