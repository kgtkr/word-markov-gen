use clap::{App, Arg, SubCommand};
use markov::Chain;

fn main() -> Result<(), Box<std::error::Error>> {
    let matches = App::new("Word Markov Gen")
        .subcommand(
            SubCommand::with_name("create").arg(
                Arg::with_name("order")
                    .long("order")
                    .short("o")
                    .default_value("1"),
            ),
        )
        .subcommand(
            SubCommand::with_name("gen").arg(
                Arg::with_name("count")
                    .long("count")
                    .short("c")
                    .default_value("100"),
            ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        let order = clap::value_t!(matches.value_of("order"), usize).unwrap_or_else(|e| e.exit());

        std::fs::read_to_string("english-words/words.txt")?
            .split("\n")
            .filter(|&word| word.len() > 0 && word.chars().all(|c| c.is_ascii_lowercase()))
            .map(|word| word.chars().collect::<Vec<_>>())
            .fold(Chain::of_order(order), |mut chain, word| {
                chain.feed(word);
                chain
            })
            .save("data.bin")?;
    }

    if let Some(matches) = matches.subcommand_matches("gen") {
        let count = clap::value_t!(matches.value_of("count"), usize).unwrap_or_else(|e| e.exit());

        Chain::<char>::load("data.bin")?
            .iter_for(count)
            .for_each(|word| println!("{}", word.into_iter().collect::<String>()));
    }

    Ok(())
}
