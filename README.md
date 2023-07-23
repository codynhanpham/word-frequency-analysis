# Word Frequency Analysis
A simple tool to quickly analyze and visualize the frequency of words in text files.

### Features Checklist
- [x] Batch process all `TXT` files in a folder
- [x] Normalization of capitalized words
- [x] Remove [stop words](#stop-words)
- [x] Specify any additional [target phrases](#the-target-phrases-can-be-specified-in-a-json-file-following-this-format) to include
- [ ] Visualizations (work in progress)

### Analyses
- [x] Word frequency
- [x] Term frequency - inverse document frequency (TF-IDF) [see more below](#term-frequency---inverse-document-frequency-tf-idf)
- [ ] Sentiment analysis (work in progress)

</br>

# Installation
You can either build the tool from source or download the latest release from the [releases page](https://github.com/codynhanpham/word-frequency-analysis/releases).

### Build from Source
This is a Rust project, so you probably know what you are doing XD

### Download the Latest Release
The pre-built binary is only available for Windows at the moment. You can check out the [releases page](https://github.com/codynhanpham/word-frequency-analysis/releases).

</br>

# Usage

## To start
Simply run the executable file and follow the instructions as you go.

    1. Enter the path to the folder containing the text files to be analyzed.
    2. Enter the path to a "settings.json" file.

***Note that for `1.`, the tool take in a folder path as input, not a file path. The tool will then process all `TXT` files in the folder.***

---
#### The target phrases can be specified in a `JSON` file, following [this format](https://github.com/codynhanpham/word-frequency-analysis/blob/main/settings.json):

```json
{
    "phrases": [
        "target phrase 1",
        "target phrase 2",
        "target phrase n"
    ]
}
```
Think of `target phrases` as a **case-sensitive** `Ctrl/Command + F` search. Instead of counting the number of occurrences after [word splitting](#word-splitting), the tool will do the search for the phrases in the raw text. Similar to how `Ctrl/Command + F` works, if a phrase such as

**oxide is a chemical compound that**

was entered, then

>*mon***oxide is a chemical compound that** *or*
>
>*di***oxide is a chemical compound that**

will also be counted.

---

Of course, you don't have to name the config file `setttings.json`. Any `JSON` file will do. You can also specify a "chapter" separator in this `.json` file to split each input file into multiple documents for TF-IDF analysis. [see below](#term-frequency---inverse-document-frequency-tf-idf)

Or you can simply leave the field blank to skip this step.

## Then
After just a bit, you will see the results in the `input\outputs` folder.

*This might change in the future when the visualization feature is added.*
    
        input/ (the folder you specified)
            ├── txtfile1.txt
            ├── volume2.txt
            ├── ...
            └── outputs/
                ├── input_tf-idf-no-stopwords.csv
                ├── input_wordFreq.csv
                └── input_wordFreq_no-stopwords.csv


The output is a combined `CSV` file containing the frequency of all words in all input files.

## Finally

*(I am working on this...)*

</br>

# Analyses in Detail
*Just a bit more details on the analyses performed by the tool.*

## Word splitting
By default, the tool will split the words by any non-alphanumeric character, with the exception of the `-`, `*`, and `’` characters. So words such as `TF-IDF` will be kept as a single word. Note this does not include `'`, so `don't` will be split into `don` and `t`.

Most novels uses `’` in place of `'`, though! So `don’t` will be kept as a single word.

In the future, you will have the option to specify a list of characters to be excluded from the splitting process in the `settings.json` file. *Coming soon...*

## Sort order
Think of your data as 3 different "buckets" of words: `target phrases`, `capitalized words`, and `normal/lowercase words`.

First is the order of these "buckets" in the results:

1. If you have entered any [target phrases](#the-target-phrases-can-be-specified-in-a-json-file-following-this-format), they will be bring to the top of the results.
2. Then, the capitalized words will be listed.
3. Finally, the normal/lowercase words will be listed.

Within each "bucket", the words are sorted in descending order of their total ***corpus*** value (frequency/TF-IDF/...); in other words, decending order of the last column. The words with the same value are sorted in alphabetical order.


## Stop words
The list of stop words can be view here: [stopwords.txt](https://github.com/codynhanpham/word-frequency-analysis/blob/main/src/utils/stopwords.txt). It is originally taken from Kaggle [here](https://www.kaggle.com/datasets/rowhitswami/stopwords). Thanks, [Ragnar](https://www.kaggle.com/rowhitswami)!

The words from the stop words list are removed from the input files either before or after any analysis is performed depending on the analysis. Most of the time, the stop words are removed before the analysis, as it affects the weight or value of the result (TF-IDF analysis, for example). However, in some cases, it is just more convenient/efficient to remove stop words after the analysis as it does not affect the results (word frequency analysis, for example).

## Word Frequency
Generic word frequency analysis by counting the number of occurrences of each word in the input files. The analysis is done *with* and *without* [stop words](#stop-words).

The results are then combined to a single `CSV` file in this format:
```csv
Words,filename1,filename2,filename3,Total
word B,4,5,6,15
word A,1,2,3,6
```
| Words | filename1 | filename2 | filename3 | Total |
|-------|-----------|-----------|-----------|-------|
| word B | 4 | 5 | 6 | 15 |
| word A | 1 | 2 | 3 | 6 |

## Term Frequency - Inverse Document Frequency (TF-IDF)
TF-IDF is a numerical statistic that is intended to reflect how important a word is to a document in a collection or corpus. It is often used as a weighting factor in information retrieval and text mining. [see Wikipedia](https://en.wikipedia.org/wiki/Tf%E2%80%93idf)

By default, a "document" in the context of this tool is a single input `.txt` file. However, you can specify a "chapter" separator in a `settings.json` file to split each input file into multiple documents. The default separator is `<|eoc|>`.

*For example, if your files are formatted so that each chapter or section are separated by 5 newlines (`\r\n` character), you can specify the separator as follow:*

```json
{
    "chapter_separator": "\r\n\r\n\r\n\r\n\r\n",
    "phrases": []
}
```

### TF-IDF Calculation
**TF-IDF is calculated _after_ the stop words have been removed.**

The formular for TF-IDF used in this tool is as follow:

> **TF** = (Number of times a term appears in a document) / (Total number of terms in the document)
>
>**IDF** = log<sub>10</sub>[(Total number of documents) / (Number of documents with the term in it)]
>
>**TF-IDF** = TF * IDF

Note that there is ***no*** smoothing applied to the IDF calculation. In the case where a term does not appear in any document, the denominator of IDF will be forced to be 1 to avoid division by zero, and a warning will be printed to the CLI.

### TF-IDF Results
TF-IDF results format can be a bit different depending on whether there are multiple documents in each input file or not. It is simply for the sake of readability.

**1. If there are "chapters" (or separated documents) in any of the input file, the results are combined to a single `CSV` file in this format:**
```csv
Words,filename1 _ #1,filename1 Total,filename2 _ #1,filename2 _ #2,filename2 Total,Corpus Total
word A,0.020,0.020,0.010,0.020,0.030,0.050
```
| Words | filename1 _ #1 | filename1 Total | filename2 _ #1 | filename2 _ #2 | filename2 Total | Corpus Total |
|-------|-----------|-----------------|-----------|-----------|-----------------|--------------|
| word A | 0.020 | 0.020 | 0.010 | 0.020 | 0.030 | 0.050 |

</br>

**2. If there is only one document in each input file, the results are a bit simpler. They are combined to a single `CSV` file in this format:**
```csv
Words,filename1,filename2,filename3,Corpus Total
word A,0.020,0.010,0.030,0.060
```
| Words | filename1 | filename2 | filename3 | Corpus Total |
|-------|-----------|-----------|-----------|--------------|
| word A | 0.020 | 0.010 | 0.030 | 0.060 |

### *Why TF-IDF was not calculated?*
The purpose of TF-IDF is to "reflect how important a word is to a document in a collection or corpus." Which mean that, **if there is only a single document** (a single file without any "chapter") in the input folder, **the TF-IDF analysis will be skipped** as it is quite meaningless.


# Contributing
There is always a chance that bugs and things might occur. Please feel free to open an issue or submit a pull request if you have any ideas or suggestions.