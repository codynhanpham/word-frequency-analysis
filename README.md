# Word Frequency Analysis

A simple tool to quickly analyze and visualize the frequency of words in text files.

## Features Checklist

- [x] Batch process all `TXT` files in a folder
- [x] Normalization of capitalized words
- [x] Specify any additional target phrases to include
- [x] Generate a **word frequency table** in `CSV` and `TXT` formats
- [ ] Visualizations (work in progress)


# Installation

You can either build the tool from source or download the latest release from the [releases page](https://github.com/codynhanpham/word-frequency-analysis/releases).

### Build from Source

This is a Rust project, so you probably know what you are doing XD

### Download the Latest Release

The pre-built binary is only available for Windows at the moment. You can check out the [releases page](https://github.com/codynhanpham/word-frequency-analysis/releases).


# Usage

### To start

Simply run the executable file and follow the instructions as you go.

    1. Enter the path to the folder containing the text files to be analyzed.
    2. Enter the path to specific target phrases to be included in the analysis.

***Note that the tool take in a folder path as input, not a file path. The tool will then process all `TXT` files in the folder.***

The target phrases can be specified in a `JSON` file, following [this format](https://github.com/codynhanpham/word-frequency-analysis/blob/main/lookup-phrases.json):

```json
{
    "phrases": [
        "target phrase 1",
        "target phrase 2",
        "target phrase n"
    ]
}
```

Or you can simply leave the field blank to skip this step.

### Then

After just a bit, you will see the results in the `input\outputs` folder.

*This might change in the future when the visualization feature is added.*
    
        input (the folder you specified)
            ├── txtfile1.txt
            ├── volume2.txt
            ├── ...
            └───outputs
                input-file1_wordFreq.csv
                input-file1_wordFreq.txt
                ...
                (data_joined.csv)


If the folder contains multiple `TXT` files, the tool will generate an additional `data_joined.csv` file that contains the combined results of all the files.

### Finally

*(I am working on this...)*


# Contributing

Feel free to open an issue or submit a pull request if you have any ideas or suggestions.