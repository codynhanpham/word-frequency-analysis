use std::path::{Path};
use std::io::{stdin,stdout,Write};
use std::collections::HashMap;
use std::thread;
use std::fs;
use serde_json::Value;


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
        println!("File is not a file! Please enter a valid file path.");
        return get_file_dir(prompt, is_optional);
    }

    input
}

// Prompt to get a file_dir, only allow .json files, then parse the file into a HashMap, collect "phrases" into a Vec<String> and return it
pub fn get_phrases_from_json(prompt: &str) -> Vec<String> {
    let input = get_file_dir(prompt, true);

    // if input is empty, return an empty Vec
    if input.is_empty() {
        return Vec::new();
    }

    // check if file is a .json file
    if !input.ends_with(".json") {
        println!("File is not a .json file! Please enter a valid .json file path.");
        return get_phrases_from_json(prompt);
    }

    // read file
    let phrases_txt = fs::read_to_string(&input).expect("Unable to read file");
    let phrases_parsed: Value = serde_json::from_str(&phrases_txt).expect("JSON was not well-formatted");
    // a vector of phrases to be targeted
    let phrases: Vec<String> = phrases_parsed["phrases"].as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_string()).collect();

    phrases
}

pub fn trim_punctuations(text: &str) -> String {
    // turn punctuations, except for "-" into whitespace, then replace multiple whitespaces with one
    let mut result = String::new();
    let mut text = text.to_string();
    // replace "’" with "'"
    text = text.replace("’", "'");
    for c in text.chars() {
        if c.is_alphanumeric() || c.is_whitespace() || c == '-' || c == '*' { // add some other exceptions here
            result.push(c);
        } else {
            result.push(' ');
        }
    }
    result = result.split_whitespace().collect::<Vec<&str>>().join(" ");
    result
}

pub fn split_into_words(text: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for word in text.split_whitespace() {
        result.push(word.to_string());
    }

    result
}

fn search_word_in_vec(word: &str, vec: &[String]) -> bool {
    use rayon::prelude::*;
    // parallelize search for word in vec using rayon
    let found = vec.par_iter().any(|s| s == word);

    found
}

// a capitalization normalization function: if a capitalized word has a lowercase version in the text, then the lowercase version is used instead
pub fn normalize_capitalization(input: &[String]) -> Vec<String> {
    println!("NORMALIZING CAPITALIZATION...");
    let mut lowercased: Vec<String> = Vec::new();
    let mut capitalized: Vec<String> = Vec::new();

    // split words into two vectors: lowercased and capitalized
    for word in input {
        if word.chars().next().unwrap().is_lowercase() {
            lowercased.push(word.to_string());
        } else {
            capitalized.push(word.to_string());
        }
    }
    // make a vector of unique capitalized words
    let mut unique_capitalized: Vec<String> = Vec::new();
    for word in input {
        if word.chars().next().unwrap().is_uppercase() {
            if !unique_capitalized.contains(&word.to_string()) {
                unique_capitalized.push(word.to_string());
            }
        }
    }

    // print the number of capitalized words vs. unique capitalized words
    println!("Number of capitalized words: {}", capitalized.len());
    println!("Number of unique capitalized words: {}", unique_capitalized.len());

    // parallelize search for lowercase versions of capitalized words using search_word_in_vec function
    // if a lowercase version is found, then go through the capitalized vector and replace all capitalized versions with the lowercase version
    // if a lowercase version is not found, then the capitalized vector is unchanged for that word
    println!("Un-casing...");
    for (i, word) in unique_capitalized.iter().enumerate() {
        if search_word_in_vec(&word.to_lowercase(), &lowercased) {
            for j in 0..capitalized.len() {
                if capitalized[j] == *word {
                    capitalized[j] = word.to_lowercase();
                }
            }
        } else {
            continue;
        }
        print!("\r{}/{}", i + 1, unique_capitalized.len());
        let _ = std::io::stdout().flush();
    }
    println!();
    let mut result: Vec<String> = Vec::new();
    result.extend(lowercased);
    result.extend(capitalized);

    result

}

// multithreaded word frequency counter, where input is a vector of String words
pub fn word_frequency(input: &[String], worker_count: usize) -> HashMap<String, usize> {
    println!("COUNTING WORD FREQUENCY...");
    let mut result: HashMap<String, usize> = HashMap::new();
    let chunks = input.chunks((input.len() / worker_count).max(1));
    let mut handles = Vec::new();

    for chunk in chunks {
        let word = chunk.to_vec();
        // return a HashMap from each thread, the JoinHandle wraps this hashmap
        let handle = thread::spawn(move || {
            let mut map: HashMap<String, usize> = HashMap::new();
            for n in word {
                *map.entry(n).or_default() += 1;
            }
            map
        });
        handles.push(handle);
    }

    // wait for each thread to finish and combine every HashMap into the final result
    for handle in handles {
        let map = handle.join().unwrap();
        for (key, value) in map {
            *result.entry(key).or_default() += value;
        }
    }

    drop(input);
    return result;
}

pub fn phrase_frequency(text: &str, phrases: &Vec<String>) -> HashMap<String, usize> {
    let freq_map: HashMap<_, _> = phrases
        .iter()
        .map(|sub| (sub.clone(), text.match_indices(sub).count()))
        .collect();

    freq_map
}

pub fn prioritize_capitalized_words(input: &Vec<(String, usize)>) -> Vec<(String, usize)> {
    let mut result: Vec<(String, usize)> = Vec::new();
    let mut capitalized: Vec<(String, usize)> = Vec::new();
    let mut lowercased: Vec<(String, usize)> = Vec::new();
    let mut numeric: Vec<(String, usize)> = Vec::new();

    // split the input vector into three vectors: capitalized, lowercased, and numeric
    for (word, frequency) in input {

        if word.parse::<f64>().is_ok() {
            numeric.push((word.to_string(), *frequency));
        }
        else if word.chars().next().unwrap().is_uppercase() {
            capitalized.push((word.to_string(), *frequency));
        }
        else {
            lowercased.push((word.to_string(), *frequency));
        }
    }

    // sort the three vectors, by alphabetical order first (a to z), then by frequency (highest to lowest)
    capitalized.sort_by(|b, a| b.0.cmp(&a.0));
    capitalized.sort_by(|a, b| b.1.cmp(&a.1));
    lowercased.sort_by(|b, a| b.0.cmp(&a.0));
    lowercased.sort_by(|a, b| b.1.cmp(&a.1));
    numeric.sort_by(|b, a| b.0.cmp(&a.0));
    numeric.sort_by(|a, b| b.1.cmp(&a.1));

    // combine the three vectors into one
    result.extend(capitalized);
    result.extend(lowercased);
    result.extend(numeric);

    result
}

pub fn vec_to_csv_string(input: &Vec<(String, usize)>) -> String {
    // csv string format: word,frequency\n
    // header: word,frequency\n
    let mut result = String::from("word,frequency\n");
    for (word, frequency) in input {
        result.push_str(&format!("{},{}\n", word, frequency));
    }
    result
}

// take a hashmap of <word, <file name, frequency>> and return a csv string
pub fn combined_map_to_csv_string(data: HashMap<String, HashMap<String, usize>>, phrases: &Vec<String>) -> String {
    // target is a csv file with the following format
    // headers: word, file1, file2, file3, file4, file5..., total
    // each row is a word, and the frequency of that word in each file, and the total frequency accross all files
    // the rows are sorted by the word alphabetical order

    // first, get a list of all the words
    let mut words: Vec<String> = Vec::new();
    for (word, _) in &data {
        words.push(word.to_string());
    }

    // then, get a list of all the files, these will be the headers
    let mut files: Vec<String> = Vec::new();
    // go through all of the words, and add the files to the files vector if they aren't already in there
    for (_, file) in &data {
        for (file_name, _) in file {
            if !files.contains(file_name) {
                files.push(file_name.to_string());
            }
        }
    }
    files.sort();

    // create the header
    let mut result = String::from("Words");
    for file in &files {
        result.push_str(&format!(",{}", file));
    }
    result.push_str(",Total\n");
    
    // categorize words into different vectors based on their properties
    let mut target_phrases: Vec<String> = Vec::new();
    let mut capitalized_words: Vec<String> = Vec::new();
    let mut lowercase_words: Vec<String> = Vec::new();
    let mut numeric_numbers: Vec<String> = Vec::new();

    for word in &words {
        // if the word is a target phrase, add it to the target_phrases vector
        if phrases.contains(word) {
            target_phrases.push(word.to_string());
        }
        else if word.chars().next().unwrap().is_uppercase() {
            capitalized_words.push(word.to_string());
        }
        else if word.chars().all(char::is_numeric) {
            numeric_numbers.push(word.to_string());
        }
        else {
            lowercase_words.push(word.to_string());
        }
    }

    // combine the vectors in order
    let mut sorted_words: Vec<String> = Vec::new();
    sorted_words.extend(target_phrases);
    sorted_words.extend(capitalized_words);
    sorted_words.extend(lowercase_words);
    sorted_words.extend(numeric_numbers);
    
    // create the rows
    let mut rows: Vec<(String, usize, bool, bool)> = Vec::new();
    for word in &sorted_words {
        let mut total = 0;
        for file in &files {
            if data[word].contains_key(file) {
                total += data[word][file];
            }
        }
        let is_capitalized = word.chars().next().unwrap().is_uppercase();
        let is_target_phrase = phrases.contains(word);
        rows.push((word.to_string(), total, is_capitalized, is_target_phrase));
    }

    // sort the rows: by target_phrase, then by is_capitalized, then by total, then by word
    rows.sort_by(|a, b| {
        if a.3 != b.3 {
            // Sort by is_target_phrase in descending order
            b.3.cmp(&a.3)
        } else if a.2 != b.2 {
            // Sort by is_capitalized in descending order
            b.2.cmp(&a.2)
        } else if a.1 != b.1 {
            // Sort by total in descending order
            b.1.cmp(&a.1)
        } else {
            // Sort by word in ascending order
            a.0.cmp(&b.0)
        }
    });

    // create the final result string
    let mut result = String::from("Words");
    for file in &files {
        result.push_str(&format!(",{}", file));
    }
    result.push_str(",Total\n");

    for (word, total, _, _) in rows {
        result.push_str(&format!("{}", word));
        for file in &files {
            if data[&word].contains_key(file) {
                result.push_str(&format!(",{}", data[&word][file]));
            } else {
                result.push_str(",0");
            }
        }
        result.push_str(&format!(",{}\n", total));
    }

    result
}