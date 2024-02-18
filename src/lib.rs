use clap::{builder::PossibleValuesParser, command, Arg, ArgAction};
use regex::RegexSet;
use walkdir::WalkDir;

pub struct Input {
    pub paths: Vec<String>,
    names: Option<RegexSet>,
    types: Vec<EntryType>,
    max_depth: Option<usize>,
    min_depth: Option<usize>,
}

#[derive(PartialEq)]
enum EntryType {
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
            Arg::new("max_depth")
                .help("Descend at most levels (a non-negative integer) of directories below the starting-points")
                .long("max-depth")
                .value_parser(clap::value_parser!(usize)),
            )
        .arg(
            Arg::new("min_depth")
                .help("Do not apply any tests or actions at levels less than levels (a non-negative integer)")
                .long("min-depth")
                .value_parser(clap::value_parser!(usize)),
            )
        .arg(
            Arg::new("paths")
                .help("search paths")
                .action(ArgAction::Append)
                .default_value("."),
        )
        .get_matches();

    Input {
        names: matches.get_many::<String>("names").map(|inputs| {
            let regex_patterns = inputs
                .map(|pattern| glob_to_regex(pattern))
                .collect::<Vec<String>>();
            RegexSet::new(regex_patterns).expect("invalid regex pattern")
        }),
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
        max_depth: matches.get_one::<usize>("max_depth").copied(),
        min_depth: matches.get_one::<usize>("min_depth").copied(),
    }
}

pub fn execute(path: &String, input: &Input) {
    let name_closure = |entry: &walkdir::DirEntry| {
        input.names.is_none()
            || input
                .names
                .as_ref()
                .unwrap()
                .is_match(entry.file_name().to_str().unwrap())
    };

    let type_closure = |entry: &walkdir::DirEntry| {
        input.types.is_empty()
            || entry.file_type().is_dir() && input.types.contains(&EntryType::Dir)
            || entry.file_type().is_file() && input.types.contains(&EntryType::File)
            || entry.file_type().is_symlink() && input.types.contains(&EntryType::Link)
    };

    let max_depth_closure = |entry: &walkdir::DirEntry| {
        input.max_depth.is_none() || entry.depth() <= input.max_depth.unwrap()
    };

    let min_depth_closure = |entry: &walkdir::DirEntry| {
        input.min_depth.is_none() || entry.depth() >= input.min_depth.unwrap()
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
        .filter(max_depth_closure)
        .filter(min_depth_closure)
        .for_each(|item| println!("{}", item.path().display()));
}

fn glob_to_regex(pattern: &str) -> String {
    let mut regex = String::from("^");
    for char in pattern.chars() {
        match char {
            '*' => regex.push_str(".*"),
            '?' => regex.push('.'),
            '.' | '(' | ')' | '{' | '}' | '[' | ']' | '+' | '|' | '^' | '$' | '\\' => {
                regex.push('\\');
                regex.push(char);
            }
            _ => regex.push(char),
        }
    }
    regex.push('$');
    regex
}
