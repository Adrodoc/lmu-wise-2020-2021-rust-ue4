use clap::{App, Arg};
use std::collections::BTreeMap;

const TEXT: &str = "text";
const SEPERATOR: &str = "seperator";

fn main() {
    let matches = App::new("ue4")
        .arg(Arg::with_name(TEXT)
            .short("t")
            .long("text")
            .help("The text to split using the seperator")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name(SEPERATOR)
            .short("s")
            .long("sep")
            .help("The seperator to use for splitting the text")
            .takes_value(true)
            .required(true))
        .get_matches();
    let text = matches.value_of(TEXT).unwrap();
    let seperator = matches.value_of(SEPERATOR).unwrap();
    let map = split(text, seperator);
    println!("{:#?}", map);
}

fn split<'l>(t: &'l str, s: &str) -> BTreeMap<usize, &'l str> {
    t.split(s).filter(|e| e.len() != 0).map(|e| (e.as_ptr() as usize - t.as_ptr() as usize, e)).collect()
}
