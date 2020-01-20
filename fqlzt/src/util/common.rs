use fxhash::FxHashMap;
use std::io::{self, prelude::*, stdout, Write, Read, BufReader, BufWriter};
use std::fs::File;


// Trimm new line characer -> there is probably a better way to do this
pub fn trim_nl(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}


pub fn make_reader(file: &str)-> BufReader<Box< dyn Read >> {

    let tmp : Box<dyn Read> = match file {
        "stdin" => {
            Box::new(io::stdin())
        },
        _       => {
            Box::new(File::open(file)
                .expect(&(format!("Error opening {} file",file))))
        }
    };
    BufReader::new(tmp)
}


pub fn make_writer (file: &str)-> BufWriter<Box<dyn Write>> {

    let tmp : Box<dyn Write> = match file {
        "stdout" => {
            Box::new(io::stdout())
        },
        _       => {
            Box::new(File::create(file)
                .expect(&(format!("Error opening {} file",file))))
        }
    };
    BufWriter::new(tmp)
}




pub fn make_sort_idx(hash: &mut FxHashMap<usize, Vec<usize>>, vec: &mut Vec<String>)-> bool {

    vec.sort();

    let mut seq : String = String::new();
    let mut count  = 0;
    let mut sort   = 0;
    let mut usort  = 0;
    let mut hlen   = 0;
    let     last   = vec.last().unwrap().clone();

    for i in vec.into_iter() {
        let st = i.clone();
        let v = st.split("\t").collect::<Vec<&str>>();
        if seq != v[0]  && seq.len() > 0{
            hash.insert(usort,vec![count,sort]);
            // one for head seq binding and one for \n
            sort = sort + seq.len()  + hlen + 3 + count.to_string().len() ;
            count=0;
        }

        seq   = v[0].clone().to_owned();
        hlen  = v[1].parse::<usize>().unwrap();
        usort = v[2].parse::<usize>().unwrap();
        count = count+1;

        if st == *last {
            hash.insert(usort,vec![count,sort]);
        }
    }

    true
}
