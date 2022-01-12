#![warn(clippy::pedantic)]

use clap::Parser;
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use rand::rngs::ThreadRng;
use std::fs;
use std::io::{self, Write};

fn gen_word(
    prev: &mut [Option<String>],
    word_list: &[String],
    len: usize,
    count: usize,
    out: &mut impl Write,
    rng: &mut ThreadRng,
    dist: &WeightedIndex<usize>,
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
        word = word_list.get(dist.sample(rng)).unwrap().to_string();
    }
    write!(*out, "{} ", word).unwrap();
    prev[prev_len % count] = Some(word);
}

fn gen_words(
    count: Option<usize>,
    words: &[String],
    writer: &mut impl Write,
    rng: &mut ThreadRng,
    dist: &WeightedIndex<usize>,
) {
    let word_amt = words.len();
    let mut prev = vec![None; word_amt - 1];

    if let Some(count) = count {
        for i in 1..=count {
            gen_word(&mut prev, words, word_amt, i, writer, rng, dist);
        }
    } else {
        let mut count = 1;
        loop {
            gen_word(&mut prev, words, word_amt, count, writer, rng, dist);
            count += 1;
        }
    }
}

fn main() {
    let mut args = Args::parse();
    let words: Vec<String> = if let Some(ref a) = args.words {
        a.to_string()
    } else {
        "the giant cinnamon toast crunch".to_string()
    }
    .split(' ')
    .map(ToString::to_string)
    .collect();
    let dist = WeightedIndex::new(if args.words.is_some() {
        &vec![1; words.len()]
    } else {
        &vec![2, 1, 1, 1, 1]
    })
    .unwrap();
    let count = args.count;
    let mut rng = rand::thread_rng();
    if let Some(path) = args.output.as_mut() {
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .expect("Could not create or open a file from path supplied");
        let mut writer = io::BufWriter::new(file);
        gen_words(count, &words, &mut writer, &mut rng, &dist);
        writeln!(writer).unwrap();
    } else {
        let stdout = io::stdout();
        let mut writer = stdout.lock();
        gen_words(count, &words, &mut writer, &mut rng, &dist);
        writeln!(writer).unwrap();
    }
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Words to print. Should each be separated by a space inside a string.
    #[clap(short, long)]
    words: Option<String>,

    /// Amount of words to print out
    #[clap(short, long)]
    count: Option<usize>,

    /// Output file/stream. Default is standard output
    #[clap(short, long)]
    output: Option<std::path::PathBuf>,
}
