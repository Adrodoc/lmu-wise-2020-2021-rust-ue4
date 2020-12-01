use clap::{App, Arg};
use std::collections::BTreeMap;

const TEXT: &str = "text";
const SEPERATOR: &str = "seperator";

fn main() {
    let matches = App::new("ue4")
        .arg(
            Arg::with_name(TEXT)
                .short("t")
                .long("text")
                .help("The text to split using the seperator")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name(SEPERATOR)
                .short("s")
                .long("sep")
                .help("The seperator to use for splitting the text")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let text = matches.value_of(TEXT).unwrap();
    let seperator = matches.value_of(SEPERATOR).unwrap();
    let map = split(text, seperator);
    println!("{:#?}", map);
}

fn split<'l>(text: &'l str, seperator: &str) -> BTreeMap<usize, &'l str> {
    text.split(seperator)
        .filter(|element| element.len() != 0)
        .map(|element| (element.as_ptr() as usize - text.as_ptr() as usize, element))
        .collect()
}
