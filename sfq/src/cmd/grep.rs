use std::fs;
use std::str;
use clap::*;
use crate::util::common::*;
use std::time::Instant;
use std::path::Path;
use std::str::FromStr;
use std::ffi::OsStr;
use seq::{
    Fdb,
    Set,
    Save,
    Get
};
use lzt::{
    FFI,
    Drop
};


use std::io::{prelude::*};

pub fn export (cli: ArgMatches<'static>) -> bool {

    let before = Instant::now();

    let memmod : bool = if let Some(x) = cli.value_of("mem-mod") {
        if x == "R" {true}
        else {false}
    } else {true};

    let mut fdb = Fdb::new(cli.value_of("infmt").unwrap());

    // if keyword -o is defined, use that name,
    // otherwise send query output to stdout
    let output: &str = match cli.value_of("output") {
        Some(_x) => { cli.value_of("output").unwrap() }
        None => { "stdout" }
    };

    if let Some(y) = cli.value_of("input").unwrap().find(".L") {
        let input: &str = cli.value_of("input").unwrap();
        fdb.lossy = usize::from_str(input.get((y+2 as usize)..(y+3 as usize)).unwrap()).unwrap();
    }

    if fdb.rm_file(output) == false {
        panic!("cannot rm file ");
    }

    match cli.value_of("infmt") {
        Some(x) => {
            match x {
                "fastq" | "fasta" => {
                    let q = if x == "fastq" {true} else{false};

                    let mut head = String::new();
                    let mut qual = String::new();
                    let mut seq = String::new();

                    if let Some(x) = cli.value_of("input") {
                        let stem_name = String::from(Path::new(cli.value_of("input").unwrap())
                                            .file_stem().and_then(OsStr::to_str).unwrap());
                        head = format!("{}/{}.{}",x,stem_name,"head.sfastq");
                        seq  = format!("{}/{}.{}",x,stem_name,"seq.sfastq");
                        qual = format!("{}/{}.{}",x,stem_name,"qual.sfastq");
                    }

                    let mut seq_lzt  = FFI::open(&seq,memmod);
                    let mut qual_lzt = if q {FFI::open(&qual,memmod)}else{FFI::empty()};

                    let ( mut count, mut alpha, mut wlen) = (0,Vec::new(),0);

                    // get stats in the last multitrie
                    let seq_stats = get_stats( &seq_lzt.get_records("~~~~~^",&(seq_lzt.num_of_lzt as i32)));

                    count = seq_stats.0;
                    alpha = seq_stats.1.clone();
                    wlen  = seq_stats.2;
                    fdb.set_model(seq_stats.3);
                    if fdb.lossy != seq_stats.4 {
                        eprintln!("WARNING: lossy level written in Trie not reflected in naming convention.");
                        eprintln!("I will consider lossy level from Trie as correct");
                        fdb.lossy = seq_stats.4;
                    }

                    let mut head_lzt = if fdb.lossy > 1 {FFI::empty()} else {FFI::open(&head,memmod)};
                    let mut head_stats = (0 as usize, Vec::new(), 0 as usize, false, 0 as usize);
                    if fdb.lossy < 2 {
                        head_stats  = get_stats(&head_lzt.get_records("~~~~~^",&(head_lzt.num_of_lzt as i32)));
                        assert_eq!(seq_stats,head_stats);
                    }

                    let mut grep : Vec <usize> = Vec::new();

                    if let Some(y) = cli.value_of("list") {
                        if fs::metadata(y).is_ok() == true{
                            let reader = fdb.clone().make_reader(y);
                            for line in reader.lines() {
                                let uline = line.unwrap();
                                // ignore empty line
                                if uline.len() == 0 {continue;}
                                // if uline contains -, a range of records is given: handle it
                                if uline.contains("-") {
                                    let ids: Vec<&str> = uline.split("-").collect();
                                    assert!(ids.len() == 2);
                                    let start_id: usize =  ids[0].trim().parse::<usize>().unwrap();
                                    let end_id: usize = ids[1].trim().parse::<usize>().unwrap() + 1;
                                    for id in start_id..end_id {
                                        if id > count {continue;}
                                        grep.push(id);
                                    }
                                // otherwise, a single record is given
                                } else {
                                    let id = uline.parse::<usize>().unwrap();
                                    if id > count {continue;}
                                    grep.push(id);
                                }
                            }
                        } else {
                            let (f, v) = parse_conditional(y);
                            match &f[..] {
                                "rand" => {
                                    grep = make_rand_uvec(v.parse::<usize>().unwrap(), count);
                                },
                                _  => {
                                    panic!("{}() : Not recognized!", f);
                                }
                            }
                        }

                    } else {panic!("Record list not provided")}

                    for id in grep.iter(){

                        if *id < 1 {
                            eprintln!("You are searching for record nr. {:?}.", *id);
                            eprintln!("Values below 1 are not allowed, I am skipping this query...");
                            continue;
                        }
                        let prefix = encode(*id, wlen, &alpha);
                        let enc = str::from_utf8(&prefix).unwrap();
                        //let mut pos: i32 = ((*id as f32) / (num_lzt_rec as f32)).ceil() as i32;
                        // if calculated position if larger than the number of Tries in multitrie
                        // this is probably because fwd and rev are mismatched
                        // instead, let's search the entire multiTrie for that index
                        //if &head_lzt.num_of_lzt < &(pos as u8) {pos = -1;}

                        let mut cpcnt: Vec<usize> = Vec::new();
                        {
                            let mut seq_out = seq_lzt.get_records(&enc,&-1);
                            let dis = deindex(&mut seq_out);
                            if fdb.lossy > 2 { cpcnt = remove_cpcnt(&mut seq_out); }
                            let mut numcnt  = 0;
                            for p in seq_out.iter(){
                                if *p == 10u8 {numcnt += 1;}
                            }
                            // number of records extracted
                            fdb.set_numrec(numcnt);
                            fdb.set_seq(seq_out);
                            if fdb.lossy > 2 { fdb.set_cpcnt(cpcnt.clone()); }
                            else { fdb.set_cpcnt(dis); }
                        }
                        {
                            if fdb.lossy < 2 {
                                let mut head_out = head_lzt.get_records(&enc,&-1);
                                let _dis = deindex(&mut head_out);
                                fdb.set_head(head_out);
                            } else {
                                let mut head_out = head_lzt.generate_header(*id,*id,fdb.paired,cpcnt); 
                                fdb.set_head(head_out);
                            }
                        }
                        if q {
                            let mut qual_out = qual_lzt.get_records(&enc,&-1);
                            let _dis = deindex(&mut qual_out);
                            fdb.set_qual(qual_out);
                        }else{
                            let qvec = vec!['\n' as u8; fdb.get_numrec()];
                            fdb.set_qual(qvec);
                        }

                        fdb.save_append(output,cli.value_of("outfmt").unwrap());
                        fdb.clear();
                    }
                    head_lzt.drop();
                    qual_lzt.drop();
                    seq_lzt.drop();
                },
                _ => {
                    panic!("File format {} not recognized",x)
                }
            }
        }
        None    => {
            panic!("File format not set");
        }
    }

    eprintln!(" {:.2?}", before.elapsed());

    true
}
