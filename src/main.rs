#![warn(clippy::pedantic)]

use clap::Parser;
use std::fs;
use std::io::{self, Write};

fn gen_word(
    prev: &mut [Option<String>],
    word_list: &[String],
    len: usize,
    count: usize,
    out: &mut impl Write,
) {
    let mut word = word_list
        .get(rand::random::<usize>() % len)
        .unwrap()
        .to_string();
    let prev_len = prev.len() - 1;
    while prev[..]
        == word_list
            .iter()
            .map(|w| Some(w.clone()))
            .collect::<Vec<_>>()[0..(len - 1)]
        && prev.contains(&Some(word.clone()))
    {
        word = word_list
            .get(rand::random::<usize>() % len)
            .unwrap()
            .to_string();
    }
    write!(*out, "{} ", word).unwrap();
    prev[prev_len % count] = Some(word);
}

fn gen_words(count: Option<usize>, words: &[String], writer: &mut impl Write) {
    let word_amt = words.len();
    let mut prev = vec![None; word_amt - 1];

    if let Some(count) = count {
        for i in 1..=count {
            gen_word(&mut prev, words, word_amt, i, writer);
        }
    } else {
        let mut count = 1;
        loop {
            gen_word(&mut prev, words, word_amt, count, writer);
            count += 1;
        }
    }
}

fn main() {
    let mut args = Args::parse();
    let words: Vec<String> = args.words.split(' ').map(ToString::to_string).collect();
    let count = args.count;
    if let Some(path) = args.output.as_mut() {
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .expect("Could not create or open a file from path supplied");
        let mut writer = io::BufWriter::new(file);
        gen_words(count, &words, &mut writer);
        writeln!(writer).unwrap();
    } else {
        let stdout = io::stdout();
        let mut writer = stdout.lock();
        gen_words(count, &words, &mut writer);
        writeln!(writer).unwrap();
    }
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Words to print. Should each be separated by a space inside a string.
    #[clap(short, long, default_value_t = String::from("the the giant cinnamon toast crunch"))]
    words: String,

    /// Amount of words to print out
    #[clap(short, long)]
    count: Option<usize>,

    /// Output file/stream. Default is standard output
    #[clap(short, long)]
    output: Option<std::path::PathBuf>,
}
