extern crate ansi_term;
extern crate regex;

use std::env;
use std::io;
use std::io::{BufReader, BufRead};
use ansi_term::Color::{Green, Red, Yellow};
use ansi_term::Style;
use regex::Regex;
use std::fs::File;

#[derive(Clone, Copy)]
enum Language {
    English,
    Polish
}

#[derive(Clone)]
struct Word {
    english:Vec<String>,
    polish:Vec<String>,
    extended: bool
}

impl Word {
    fn get_lang(&self, lang:Language) -> &Vec<String> {
            match lang {
                Language::English => &self.english,
                Language::Polish => &self.polish
            }
    }

    fn check(&self, lang:Language, answer:&str) -> bool {
        let answers = self.get_lang(lang); 

        let answer = answer.trim();

        let mut is_correct = false;

        for word in answers {
            if word == answer {
                is_correct = true;
            }
        }
        
        is_correct
    }

    fn get_answers(&self, lang:Language) -> String {
        let mut answers = String::new();
        let answers_vec = self.get_lang(lang);
        
        for (id, word) in answers_vec.iter().enumerate() {
            answers.push_str(&word);
            if id != answers_vec.len() - 1 {
                answers.push_str(", ");
            }
        };

        answers
    }

    fn ask(&self, lang:Language) -> bool {
        let ask_lang = lang;
        let answer_lang = match lang {
            Language::English => Language::Polish,
            Language::Polish => Language::English
        };
        
        println!("{} {}", Style::new().bold().paint(self.get_answers(ask_lang)), if self.extended {
            Yellow.paint("extended level")
        }
        else
        {
            Yellow.paint("basic level")
        });
        
        let mut answer = String::new();
        io::stdin().read_line(&mut answer)
            .expect("Cannot read line");

        let result = self.check(answer_lang, &answer);

        if result {
            println!("{}", Green.bold().paint("OK!"));
        } 
        else {
            println!("{} {}", Red.bold().paint("Wrong, correct answer(s):"),self.get_answers(answer_lang)); 
        };

        result
    }

    fn is_extended(&self) -> bool {
        self.extended
    }
}

struct WordBuilder {
    english:Vec<String>,
    polish:Vec<String>,
    extended: bool
}

impl WordBuilder {
    fn new() -> WordBuilder {
        WordBuilder {english: vec!(), polish: vec!(), extended: false}
    }  

    fn english(&mut self, word:&str) -> &mut WordBuilder {
        self.english.push(word.to_string());
        self
    }

    fn polish(&mut self, word:&str) -> &mut WordBuilder {
        self.polish.push(word.to_string());
        self
    }
    
    fn extended(&mut self) -> &mut WordBuilder {
        self.extended = true;
        self
    }

    fn build(&self) -> Word {
        Word { english: self.english.clone(), polish: self.polish.clone(), extended: self.extended }
    }

    fn parse(&mut self, line:&str) -> &mut WordBuilder {
        // Trim whitespaces
        let line = line.trim();
       
        // Check if is extended word
        self.extended = false;
        let ext_regexp = Regex::new(r"^\*").unwrap();
        if ext_regexp.is_match(line) {
            self.extended();
        };

        // Split word into english and polish part
        let eq_pos = line.find("=").
            expect("Cannot find = sign");
        let (eng_part, pol_part) = line.split_at(eq_pos);

        // Add English meanings
        let meanings_en = eng_part.split(',');
        for meaning_en in meanings_en {
            let meaning_en = meaning_en.trim_matches('*');
            self.english(meaning_en.trim());    
        }

        // Add Polish meanings
        let meanings_pol = pol_part.split(',');
        for meaning_pol in meanings_pol {
            let meaning_pol = meaning_pol.trim_matches('=');
            self.polish(meaning_pol.trim());
        }

        self
    }
}

type WordBase = Vec<Word>;
    
trait WordBaseOps
{
    fn load(file:&str) -> io::Result<WordBase>;
    fn get_basic_level(&self) -> WordBase;
}

impl WordBaseOps for WordBase {
    fn load(file:&str) -> io::Result<WordBase> {
        let mut base:WordBase = Vec::new();
        
        // Open file
        let f = try!(File::open(file));
        let f_reader = BufReader::new(f);

        for line in f_reader.lines() {
            let line = try!(line);
            let word = WordBuilder::new()
                .parse(&line)
                .build();
            base.push(word);
        }

        Ok(base)
    }

    fn get_basic_level(&self) -> WordBase {
        let word_base:&WordBase = self;
        let mut basic_words:WordBase = Vec::new();

        for word in word_base.iter() {
            if !word.is_extended() {
                &basic_words.push(word.clone());
            }
        };

        basic_words
    }
}

fn main() {
    let args:Vec<_> = env::args().collect();

    if args.len() > 1 {
        let base = WordBase::load(&args[1])
            .expect("Cannot load file");

        for word in base {
            word.ask(Language::English);
            println!("");
        }
    } else {
        println!("Syntax: swt base (--polish/english) (--basic/extended) (--learn) (--test x)"); 
    }
}
