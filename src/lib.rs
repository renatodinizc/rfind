use clap::{builder::PossibleValuesParser, command, Arg, ArgAction};

#[derive(Debug)]
pub struct Input {
    pub paths: Vec<String>,
    pub types: Vec<EntryType>,
    pub names: Vec<String>,
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
        names: match matches.get_many::<String>("names") {
            None => vec![],
            Some(values) => values.map(|v| v.to_string()).collect::<Vec<String>>(),
        },
    }
}
