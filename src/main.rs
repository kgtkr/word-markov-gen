use clap::{App, Arg, SubCommand};
use rand::seq::SliceRandom;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct DataBase {
    data: HashMap<Option<char>, Vec<Option<char>>>,
}

impl From<DataBaseJSON> for DataBase {
    fn from(DataBaseJSON { data }: DataBaseJSON) -> Self {
        DataBase {
            data: data.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct DataBaseJSON {
    data: Vec<(Option<char>, Vec<Option<char>>)>,
}

impl From<DataBase> for DataBaseJSON {
    fn from(DataBase { data }: DataBase) -> Self {
        DataBaseJSON {
            data: data.into_iter().collect(),
        }
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let matches = App::new("Word Markov Gen")
        .subcommand(SubCommand::with_name("create"))
        .subcommand(
            SubCommand::with_name("gen").arg(
                Arg::with_name("count")
                    .long("count")
                    .short("c")
                    .default_value("100"),
            ),
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("create") {
        let text = std::fs::read_to_string("english-words/words.txt")?;
        let words = text
            .split("\n")
            .filter(|&word| word.len() > 0 && word.chars().all(|c| c.is_ascii_lowercase()))
            .map(|word| word.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut data = HashMap::new();

        for word in words {
            let mut prev = None;
            for c in word {
                data.entry(prev).or_insert_with(Vec::new).push(Some(c));
                prev = Some(c);
            }
            data.entry(prev).or_insert_with(Vec::new).push(None);
        }
        std::fs::write(
            "data.json",
            serde_json::to_string(&DataBaseJSON::from(DataBase { data: data }))?,
        )?;
    }

    if let Some(matches) = matches.subcommand_matches("gen") {
        let count = clap::value_t!(matches.value_of("count"), u32).unwrap_or_else(|e| e.exit());

        let data = DataBase::from(serde_json::from_str::<DataBaseJSON>(
            &std::fs::read_to_string("data.json")?,
        )?);

        for _ in 0..count {
            let mut rng = rand::thread_rng();
            let mut word = Vec::new();
            let mut prev = None;
            loop {
                let next = data
                    .data
                    .get(&prev)
                    .and_then(|v| v.choose(&mut rng))
                    .cloned()
                    .unwrap_or(None);
                if let Some(next) = next {
                    word.push(next);
                    prev = Some(next);
                } else {
                    break;
                }
            }
            println!("{}", word.into_iter().collect::<String>());
        }
    }

    Ok(())
}
