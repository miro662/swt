extern crate ansi_term;
extern crate regex;
extern crate clap;
extern crate rand;

mod word;

use clap::{App, Arg};
use word::{WordBuilder, Language, WordBase, WordBaseOps};
use rand::Rng;

fn teach(wordbase:&WordBase, lang: Language) -> ! {
    let mut last_word = WordBuilder::new().build();
    'main: loop {
        let mut word = rand::thread_rng().choose(wordbase).unwrap();
        'choose: loop {
            if *word != last_word {
                break;
            }
            word = rand::thread_rng().choose(wordbase).unwrap();
        }
        word.ask(lang, false);
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
        .arg(Arg::with_name("lang")
            .short("l")
            .long("lang")
            .help("Language in which words will be asked")
            .takes_value(true)
            .default_value("polish")
            .possible_values(&["english", "polish"])
        )
        .arg(Arg::with_name("extended")
            .short("e")
            .long("extended")
            .help("Should SWT also teach extended-level words?")
        )
        .get_matches();
    
    let filename = matches.value_of("base").unwrap();

    let mut wordbase = WordBase::load(filename)
        .expect("Cannot load given wordbase");
    
    if !matches.is_present("extended") {
        wordbase = wordbase.get_basic_level();
    }
    
    let language = match matches.value_of("lang").unwrap() {
        "english" => Language::English,
        "polish" => Language::Polish,
        _ => panic!("Unknown language")
    };
    
    teach(&wordbase, language);
}
