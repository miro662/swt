# swt
Very simple word teacher designed for people preparing for polish Matura exam, written in Rust.

## Syntax
```
swt wordbase
```

## Wordbase
SWT relies on wordbases. They are UTF-8 textfiles. Every line contains 1 word.

The basic syntax of the word is (whitespaces between commas, equation signs and asterisks are ignored):
```
(*)english_meanings = polish_meanings
```

Asterisk at beginning of definition is optional. It means that this word is on extended Matura exam level. Words without asterisk are words for both basic and extended Matura exams.

There can be multiple meanings both in English and in Polish, they should be seperated by commas, e.g
```
prison, jail = więzienie, zakład karny
```
