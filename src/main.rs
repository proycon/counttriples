use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::env;

fn main() -> io::Result<()> {
    if let Some(filename) = env::args().nth(1) {
        eprintln!("Reading {}",filename);
        let f = File::open(filename)?;
        let mut reader = BufReader::new(f);
        let mut metaline = String::new();
        reader.read_line(&mut metaline)?;
        let mut inputline = String::new();
        reader.read_line(&mut inputline)?;
        let ratio: u32 = metaline.split_ascii_whitespace().nth(1).map(|s| s.parse::<u32>().expect("Integer") ).expect("Ratio");
        let input: Vec<u32> = inputline.split_ascii_whitespace().map(|s| s.parse::<u32>().expect("Integer") ).collect();
        let count = counttriplets(input, ratio);
        println!("{}",count);
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound,"No input file specified"))
    }
}

struct Counter {
    total: u32,
    thusfar: u32,
}

impl Default for Counter {
    fn default() -> Counter {
        Counter {
            total: 0,
            thusfar: 0
        }
    }
}


fn counttriplets(input: Vec<u32>, ratio: u32) -> usize {
    let mut count: usize = 0;
    let n = input.len();

    eprintln!("Counting values...");

    //Instantiate the counter; data structure is ordered by key and allows for quick lookups
    let mut counter: BTreeMap<u32, Counter> = BTreeMap::new();

    //Fill the counter, we simply count how many times each value occurs
    for (i,value) in input.iter().enumerate() {
        let entry = counter.entry(*value).or_insert(Counter::default());
        entry.total += 1;
    }

    eprintln!("Counting triples...");

    //Loop through the input array, considering a focus item (the middle number of the triplets)
    for (i,value) in input.iter().enumerate() {

        //discard the first and last element as they can never be a focus item, and the focus can only be a valid focus if it can be divided by the ratio
        if i > 0 && i < n-1 && value % ratio == 0 {
            let leftvalue = value / ratio;
            if let Some(rightvalue) = value.checked_mul(ratio) { //checked multiplication to protect against integer overflow (if it overflow it won't be a valid triple anyhow)

                //(leftvalue,value,rightvalue) forms  a value triple of geometric progression with the given ratio
                //Check how many times we have seen leftvalue and how many times we have seen
                //rightvalue
                if let (Some(left), Some(right)) = (counter.get(&leftvalue),counter.get(&rightvalue)) {
                    count += left.thusfar as usize * (right.total - right.thusfar) as usize;
                    //eprintln!("@{} -> +{} (|{}|,|{}|)", focusindex,subcount, leftindices.len(), rightindices.len());
                }
            }

        }

        //count the current value
        if let Some(mut current) = counter.get_mut(&value) {
            current.thusfar += 1;
        }
    }
    count
}
