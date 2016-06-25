extern crate rustc_serialize;
extern crate docopt;

mod markov_generator;

use std::fs::File;
use std::io::prelude::*;
use docopt::*;
use markov_generator::MarkovGenerator;

const USAGE: &'static str = "
Random text generator.

Usage:
  markov-text --input <INPUT> --size <SIZE> --output <OUTPUT>
  markov-text (-h | --help)
  markov-text --version

Options:
  -h --help                Show this screen.
  --version                Show version.
  --input <INPUT>          Path to the file with full text.
  --output <OUTPUT>        File with result of generation.
  --size <SIZE>            Size of the generated text in sentences.
";

#[derive(RustcDecodable)]
struct Args {
    flag_version: bool,
    flag_input: Option<String>,
    flag_output: Option<String>,
    flag_size: Option<u32>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("markov-text v0.1.0");
        return;
    }
    let input_path = args.flag_input.unwrap_or_else(|| panic!("Input file is not specified!"));
    let output_path = args.flag_output.unwrap_or_else(|| panic!("Output file is not specified!"));
    let gen_size = args.flag_size.unwrap_or_else(|| panic!("Text size is not specified!"));

    let mut f = File::open(input_path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let mut markov_net = MarkovGenerator::new();
    let _ = markov_net.process_all(&s.split(|c| c == ' ' || c == '\n').collect());

    let mut count = 0;
    let mut word = markov_net.get_random();
    let mut gen_text = String::new();
    while count < gen_size {
        gen_text.push_str(word);
        gen_text.push(' ');
        word = markov_net.get(&word).unwrap();
        count += 1;
    }

    f = File::create(output_path).unwrap();
    f.write_all(gen_text.as_bytes()).unwrap();
}
