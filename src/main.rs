use clap::Parser;

fn gen_word(prev: &mut Vec<Option<String>>, word_list: &Vec<String>, len: usize, count: usize) {
    let mut word = word_list.get(rand::random::<usize>() % len).unwrap().to_string();
    let prev_len = prev.len() - 1;
    while prev[..] == word_list.iter().map(|w| Some(w.clone())).collect::<Vec<_>>()[0..(len-1)] 
        && prev.contains(&Some(word.clone())) {
        word = word_list.get(rand::random::<usize>() % len).unwrap().to_string();
    }
    print!("{} ", word);
    prev[prev_len % count] = Some(word);
}

fn gen_words(count: Option<usize>, words: Vec<String>) {
    let word_amt = words.len();
    let mut prev = vec![None; word_amt - 1];

    if let Some(count) = count {
        for i in 1..=count {
            gen_word(&mut prev, &words, word_amt, i);
        }
    } else {
        let mut count = 1;
        loop {
            gen_word(&mut prev, &words, word_amt, count);
            count += 1;
        }
    }
}

fn main() {
    let args = Args::parse();
    let words: Vec<String> = args.words.split(" ").map(|x| x.into()).collect();
    let count = args.count;
    gen_words(count, words);
    println!("");
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
}
