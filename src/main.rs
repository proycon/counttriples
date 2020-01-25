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


fn counttriplets(input: Vec<u32>, ratio: u32) -> u64 {
    let mut count: u64 = 0;
    let n = input.len();

    eprintln!("Building index...");

    //Instantiate the index; data structure is ordered by key and allows for quick lookups
    let mut index: BTreeMap<u32, Vec<u32>> = BTreeMap::new();

    //Fill the index
    for (i,value) in input.iter().enumerate() {
        let entry = index.entry(*value).or_insert(Vec::new());
        let i: u32 = i.try_into().expect("Downcasting usize to u32");
        entry.push(i);
    }

    eprintln!("Counting...");

    //Loop through the input array, considering a focus item (the middle number of the triplets)
    for (i,value) in input.iter().enumerate() {
        //discard the first and last element as they can never be a focus item, and the focus can only be a valid focus if it can be divided by the ratio
        if i > 0 && i < n-1 && value % ratio == 0 {
            let leftvalue = value / ratio;
            let rightvalue = value.checked_mul(ratio);

            if rightvalue == None {
                //multiplication overflow, focus can't be valid
                continue;
            }
            let rightvalue = rightvalue.unwrap();

            //(leftvalue,value,rightvalue) forms  a value triple of geometric progression with the given ratio
            //now we check if this pair actually exists in our input, by looking it up in the
            //index

            let leftindices = index.get(&leftvalue);
            let rightindices = index.get(&rightvalue);
            if leftindices.is_none() || rightindices.is_none() {
                //one of the values doesn't exist, so discard this focus item and carry on with the
                //next one
                continue
            }

            //Now all left indices, focus index (i), and right indexes are valid combinations as
            //long as left < focus < right
            let focusindex: u32 = i.try_into().expect("Downcasting to u32");
            for leftindex in leftindices.unwrap().iter() {
                for rightindex in rightindices.unwrap().iter() {
                    if *leftindex < focusindex && focusindex < *rightindex {
                        count += 1;
                        println!("[ {},{},{} ] @ ( {}, {}, {} )", leftvalue, value, rightvalue, leftindex, i, rightindex)
                    }
                }
            }

        }
    }
    count
}
