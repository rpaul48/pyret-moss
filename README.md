# Pyret Moss
Pyret Moss is a command line application for determining the similarity of programs written in [Pyret](https://www.pyret.org/). It was inspired by the "Measure of Software Similarity" system developed at Stanford, and its core ideas can be found in [this paper](http://theory.stanford.edu/~aiken/publications/papers/sigmod03.pdf).

Please note that while the primary intended use case for this program is to help detect plagiarism across homework assignment submissions written in Pyret, **proof of plagiarism may not be guaranteed solely from the similarity scores returned in the output**. Instructors should manually check pairs of submissions with high similarity scores before determining the presence of plagiarism. The responsibility of Moss is to present the **relative similarity** between pairs of submissions, leaving the decision of what constitutes plagiarism to the instructor. More information about understanding output can be found in the corresponding section below.

## How to Build and Run
### Building for Development
If you'd like to work on `pyret-moss` for yourself, you can clone this repository and compile/run using `cargo run` from the root directory of the project. Command-line arguments can be passed after a `--` (so they won't be passed to cargo itself), e.g. `cargo run -- --help`.

You can run the test suite with `cargo test`.

### Building for Production
To build the project for use in a production setting, run `cargo build --release`. You can now run the compiled binary using `./target/release/pyret-moss` from the project directory.

#### Adding to Path
If you'd like to be able to access the Pyret Moss executable from anywhere on your system, you can add it to a directory that is part of your $PATH. Here is an example (for Unix systems) using the `~/.local/bin` directory: 

1. Copy the executable to `~/.local/bin` by running `cp ./target/release/pyret-moss ~/.local/bin/pyret-moss` from the project root.
2. Add `~/.local/bin` to your $PATH:
    - `nano ~/.bashrc`. This should open your `.bashrc` for editing. 
    - At the bottom of `~/.bashrc`, add `export PATH=${HOME}/.local/bin:$PATH`
3. Restart your terminal, and the `pyret-moss` command should be accessible from anywhere.

### Running
- [explain cli args]

Please see the sections below for information on k, t, matching, and understanding output. Unless indicated otherwise through the flags, the program will run with the following **default configuration**:

- k = 15
- t = 20
- match threshold = 0
- \<SUBMISSIONS-DIR> is expected to contain subdirectories of .arr files such that each subdirectory represents a "submission".

## Determining Similarity
The process consists of four main components.

### Normalization
As the submissions in the input directory are being read, all .arr files within each submission are first normalized to remove features from a program's text which should not differentiate it from other programs. From each original file, a normalized text is generated such that:

1. identifiers are normalized
2. type annotations are removed
3. whitespace is removed
4. docstrings are removed
5. comments are removed

During this process, the original line number of each character in the normalized text is stored such that it may later be accessed to line numbers with significant overlap.

### Fingerprinting
Each normalized text is then fingerprinted, which involves determining a set of hashed substrings (fingerprints) which represent that particular text. Given a normalized text and values of k (noise threshold) and t (guarantee threshold):

1. the text is converted into a sequence of "k-grams", or contiguous substrings of length k
2. each k-gram is converted into an integer using a rolling hash function
3. windows of hashes of length t - k + 1 are generated
4. the robust winnowing algorithm is used to select a set of hashes from the set of windows, and these hashes are the text's fingerprints

So what are k and t?
- k: the noise threshold, substring matches across normalized texts shorter than k are not considered
- t: the guarantee threshold, substring matches across normalized texts of length t or greater are guaranteed to be caught

Both k and t must be positive, and 0 < k <= 1.

### Matchmaking
Once all submissions in \<SUBMISSIONS-DIR> have been fingerprinted, submissions with shared fingerprints are paired together and their set of shared hashes is stored. If a pair of submissions has a match percentile greater than the "match threshold" value, it will be included in the output. Note that a pair's "match percentile" is calculated as the quotient of its number of shared hashes and the maximum number of shared hashes between any pairs of submissions.

### Consolidation
[include description once completed]

## Understanding Output
[insert screenshots]
