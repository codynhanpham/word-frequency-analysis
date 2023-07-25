use std::path::PathBuf;
use std::fs;

mod utils {
    pub mod utils;
    pub mod tables;
}

mod analyses {
    pub mod word_frequency;
    pub mod tf_idf;
}

fn main() {
    println!("\x1b[1;36mText frequency analysis tool ~ by @codynhanpham\x1b[0m");
    println!("https://github.com/codynhanpham/word-frequency-analysis\n\n");

    // get folder directory
    let folder_dir = utils::utils::get_folder_dir("Enter txt (documents) folder directory: ");
    // list all .txt files in folder
    let mut txt_files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(&folder_dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            if path.extension().unwrap() == "txt" {
                txt_files.push(path);
            }
        }
    }
    let number_of_files = txt_files.len();
    println!("\x1b[2m  --> Found \x1b[0;1m{}\x1b[0;2m txt files in folder\x1b[0m", number_of_files);

    // if no .txt files found, return main() for restart
    if number_of_files == 0 {
        println!("Please make sure there are .txt files in the folder\n");
        utils::utils::get_input("Press enter to restart...");
        // Restart
        println!("Trying to clear screen...");
        clearscreen::clear().expect("Failed to clear screen. The program should still work fine, though!\n\n");
        return main();
    }
    

    // get settings.json file path
    let settings = utils::utils::get_json_path("Enter the settings.json file path (leave empty for default): ");

    // get target phrases
    let phrases = utils::utils::get_phrases_from_json(&settings);
    // get chapter separator: from settings.json or default to "<|eoc|>"
    let chapter_separator = utils::utils::get_chapter_separator_from_json(&settings, "<|eoc|>".to_string());


    println!("------------------------------------------------------------");
    println!("Analyzing folder: {}", folder_dir);
    println!("And scan for {} target phrases", phrases.len());
    println!("------------------------------------------------------------");

    // start time
    let start = std::time::Instant::now();


    // Digest all files (Split by chapter if possible, and remove punctuations, split into words, and normalize capitalization)
    let digest_data = utils::utils::digest_files(&txt_files, &chapter_separator);
    let raw_data = digest_data.0;
    let data = digest_data.1;
    let corpus = digest_data.2;
    let normalized_corpus = digest_data.3;
    println!("There are {} unique words in the raw corpus", corpus.len());
    println!("There are {} after being normalized ({}%)", normalized_corpus.len(), (normalized_corpus.len() as f64 / corpus.len() as f64 * 100.0 * 100.0).round() / 100.0);

    // Analysis
    let word_freq_map = analyses::word_frequency::main(&folder_dir, raw_data, data, &phrases);
    let _tf_idf = analyses::tf_idf::main(&folder_dir, &word_freq_map, &phrases);


    // Do more here


    // end time
    let duration = start.elapsed();
    println!("------------------------------------------------------------");
    println!("Total time taken: {} ms\n", duration.as_millis());

    // drop all data
    drop(corpus);
    drop(word_freq_map);
    drop(_tf_idf);
    drop(txt_files);
    drop(folder_dir);
    drop(settings);
    drop(phrases);
    drop(chapter_separator);
    drop(start);
    drop(duration);
    drop(normalized_corpus);
    drop(number_of_files);

    // wait for user input to exit/restart: blank to restart, anything else to exit
    let input = utils::utils::get_input("Press enter to restart, or type anything to exit...");
    if input == "" {
        drop(input);
        // Restart
        println!("Trying to clear screen...");
        clearscreen::clear().expect("Failed to clear screen. The program should still work fine, though!\n\n");
        return main();
    }
}