use std::io;
use std::io::{BufReader, BufRead};
use ansi_term::Color::{Green, Red, Yellow};
use ansi_term::Style;
use regex::Regex;
use std::fs::File;

#[derive(Clone, Copy)]
pub enum Language {
    English,
    Polish,
}

#[derive(Clone, PartialEq)]
pub struct Word {
    english: Vec<String>,
    polish: Vec<String>,
    extended: bool,
}

impl Word {
    pub fn get_lang(&self, lang: Language) -> &Vec<String> {
        match lang {
            Language::English => &self.english,
            Language::Polish => &self.polish,
        }
    }

    pub fn check(&self, lang: Language, answer: &str) -> bool {
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

    pub fn get_answers(&self, lang: Language) -> String {
        let mut answers = String::new();
        let answers_vec = self.get_lang(lang);

        for (id, word) in answers_vec.iter().enumerate() {
            answers.push_str(&word);
            if id != answers_vec.len() - 1 {
                answers.push_str(", ");
            }
        }

        answers
    }

    pub fn ask(&self, lang: Language, once:bool) -> bool {
        let ask_lang = lang;
        let answer_lang = match lang {
            Language::English => Language::Polish,
            Language::Polish => Language::English,
        };
        let mut result = false;
            println!("{} {}",
                    Style::new().bold().paint(self.get_answers(ask_lang)),
                    if self.extended {
                        Yellow.paint("extended level")
                    } else {
                        Yellow.paint("basic level")
                    });
        
        'asking: loop {

            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Cannot read line");

            result = self.check(answer_lang, &answer);

            if result {
                println!("{}", Green.bold().paint("OK!"));
                break 'asking;
            } else {
                println!("{} {}",
                        Red.bold().paint("Wrong, correct answer(s):"),
                        self.get_answers(answer_lang));
                
                if once {
                    break 'asking;
                }
            };
        }
        println!("");

        result
    }

    pub fn is_extended(&self) -> bool {
        self.extended
    }
}

pub struct WordBuilder {
    english: Vec<String>,
    polish: Vec<String>,
    extended: bool,
}

impl WordBuilder {
    pub fn new() -> WordBuilder {
        WordBuilder {
            english: vec![],
            polish: vec![],
            extended: false,
        }
    }

    pub fn english(&mut self, word: &str) -> &mut WordBuilder {
        self.english.push(word.to_string());
        self
    }

    pub fn polish(&mut self, word: &str) -> &mut WordBuilder {
        self.polish.push(word.to_string());
        self
    }

    pub fn extended(&mut self) -> &mut WordBuilder {
        self.extended = true;
        self
    }

    pub fn build(&self) -> Word {
        Word {
            english: self.english.clone(),
            polish: self.polish.clone(),
            extended: self.extended,
        }
    }

    pub fn parse(&mut self, line: &str) -> &mut WordBuilder {
        // Trim whitespaces
        let line = line.trim();

        // Check if is extended word
        self.extended = false;
        let ext_regexp = Regex::new(r"^\*").unwrap();
        if ext_regexp.is_match(line) {
            self.extended();
        };

        // Split word into english and polish part
        let eq_pos = line.find("=").expect("Cannot find = sign");
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

pub type WordBase = Vec<Word>;

pub trait WordBaseOps {
    fn load(file: &str) -> io::Result<WordBase>;
    fn get_basic_level(&self) -> WordBase;
}

impl WordBaseOps for WordBase {
    fn load(file: &str) -> io::Result<WordBase> {
        let mut base: WordBase = Vec::new();

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
        let word_base: &WordBase = self;
        let mut basic_words: WordBase = Vec::new();

        for word in word_base.iter() {
            if !word.is_extended() {
                &basic_words.push(word.clone());
            }
        }

        basic_words
    }
}
