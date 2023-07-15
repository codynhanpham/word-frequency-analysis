use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use std::fs;
use rayon::prelude::*;

// import utils/tables.rs
use crate::utils::tables;
use crate::utils::utils;

fn phrase_frequency(text: &str, phrases: &Vec<String>) -> HashMap<String, usize> {
    let freq_map: HashMap<_, _> = phrases
        .iter()
        .map(|sub| (sub.clone(), text.match_indices(sub).count()))
        .collect();

    freq_map
}

// main take in data and generate frequency analysis (tables and graphs), along with a master word frequency hashmap
// also take in phrases to search for and make a table of the frequency of those phrases
// then combine all the data into a single hashmap matching the file name with the word frequency hashmap
// master word frequency hashmap: HashMap<String, Vec<HashMap<String, usize>>> is a hashmap of <file name, chapter<<word, frequency>>>
pub fn main(folder_dir: &String, raw_data: HashMap<String, Vec<String>>, data: HashMap<String, Vec<Vec<String>>>, phrases: Vec<String>) -> HashMap<String, Vec<HashMap<String, usize>>> {
    println!("------------------------------------------------------------");
    println!("Analyzing word frequency...");
    // start time for the whole function
    let start_total = std::time::Instant::now();

    let start = std::time::Instant::now();
    // Count frequency of each word in each file and chapters (in parallel) (case sensitive, but normalized capitalization)
    let word_freq: HashMap<String, Vec<HashMap<String, usize>>> = data
        .par_iter()
        .map(|(file_name, chapters)| {
            let mut file_word_freq: Vec<HashMap<String, usize>> = Vec::new();
            for chapter in chapters {
                let mut chapter_word_freq: HashMap<String, usize> = HashMap::new();
                for word in chapter {
                    let count = chapter_word_freq.entry(word.clone()).or_insert(0);
                    *count += 1;
                }
                file_word_freq.push(chapter_word_freq);
            }
            (file_name.clone(), file_word_freq)
        })
        .collect();
    let duration = start.elapsed();
    println!("\x1b[2m  Word frequency count completed in {} ms\x1b[0m", duration.as_millis());

    let start = std::time::Instant::now();
    // Count frequency of each phrase in each file and chapters using the raw_data text (in parallel) (case sensitive)
    let phrase_freq: HashMap<String, Vec<HashMap<String, usize>>> = raw_data
        .par_iter()
        .map(|(file_name, chapters)| {
            let mut file_phrase_freq: Vec<HashMap<String, usize>> = Vec::new();
            for chapter in chapters {
                let mut chapter_phrase_freq: HashMap<String, usize> = HashMap::new();
                for (phrase, count) in phrase_frequency(&chapter, &phrases) {
                    chapter_phrase_freq.insert(phrase, count);
                }
                file_phrase_freq.push(chapter_phrase_freq);
            }
            (file_name.clone(), file_phrase_freq)
        })
        .collect();
    let duration = start.elapsed();
    println!("\x1b[2m  Phrase frequency count completed in {} ms\x1b[0m", duration.as_millis());

    let start = std::time::Instant::now();
    let mut master_word_freq_map: HashMap<String, Vec<HashMap<String, usize>>> = HashMap::new();
    for (file_name, word_freq_vec) in word_freq {
        let mut word_freq_vec = word_freq_vec;
        let phrase_freq_vec = phrase_freq.get(&file_name).unwrap();
        for (i, word_freq) in word_freq_vec.iter_mut().enumerate() {
            let phrase_freq = phrase_freq_vec.get(i).unwrap();
            for (phrase, count) in phrase_freq {
                word_freq.insert(phrase.clone(), *count);
            }
        }
        master_word_freq_map.insert(file_name, word_freq_vec);
    }

    let mut simple_word_freq_map: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for (file_name, word_freq_vec) in &master_word_freq_map {
        let mut simple_word_freq: HashMap<String, usize> = HashMap::new();
        for word_freq in word_freq_vec {
            for (word, freq) in word_freq {
                let count = simple_word_freq.entry(word.clone()).or_insert(0);
                *count += *freq;
            }
        }
        simple_word_freq_map.insert(file_name.clone(), simple_word_freq);
    }
    let duration = start.elapsed();
    println!("\x1b[2m  Frequency maps generated in {} ms\x1b[0m", duration.as_millis());

    let start = std::time::Instant::now();
    let simple_word_freq_map_no_stopwords = utils::remove_stopwords_no_chapters(&simple_word_freq_map);
    let duration = start.elapsed();
    println!("\x1b[2m  Stopwords removed in {} ms\x1b[0m", duration.as_millis());

    let start = std::time::Instant::now();
    // get all the file names
    let mut file_names: Vec<String> = Vec::new();
    for (file_name, _) in &simple_word_freq_map {
        file_names.push(file_name.clone());
    }
    file_names.sort();
    // get all the words into a HashSet
    let mut words_complete: HashSet<String> = HashSet::new();
    for (_, word_freq) in &simple_word_freq_map {
        for word in word_freq.keys() {
            words_complete.insert(word.clone());
        }
    }
    // same as words_complete, but without stopwords
    let mut words_no_stopword: HashSet<String> = HashSet::new();
    for (_, word_freq) in &simple_word_freq_map_no_stopwords {
        for word in word_freq.keys() {
            words_no_stopword.insert(word.clone());
        }
    }
    let duration = start.elapsed();
    println!("\x1b[2m  Formating data in {} ms\x1b[0m", duration.as_millis());


    // Generate CSV table(s)
    let start = std::time::Instant::now();
    let folder_dir_path = PathBuf::from(folder_dir.clone());
    let folder_name = Path::new(folder_dir).file_name().unwrap().to_str().unwrap();
    let outputs_folder_path = folder_dir_path.join("outputs");
    if !outputs_folder_path.exists() {
        fs::create_dir(&outputs_folder_path).expect("Failed to create outputs folder");
    }

    let csv_string = tables::combined_file_map_to_csv_string(&file_names, &words_complete, &simple_word_freq_map, &phrases);
    let output_file_path = outputs_folder_path.join(format!("{}_wordFreq.csv", folder_name));
    fs::write(&output_file_path, csv_string.as_bytes()).expect("Unable to write file");

    let csv_string = tables::combined_file_map_to_csv_string(&file_names, &words_no_stopword, &simple_word_freq_map_no_stopwords, &phrases);
    let output_file_path = outputs_folder_path.join(format!("{}_wordFreq_no-stopwords.csv", folder_name));
    fs::write(&output_file_path, csv_string.as_bytes()).expect("Unable to write file");

    // More CSV here

    let duration = start.elapsed();
    println!("\nCSV table(s) generated in {} ms", duration.as_millis());

    // Do more here



    // end time
    let duration = start_total.elapsed();
    println!("Word frequency analysis completed in {} ms", duration.as_millis());
    master_word_freq_map
}