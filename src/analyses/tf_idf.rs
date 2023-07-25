use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use::rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::fs;

use crate::utils::utils;
use crate::utils::tables;

// TF: (frequency of a word in the document) / (total number of words in the document)
// IDF: log_10(total number of documents / (Number of documents with the word in it).max(1)) --> if the word is not in any document, then the denominator is 1
// Log a warning if number of documents with the word in it is 0 and was forced to 1 if this happens
// TF-IDF: TF * IDF
fn calculate_tf_idf(data: &HashMap<String, Vec<HashMap<String, usize>>>) -> HashMap<String, Vec<HashMap<String, f64>>> {
    // return a hashmap of <file name, <word, tf-idf>>
    let start = std::time::Instant::now();
    // Number of documents = number of chapters across all files
    let number_of_documents = data.values().map(|chapters| chapters.len()).sum::<usize>();

    // Count number of documents with the word in it
    let mut number_of_documents_with_word: HashMap<String, usize> = HashMap::new();
    for (_, chapters) in data {
        for chapter in chapters {
            for (word, freq) in chapter {
                // Ignore the entry in this chapter if the frequency is 0
                if *freq == 0 {
                    continue;
                }
                let count = number_of_documents_with_word.entry(word.clone()).or_insert(0);
                *count += 1;
            }
        }
    }

    // Calculate TF-IDF in parallel
    let warning_list: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
    let tf_idf: HashMap<String, Vec<HashMap<String, f64>>> = data 
        .into_par_iter()
        .map_init(|| warning_list.clone(), |warning_list, (file_name, chapters)| {
            let mut tf_idf_file: Vec<HashMap<String, f64>> = Vec::new();
            for chapter in chapters {
                let mut tf_idf_chapter: HashMap<String, f64> = HashMap::new();
                for (word, frequency) in chapter {
                    let tf = *frequency as f64 / chapter.values().sum::<usize>() as f64;
                    let idf = (number_of_documents as f64 / *number_of_documents_with_word.get(word).unwrap_or(&1) as f64).log10();
                    if *number_of_documents_with_word.get(word).unwrap_or(&0) == 0 {
                        // Log a warning if number of documents with the word in it is 0 and was forced to 1 if this happens
                        // println!("\x1b[33m  WARNING: Number of documents with the word \"{}\" in it is 0 and was forced to 1 for IDF calculation\x1b[0m", word);
                        warning_list.lock().unwrap().insert(word.clone());
                    }
                    tf_idf_chapter.insert(word.clone(), tf * idf);
                }
                tf_idf_file.push(tf_idf_chapter);
            }
            (file_name.clone(), tf_idf_file)
        })
        .collect();

    // Print warning list
    for word in warning_list.lock().unwrap().iter() {
        println!("\x1b[33m  WARNING: Number of documents with the word \"{}\" in it is 0 and was forced to 1 for IDF calculation\x1b[0m", word);
    }

    let duration = start.elapsed();
    println!("\x1b[2m  TF-IDF calculation completed in {} ms\x1b[0m", duration.as_millis());

    tf_idf
}


// basically similar to word_frequency.rs, but with tf-idf values, and depends on the word_frequency.rs result
// main take in frequency data and generate tf-idf values for each word, along with a master tf-idf hashmap
// master tf-idf hashmap: <HashMap<String, usize>> is a hashmap of <word, tf-idf>
pub fn main(folder_dir: &String, data: &HashMap<String, Vec<HashMap<String, usize>>>, phrases: &Vec<String>) -> HashMap<String, Vec<HashMap<String, f64>>> {
    // TF-IDF is calculated using the chapters as "documents"
    // If there are no chapters, then each file is considered a "document"
    // If there is only 1 file and 1 chapter, do not calculate TF-IDF and return an empty hashmap

    if data.len() == 1 && data.values().next().unwrap().len() == 1 {
        println!("------------------------------------------------------------");
        println!("Only a single document with no chapters, skipping TF-IDF analysis");
        return HashMap::new();
    }

    // data is a HashMap of <file name, chapters<words, frequency>> --> No need to calculate word frequency again

    println!("------------------------------------------------------------");
    println!("ANALYZING TF-IDF...");
    // start time for the whole function
    let start_total = std::time::Instant::now();

    // Determine if there are chapters or not: if there are any file with chapters, then treat all files as having chapters
    let mut has_chapters = false;
    for (_, chapters) in data {
        if chapters.len() > 1 {
            has_chapters = true;
            break;
        }
    }

    if has_chapters {
        println!("There are chapter markers in the data, calculating TF-IDF using chapters as document units");
    } else {
        println!("There are no chapter markers in the data, calculating TF-IDF treating each file as a document unit");
    }

    // file names list
    let mut file_names: Vec<String> = Vec::new();
    for (file_name, _) in data {
        file_names.push(file_name.clone());
    }
    file_names.sort();

    // Calculate TF-IDF: no stopwords only
    let tf_idf_no_stopwords = calculate_tf_idf(&utils::remove_stopwords_with_chapters(data));
    let tf_idf_no_stopwords_words: HashSet<String> = tf_idf_no_stopwords
        .values()
        .flatten()
        .map(|chapter| chapter.keys())
        .flatten()
        .map(|word| word.clone())
        .collect();

    // Generate TF-IDF csv file(s)
    let start = std::time::Instant::now();
    let folder_dir_path = PathBuf::from(folder_dir.clone());
    let folder_name = Path::new(folder_dir).file_name().unwrap().to_str().unwrap();
    let outputs_folder_path = folder_dir_path.join("outputs");
    if !outputs_folder_path.exists() {
        fs::create_dir(&outputs_folder_path).expect("Failed to create outputs folder");
    }

    let tf_idf_csv_string = tables::tf_idf_combined_file_map_to_csv_string_f64_fullsize(&file_names, &tf_idf_no_stopwords_words, &tf_idf_no_stopwords, &phrases);
    let output_file_path = outputs_folder_path.join(format!("{}_TF-IDF_no-stopwords.csv", folder_name));
    fs::write(&output_file_path, tf_idf_csv_string.as_bytes()).expect("Unable to write file");

    // More csv files here

    let duration = start.elapsed();
    println!("CSV table(s) generated in {} ms", duration.as_millis());

    // Do more things here


    // end time for the whole function
    let duration_total = start_total.elapsed();
    println!("TF-IDF analysis completed in {} ms", duration_total.as_millis());

    // drop all intermediate data
    drop(tf_idf_no_stopwords_words);
    drop(tf_idf_csv_string);
    drop(output_file_path);
    drop(outputs_folder_path);
    drop(folder_name);
    drop(folder_dir_path);
    drop(file_names);
    drop(has_chapters);
    drop(start);
    drop(duration);
    drop(start_total);
    drop(duration_total);


    tf_idf_no_stopwords
}