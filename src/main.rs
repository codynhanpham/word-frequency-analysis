use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

mod utils;

fn main() {
    println!("Text frequency analysis tool ~ by @codynhanpham\n\n");

    // get folder directory
    let folder_dir = utils::get_folder_dir("Enter txt folder directory: ");
    // get target phrases json file path
    let phrases = utils::get_phrases_from_json("Enter the target phrases json file path (leave empty to disable): ");

    // start time
    let start = std::time::Instant::now();

    let folder_dir_path = PathBuf::from(folder_dir.clone());
    // list all .txt files in folder
    let mut txt_files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(folder_dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            if path.extension().unwrap() == "txt" {
                txt_files.push(path);
            }
        }
    }
    let number_of_files = txt_files.len();
    // print all .txt files name in folder
    println!("Found {} txt files in folder\n", number_of_files);

    // A combined hashmap of all word frequency accross all files for future analysis, a hashmap of hashmap with the key being the file name
    let mut master_word_freq: HashMap<String, HashMap<String, usize>> = HashMap::new();

    // Processing, by looping through each file
    for file in txt_files {
        let file_copy = file.clone();
        // file name
        let file_name = file_copy.file_stem().unwrap().to_str().unwrap();
        println!("------------------------------------------------------------");
        println!("Processing file: {}", file.display());
        // read file
        let text = fs::read_to_string(file).expect("Failed to read file");
        // keep the raw text for future analysis
        let raw_text = text.clone();
        // trim punctuations
        let text = utils::trim_punctuations(&text);
        // split into words
        let words = utils::split_into_words(&text);

        // 1: cased: both lowercase and uppercase, this is the original text, I would say that it's noisy, though can be interesting
        // let start = std::time::Instant::now();
        // println!("Number of words: {}", words.len());
        // // count word frequency
        // let word_freq = utils::word_frequency(&words, 8);
        // let duration = start.elapsed();

        // // print the hashmap size
        // println!("Number of unique words: {}", word_freq.len());
        // println!("Time: {:?}", duration);
        // println!();

        // 2: uncased: normalized case
        let start = std::time::Instant::now();
        let words_uncased: Vec<String> = utils::normalize_capitalization(&words);
        println!("Number of words (normalized): {}", words.len());
        // count word frequency
        let word_freq = utils::word_frequency(&words_uncased, 8);
        // count target phrases frequency
        let target_phrases_freq = utils::phrase_frequency(&raw_text, &phrases);
        let duration = start.elapsed();

        // print the hashmap size
        println!("Number of unique words (normalized): {}", word_freq.len());
        println!("Frequency calculation time: {:?}", duration);

        // sort the uncased word frequency, highest to lowest
        println!("Sorting word frequency...");
        let mut word_freq: Vec<_> = word_freq.into_iter().collect();
        word_freq.sort_by(|a, b| b.1.cmp(&a.1));
        // bring the capitalized words up top of the list
        let sorted_word_freq = utils::prioritize_capitalized_words(&word_freq);
        
        // also sort the target phrases frequency, highest to lowest
        let mut target_phrases_freq: Vec<_> = target_phrases_freq.into_iter().collect();
        target_phrases_freq.sort_by(|a, b| b.1.cmp(&a.1));
        // bring the capitalized words up top of the list
        let sorted_target_phrases_freq = utils::prioritize_capitalized_words(&target_phrases_freq);

        // pooled the word frequency and target phrases frequency to a "sorted_table", target phrases first
        let mut sorted_table: Vec<(String, usize)> = Vec::new();
        for (word, freq) in sorted_target_phrases_freq {
            sorted_table.push((word, freq));
        }
        for (word, freq) in sorted_word_freq {
            sorted_table.push((word, freq));
        }

        // save the word frequency to a file as "word: frequency" separated by newline
        let output_string = sorted_table.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<String>>().join("\n");
        
        // write to folder_dir_path\outputs\file_name.txt
        println!("Writing to file...");
        let output_file_path = folder_dir_path.join("outputs").join(format!("{}_wordFreq.txt", file_name));
        println!("{}", output_file_path.display());
        // create the outputs folder if it doesn't exist
        let outputs_folder_path = folder_dir_path.join("outputs");
        if !outputs_folder_path.exists() {
            fs::create_dir(outputs_folder_path).expect("Failed to create outputs folder");
        }
        fs::write(output_file_path, output_string).expect("Failed to write to file");
        // also write to folder_dir_path\outputs\file_name.csv
        let output_file_path = folder_dir_path.join("outputs").join(format!("{}_wordFreq.csv", file_name));
        let csv_string = utils::vec_to_csv_string(&sorted_table);
        fs::write(output_file_path, csv_string).expect("Failed to write to file");

        // add the word frequency to the master hashmap
        master_word_freq.insert(file_name.to_string(), sorted_table.into_iter().collect());

        println!("Done!");
    }
    println!("------------------------------------------------------------");
    // if there is only 1 file, it's done
    if number_of_files == 1 {
        // end time
        let duration = start.elapsed();
        println!("Finished processing 1 file in {:?}", duration);
        println!();

        // let up a user input so that the program doesn't exit immediately
        println!("\n");
        let _ = utils::get_input("Press enter to exit...");
        return;
    }

    // Otherwise
    // A giant table of all words and their frequency accross all files
    // in a HashMap<String, HashMap<String, usize>> where the first String is the word, the second String is the file name, and usize is the frequency

    let mut joined_words_freq: HashMap<String, HashMap<String, usize>> = HashMap::new();

    // loop through each file, make a union of the words
    for (file_name, word_freq) in master_word_freq.iter() {
        // loop through each word in the file
        for (word, freq) in word_freq.iter() {
            // if the word is already in the joined_words_freq, add the frequency to the existing hashmap
            if joined_words_freq.contains_key(word) {
                let file_freq = joined_words_freq.get_mut(word).unwrap();
                file_freq.insert(file_name.to_string(), *freq);
            } else {
                // otherwise, add the word to the joined_words_freq
                let mut file_freq: HashMap<String, usize> = HashMap::new();
                file_freq.insert(file_name.to_string(), *freq);
                joined_words_freq.insert(word.to_string(), file_freq);
            }
        }
    }

    let shared_words_csv_string = utils::combined_map_to_csv_string(joined_words_freq, &phrases);

    // write to folder_dir_path\outputs\data_joined.csv
    let output_file_path = folder_dir_path.join("outputs").join("data_joined.csv");
    fs::write(output_file_path, shared_words_csv_string).expect("Failed to write to file");

    println!("\nAll done!!\n");
    // end time
    let duration = start.elapsed();
    println!("Finished processing {} files in {:?}", number_of_files, duration);
    println!();

    // let up a user input so that the program doesn't exit immediately
    let _ = utils::get_input("Press enter to exit...");
}