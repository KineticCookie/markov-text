extern crate rand;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

use rand::{Rng, ThreadRng};

struct MarkovGenerator<T> {
    map: HashMap<T, HashMap<T, u32>>,
    count: u32,
    rng: ThreadRng
}

impl<T:Eq+std::hash::Hash+Clone> MarkovGenerator<T> {
    fn new() -> Self {
        MarkovGenerator{
            map: HashMap::new(),
            count: 0,
            rng: rand::thread_rng()
        }
    }

    fn process_all(&mut self, input: &Vec<T>) -> Result<(),()> {
        let mut prev_item: Option<&T> = None;
        for item in input {
            if prev_item.is_some() {
                let mut submap = self.map.entry(prev_item.unwrap().clone()).or_insert(HashMap::new());
                let mut prob = submap.entry(item.clone()).or_insert(0);
                *prob += 1;
                self.count += 1;

            }
            prev_item = Some(&item);
        }
        self.map.insert(prev_item.unwrap().clone(), HashMap::new());
        Ok(())
    }

    fn get_random(&mut self) -> T {
        let keys = &self.map.keys().collect::<Vec<&T>>();
        (*self.rng.choose(keys).unwrap()).clone()
    }

    fn get(&mut self, key: &T) -> Result<T, ()> {
        match self.map.get(key) {
            Some(submap) => {
                loop {
                    let random = self.rng.gen_range(1u32, self.count);
                    for (k, v) in submap.iter() {
                        if *v >= random {
                            let r:T = k.clone();
                            return Ok(r);
                        }
                    }
                }
            },
            None => Err(())
        }
    }
}

impl<T: fmt::Display+Eq+std::hash::Hash+Clone> fmt::Display for MarkovGenerator<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str("MarkovGenerator: {\n");
        for (k,v) in &self.map {
            result.push_str(&format!("  \"{}\" => {{\n", &k));
            for (kk, vv) in v {
                result.push_str(&format!("    \"{}\" : {},\n", &kk, &vv));
            }
            result.push_str("  },\n");
        }
        result.push_str("}");
        write!(f, "{}", result)
    }
}

fn main() {
    let mut f = File::open("foo.txt").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);

    let mut markov_net = MarkovGenerator::new();
    let _ = markov_net.process_all(&s.split(|c| c==' ' || c=='\n').collect());
    println!("{}", &markov_net);

    let gen_size = 200;
    let mut count = 0;
    let mut word = markov_net.get_random();
    let mut gen_text = String::new();
    while count < gen_size {
        gen_text.push_str(word);
        gen_text.push(' ');
        word = markov_net.get(&word).unwrap();
        count += 1;
    }
    println!("{}", gen_text);
}
