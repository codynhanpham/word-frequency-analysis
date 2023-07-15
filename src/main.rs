use std::path::PathBuf;
use std::fs;

mod utils {
    pub mod utils;
    pub mod tables;
}

mod analyses {
    pub mod word_frequency;
}

fn main() {
    println!("\x1b[1;5mText frequency analysis tool ~ by @codynhanpham\x1b[0m");
    println!("https://github.com/codynhanpham/word-frequency-analysis\n\n");

    // get folder directory
    let folder_dir = utils::utils::get_folder_dir("Enter txt (documents) folder directory: ");
    // get target phrases json file path
    let phrases = utils::utils::get_phrases_from_json("Enter the target phrases json file path (leave empty to disable): ");

    println!("------------------------------------------------------------");
    println!("Analyzing folder: {}", folder_dir);
    println!("And scan for {} target phrases", phrases.len());

    // start time
    let start = std::time::Instant::now();

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
    // print all .txt files name in folder
    println!("--> Found {} txt files in folder", number_of_files);
    println!("------------------------------------------------------------");


    // Digest all files (Split by chapter if possible, and remove punctuations, split into words, and normalize capitalization)
    let digest_data = utils::utils::digest_files(&txt_files);
    let raw_data = digest_data.0;
    let data = digest_data.1;
    let corpus = digest_data.2;
    println!("There are {} unique words in the corpus. (raw, case-sensitive)", corpus.len());

    // Frequency analysis
    analyses::word_frequency::main(&folder_dir, raw_data, data, phrases);


    // end time
    let duration = start.elapsed();
    println!("------------------------------------------------------------");
    println!("Total time taken: {} ms\n", duration.as_millis());



    // wait for user input to exit
    utils::utils::get_input("Press enter to exit...");
}