extern crate rand;

use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use self::rand::{Rng, ThreadRng};

pub struct MarkovGenerator<T> {
    map: HashMap<T, HashMap<T, u32>>,
    count: u32,
    rng: ThreadRng,
}

impl<T: Eq + Hash + Clone> MarkovGenerator<T> {
    pub fn new() -> Self {
        MarkovGenerator {
            map: HashMap::new(),
            count: 0,
            rng: rand::thread_rng(),
        }
    }

    pub fn process_all(&mut self, input: &Vec<T>) -> Result<(), ()> {
        let mut prev_item: Option<&T> = None;
        for item in input {
            if prev_item.is_some() {
                let mut submap =
                    self.map.entry(prev_item.unwrap().clone()).or_insert(HashMap::new());
                let mut prob = submap.entry(item.clone()).or_insert(0);
                *prob += 1;
                self.count += 1;

            }
            prev_item = Some(&item);
        }
        self.map.insert(prev_item.unwrap().clone(), HashMap::new());
        Ok(())
    }

    pub fn get_random(&mut self) -> T {
        let keys = &self.map.keys().collect::<Vec<&T>>();
        (*self.rng.choose(keys).unwrap()).clone()
    }

    pub fn get(&mut self, key: &T) -> Result<T, ()> {
        match self.map.get(key) {
            Some(submap) => {
                loop {
                    let random = self.rng.gen_range(1u32, self.count);
                    for (k, v) in submap.iter() {
                        if *v >= random {
                            let r: T = k.clone();
                            return Ok(r);
                        }
                    }
                }
            }
            None => Err(()),
        }
    }
}
impl<T: fmt::Display + Eq + Hash + Clone> fmt::Display for MarkovGenerator<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str("MarkovGenerator: {\n");
        for (k, v) in &self.map {
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
