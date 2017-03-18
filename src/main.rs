extern crate ansi_term;
extern crate regex;
extern crate clap;
extern crate rand;

mod word;

use clap::{App, Arg};
use word::{WordBuilder, Language, WordBase, WordBaseOps};
use rand::Rng;

fn teach(wordbase:&WordBase) -> ! {
    let mut last_word = WordBuilder::new().build();
    'main: loop {
        let mut word = rand::thread_rng().choose(wordbase).unwrap();
        'choose: loop {
            if *word != last_word {
                break;
            }
            word = rand::thread_rng().choose(wordbase).unwrap();
        }
        word.ask(Language::English);
        last_word = word.clone();
    }
}

fn main() {
    let matches = App::new("swt")
        .version("0.0.1")
        .author("miro662 (miro662@gmail.com)")
        .about("Simple word trainer")
        .arg(Arg::with_name("base")
            .short("b")
            .long("base")
            .value_name("FILE")
            .help("Name of file containing wordbase")
            .takes_value(true)
            .required(true))
        .get_matches();
    
    let filename = matches.value_of("base").unwrap();

    let wordbase = WordBase::load(filename)
        .expect("Cannot load given wordbase");
    
    teach(&wordbase);
}
