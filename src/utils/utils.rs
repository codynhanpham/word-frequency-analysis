use std::path::{Path, PathBuf};
use std::io::{stdin,stdout,Write};
use std::collections::HashMap;
use std::collections::HashSet;
use rayon::prelude::*;
use std::fs;
use serde_json::Value;
use lazy_static::lazy_static;

const STOPWORDS_STRING: &str = include_str!("stopwords.txt");
fn parse_stopwords() -> HashSet<String> {
    let mut result: HashSet<String> = HashSet::new();
    for word in STOPWORDS_STRING.split_whitespace() {
        result.insert(word.to_string());
    }
    result
}

lazy_static! {
    static ref STOPWORDS: HashSet<String> = parse_stopwords();
}


pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().to_string()
}

pub fn get_folder_dir(prompt: &str) -> String {
    let mut input = get_input(prompt);
    input = input.trim_matches('"').to_string();
    input = input.trim_matches('\"').to_string();

    // check if directory exists
    if !Path::new(&input).exists() {
        println!("Location does not exist! Please enter a valid file path.");
        return get_folder_dir(prompt);
    }

    // check if directory is a folder
    if !Path::new(&input).is_dir() {
        println!("File is not a folder! Please enter a valid folder path.");
        return get_folder_dir(prompt);
    }

    input
}

// take in a prompt and an is_optinal arg, return the path String if it's valid, else return an empty String
pub fn get_file_dir(prompt: &str, is_optional: bool) -> String {
    let mut input = get_input(prompt);
    input = input.trim_matches('"').to_string();
    input = input.trim_matches('\"').to_string();

    // if input is empty and is_optional is true, return an empty String
    if input.is_empty() && is_optional {
        return input;
    }

    // check if directory exists
    if !Path::new(&input).exists() {
        println!("Location does not exist! Please enter a valid file path.");
        return get_file_dir(prompt, is_optional);
    }

    // check if directory is a file
    if !Path::new(&input).is_file() {
        println!("This is not a file! Please enter a valid file path.");
        return get_file_dir(prompt, is_optional);
    }

    input
}

pub fn get_json_path(prompt: &str) -> String {
    let input = get_file_dir(prompt, true);

    // if input is empty, return an empty String
    if input.is_empty() {
        return input;
    }

    // check if file is a .json file
    if !input.ends_with(".json") {
        println!("File is not a .json file! Please enter a valid .json file path.");
        return get_json_path(prompt);
    }

    input
}

pub fn get_phrases_from_json(json_path: &String) -> Vec<String> {
    // return an empty Vec if json_path is empty
    if json_path.is_empty() {
        return Vec::new();
    }

    // read file
    let phrases_txt = fs::read_to_string(&json_path).expect("Unable to read file");
    let phrases_parsed: Value = serde_json::from_str(&phrases_txt).expect("JSON was not well-formatted");
    // a vector of phrases to be targeted
    let phrases: Vec<String> = phrases_parsed["phrases"].as_array().unwrap_or(&Vec::new()).iter().map(|s| s.as_str().unwrap().to_string()).collect();

    phrases
}

pub fn get_chapter_separator_from_json(json_path: &String, default: String) -> String {
    // return default String if json_path is empty
    if json_path.is_empty() {
        return default;
    }

    // read file
    let phrases_txt = fs::read_to_string(&json_path).expect("Unable to read file");
    let phrases_parsed: Value = serde_json::from_str(&phrases_txt).expect("JSON was not well-formatted");
    // a vector of phrases to be targeted
    let mut chapter_separator = phrases_parsed["chapter_separator"].as_str().unwrap_or("<|eoc|>").to_string();
    // if chapter_separator is empty, use default
    if chapter_separator.is_empty() {
        chapter_separator = default;
    }

    chapter_separator
}

fn split_into_chapters(text: &str, separator: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for chapter in text.split(separator) {
        result.push(chapter.to_string());
    }

    result
}

fn trim_punctuations(text: &str) -> String {
    // turn punctuations, except for "-" (and some other exceptions below) into whitespace, then replace multiple whitespaces with one
    let mut result = String::new();
    let text = text.to_string();
    for c in text.chars() {
        if c.is_alphanumeric() || c.is_whitespace() || c == '-' || c == '*' || c == '’' { // add some other exceptions here
            result.push(c);
        } else {
            result.push(' ');
        }
    }
    result = result.split_whitespace().collect::<Vec<&str>>().join(" ");
    result
}

fn split_into_words(text: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for word in text.split_whitespace() {
        result.push(word.to_string());
    }

    result
}

// HashMap of <file name, Vec<Vec<String>> of chapters<words>>
fn file_collection(file_list: &Vec<PathBuf>, chapter_sep: &str) -> (HashMap<String, Vec<String>>, HashMap<String, Vec<Vec<String>>>) {
    let mut result_chapters: HashMap<String, Vec<String>> = HashMap::new();
    let mut result_words: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    for file in file_list {
        let file_name = file.file_stem().unwrap().to_str().unwrap();
        let text = fs::read_to_string(file).expect("Failed to read file");

        let chapters = split_into_chapters(&text, chapter_sep);

        // insert the raw text into the result_chapters hashmap
        result_chapters.insert(file_name.to_string(), chapters.clone());

        let mut chapters_words: Vec<Vec<String>> = Vec::new();
        for chapter in chapters {
            let words = split_into_words(&trim_punctuations(&chapter));
            chapters_words.push(words);
        }
        result_words.insert(file_name.to_string(), chapters_words);
    }

    // if there is no chapter separator, then the whole text is considered one chapter and the value is a Vec of length 1
    // return the HashMap of raw data split by chapter, and the HashMap of raw data split by words
    (result_chapters, result_words)
}

fn generate_word_corpus_set(file_collection: &HashMap<String, Vec<Vec<String>>>) -> HashSet<String> {
    println!("Generating word corpus...");
    // time start
    let start = std::time::Instant::now();
    let mut result: HashSet<String> = HashSet::new();
    for (_, chapters) in file_collection {
        for chapter in chapters {
            for word in chapter {
                result.insert(word.to_string());
            }
        }
    }
    let duration = start.elapsed();
    println!("\x1b[2m  Word corpus generated in {} ms\x1b[0m", duration.as_millis());

    result
}

// using the corpus, normalize the words in the file_collection and return a new file_collection
pub fn digest_files(file_list: &Vec<PathBuf>, chapter_sep: &str) -> (HashMap<String, Vec<String>>, HashMap<String, Vec<Vec<String>>>, HashSet<String>) {
    let file_collection = file_collection(file_list, chapter_sep);
    let corpus = generate_word_corpus_set(&file_collection.1);
    
    let start = std::time::Instant::now();
    let mut result_words: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    for (file_name, chapters) in file_collection.1 {
        let new_chapters = chapters.iter().map(|chapter| {
            chapter.iter().map(|word| {
                if let Some(lowercase_word) = corpus.get(&word.to_lowercase()) {
                    lowercase_word.to_string()
                } else {
                    word.to_string()
                }
            }).collect()
        }).collect();
        result_words.insert(file_name.to_string(), new_chapters);
    }
    let duration = start.elapsed();
    println!("\x1b[2m  Words' capitalization normalized in {} ms\x1b[0m", duration.as_millis());

    // return both the HashMap and the corpus
    (file_collection.0, result_words, corpus)
}

pub fn remove_stopwords_no_chapters(data: &HashMap<String, HashMap<String, usize>>) -> HashMap<String, HashMap<String, usize>> {
    // data is <file name, <word, frequency>>
    // delete stopwords from each file
    let start = std::time::Instant::now();

    // use parallel iterator
    let result: HashMap<String, HashMap<String, usize>> = data
        .par_iter()
        .map(|(file_name, word_freq)| {
            let mut new_word_freq: HashMap<String, usize> = HashMap::new();
            for (word, freq) in word_freq {
                if !STOPWORDS.contains(word) {
                    new_word_freq.insert(word.to_string(), *freq);
                }
            }
            (file_name.to_string(), new_word_freq)
        })
        .collect();

    let duration = start.elapsed();
    println!("\x1b[2m  Stopwords removed in {} ms\x1b[0m", duration.as_millis());
    result
}

pub fn remove_stopwords_with_chapters(data: &HashMap<String, Vec<HashMap<String, usize>>>) -> HashMap<String, Vec<HashMap<String, usize>>> {
    // data is <file name, chapters<word, frequency>>
    // delete stopwords from each file
    let start = std::time::Instant::now();

    // use parallel iterator
    let result: HashMap<String, Vec<HashMap<String, usize>>> = data
        .par_iter()
        .map(|(file_name, chapters)| {
            let mut new_chapters: Vec<HashMap<String, usize>> = Vec::new();
            for chapter in chapters {
                let mut new_word_freq: HashMap<String, usize> = HashMap::new();
                for (word, freq) in chapter {
                    if !STOPWORDS.contains(word) {
                        new_word_freq.insert(word.to_string(), *freq);
                    }
                }
                new_chapters.push(new_word_freq);
            }
            (file_name.to_string(), new_chapters)
        })
        .collect();

    let duration = start.elapsed();
    println!("\x1b[2m  Stopwords removed in {} ms\x1b[0m", duration.as_millis());
    result
}