use std::collections::{HashMap, HashSet};

pub fn combined_file_map_to_csv_string_usize(file_names: &Vec<String>, words_list: &HashSet<String>, data: &HashMap<String, HashMap<String, usize>>, phrases: &Vec<String>) -> String {
    // target is a csv file with the following format
    // headers: word, file1, file2, file3, file4, file5..., total
    // each row is a word, and the value of that word in each file, and the total value accross all files
    // the rows are sorted by the word alphabetical order

    println!("\nGenerating a CSV string...");

    let start = std::time::Instant::now();
    // Create a hashmap of <word, total value> so that we can sort the words by their total value
    let mut total_value: HashMap<String, usize> = HashMap::new();
    for word in words_list {
        let mut value = 0;
        for (_, word_value) in data {
            if word_value.contains_key(word) {
                value += word_value.get(word).unwrap();
            }
        }
        total_value.insert(word.clone(), value);
    }

    // categorize words into different vectors based on their properties
    let mut target_phrases: Vec<String> = Vec::new();
    let mut capitalized_words: Vec<String> = Vec::new();
    let mut lowercase_words: Vec<String> = Vec::new();
    let mut numeric_numbers: Vec<String> = Vec::new();

    for word in words_list {
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

    target_phrases.sort();
    capitalized_words.sort();
    lowercase_words.sort();
    numeric_numbers.sort();

    target_phrases.sort_by(|a, b| total_value.get(b).unwrap().cmp(total_value.get(a).unwrap()));
    capitalized_words.sort_by(|a, b| total_value.get(b).unwrap().cmp(total_value.get(a).unwrap()));
    lowercase_words.sort_by(|a, b| total_value.get(b).unwrap().cmp(total_value.get(a).unwrap()));
    numeric_numbers.sort_by(|a, b| total_value.get(b).unwrap().cmp(total_value.get(a).unwrap()));

    // combine the vectors in order
    let mut sorted_words: Vec<String> = Vec::new();
    sorted_words.extend(target_phrases);
    sorted_words.extend(capitalized_words);
    sorted_words.extend(lowercase_words);
    sorted_words.extend(numeric_numbers);
    let duration = start.elapsed();
    println!("\x1b[2m  Sorting data in {} ms\x1b[0m", duration.as_millis());
    

    // Create the csv string
    let start = std::time::Instant::now();
    // create the headers
    let mut result = String::from("Words");
    for file_name in file_names {
        result.push_str(&format!(",{}", file_name));
    }
    result.push_str(",Total\n");

    // create the rows
    for word in &sorted_words {
        result.push_str(&format!("{}", word));
        let mut total = 0;
        for file_name in file_names {
            if data.get(file_name).unwrap().contains_key(word) {
                let value = data.get(file_name).unwrap().get(word).unwrap();
                result.push_str(&format!(",{}", value));
                total += value;
            }
            else {
                result.push_str(",0");
            }
        }
        result.push_str(&format!(",{}\n", total));
    }
    let duration = start.elapsed();
    println!("\x1b[2m  Generating csv string in {} ms\x1b[0m", duration.as_millis());

    result
}

// similar to above but for the full size data and f64: HashMap<String, Vec<HashMap<String, f64>>> (includes chapters)
pub fn tf_idf_combined_file_map_to_csv_string_f64_fullsize(file_names: &Vec<String>, words_list: &HashSet<String>, data: &HashMap<String, Vec<HashMap<String, f64>>>, phrases: &Vec<String>) -> String {
    // Example:
    // Word, Book 1 _ #1, Book 1 _ #2, Book 1 Total, Book 2 _ #1, Book 2 _ #2, Book 2 _ #3, Book 2 Total, Corpus Total
    // coder, 1, 1, 2, 4, 2, 3, 9, 11

    println!("\nGenerating a CSV string...");

    let start = std::time::Instant::now();
    // Create a hashmap of <word, total value> so that we can sort the words by their total value (across all files)
    let mut total_value: HashMap<String, f64> = HashMap::new();
    for word in words_list {
        let mut value = 0.0;
        for (_, chapters) in data {
            for chapter in chapters {
                if chapter.contains_key(word) {
                    value += chapter.get(word).unwrap();
                }
            }
        }
        total_value.insert(word.clone(), value);
    }

    // categorize words into different vectors based on their properties
    let mut target_phrases: Vec<String> = Vec::new();
    let mut capitalized_words: Vec<String> = Vec::new();
    let mut lowercase_words: Vec<String> = Vec::new();
    let mut numeric_numbers: Vec<String> = Vec::new();

    for word in words_list {
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

    target_phrases.sort();
    capitalized_words.sort();
    lowercase_words.sort();
    numeric_numbers.sort();

    target_phrases.sort_by(|a, b| total_value.get(b).unwrap().partial_cmp(total_value.get(a).unwrap()).unwrap());
    capitalized_words.sort_by(|a, b| total_value.get(b).unwrap().partial_cmp(total_value.get(a).unwrap()).unwrap());
    lowercase_words.sort_by(|a, b| total_value.get(b).unwrap().partial_cmp(total_value.get(a).unwrap()).unwrap());
    numeric_numbers.sort_by(|a, b| total_value.get(b).unwrap().partial_cmp(total_value.get(a).unwrap()).unwrap());

    // combine the vectors in order
    let mut sorted_words: Vec<String> = Vec::new();
    sorted_words.extend(target_phrases);
    sorted_words.extend(capitalized_words);
    sorted_words.extend(lowercase_words);
    sorted_words.extend(numeric_numbers);
    let duration = start.elapsed();
    println!("\x1b[2m  Sorting data in {} ms\x1b[0m", duration.as_millis());

    // Check if the data has chapters --> affect how we generate the csv string
    // If there is no chapters, then the headers will simply be "Words", then the file names then "Corpus Total"
    // If there are chapters, then the headers will be something like Words, Book 1 _ #1, Book 1 _ #2, Book 1 Total, Book 2 _ #1, Book 2 _ #2, Book 2 _ #3, Book 2 Total, Corpus Total
    let mut has_chapters = false;
    for (_, chapters) in data {
        if chapters.len() > 1 {
            has_chapters = true;
            break;
        }
    }

    // Create the csv string
    let start = std::time::Instant::now();
    if has_chapters {
        // create the headers
        let mut result = String::from("Words");
        for file_name in file_names {
            let chapters = data.get(file_name).unwrap().len();
            for i in 0..chapters {
                result.push_str(&format!(",{} _ #{}", file_name, i + 1));
            }
            result.push_str(&format!(",{}_Total", file_name));
        }
        result.push_str(",Corpus Total\n");

        // create the rows
        for word in &sorted_words {
            result.push_str(&format!("{}", word));
            let mut corpus_total = 0.0;
            for file_name in file_names {
                let chapters = data.get(file_name).unwrap().len();
                let mut file_total = 0.0;
                for i in 0..chapters {
                    if data.get(file_name).unwrap()[i].contains_key(word) {
                        let value = data.get(file_name).unwrap()[i].get(word).unwrap();
                        result.push_str(&format!(",{}", value));
                        file_total += value;
                    }
                    else {
                        result.push_str(",0");
                    }
                }
                result.push_str(&format!(",{}", file_total));
                corpus_total += file_total;
            }

            result.push_str(&format!(",{}\n", corpus_total));
        }
        let duration = start.elapsed();
        println!("\x1b[2m  Generating csv string in {} ms\x1b[0m", duration.as_millis());

        return result;
    }
    // No chapters!
    else {
        // create the headers
        let mut result = String::from("Words");
        for file_name in file_names {
            result.push_str(&format!(",{}", file_name));
        }
        result.push_str(",Corpus Total\n");

        // create the rows
        for word in &sorted_words {
            result.push_str(&format!("{}", word));
            let mut corpus_total = 0.0;
            for file_name in file_names {
                if data.get(file_name).unwrap()[0].contains_key(word) { // Vec length is 1 if there is no chapters
                    let value = data.get(file_name).unwrap()[0].get(word).unwrap();
                    result.push_str(&format!(",{}", value));
                    corpus_total += value;
                }
                else {
                    result.push_str(",0");
                }
            }
            result.push_str(&format!(",{}\n", corpus_total));
        }
        let duration = start.elapsed();
        println!("\x1b[2m  Generating csv string in {} ms\x1b[0m", duration.as_millis());

        return result;
    }
}