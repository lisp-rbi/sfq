use std::fs;
use std::str;
use clap::*;
use crate::util::common::*;
use std::time::Instant;
use std::path::Path;
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
        if x == "R" {
            true
        }else{
            false
        }
    }else{
        true
    };

    let mut fdb = Fdb::new(cli.value_of("infmt").unwrap());

    let mut output: &str;
    // if keyword -o is defined, use that name,
    // otherwise send result to stdout
    match cli.value_of("output") {
        Some(x) => { output = cli.value_of("output").unwrap(); }
        None => { output = "stdout"; }
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
                        let mut stem_name = String::from(Path::new(cli.value_of("input").unwrap()).file_stem().and_then(OsStr::to_str).unwrap());
                        head = format!("{}/{}.{}",x,stem_name,"head.sfq");
                        seq  = format!("{}/{}.{}",x,stem_name,"seq.sfq");
                        qual = format!("{}/{}.{}",x,stem_name,"qual.sfq");
                    }

                    let mut head_lzt = FFI::open(&head,memmod);
                    let mut seq_lzt  = FFI::open(&seq,memmod);
                    let mut qual_lzt = if q {FFI::open(&qual,memmod)}else{FFI::empty()};

                    let ( mut count, mut alpha, mut wlen, mut model) = (0,Vec::new(),0, false);

                    {

                        let head_stats  = get_stats(&head_lzt.get_records("~~~~~^"));
                        let seq_stats   = get_stats( &seq_lzt.get_records("~~~~~^"));

                        assert_eq!(seq_stats,head_stats);

                        count = seq_stats.0;
                        alpha = seq_stats.1;
                        wlen  = seq_stats.2;

                        fdb.set_model(seq_stats.3);

                    }

                    let mut grep : Vec <usize> = Vec::new();

                    if let Some(y) = cli.value_of("list") {
                        if fs::metadata(y).is_ok() == true{

                            let reader = fdb.clone().make_reader(y);

                            for line in reader.lines() {

                                let id = line.unwrap().parse::<usize>().unwrap();

                                if id > count {
                                    continue;
                                }
                                grep.push(id);

                            }
                        }else{
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

                    }else{
                        panic!("Record list not provided")
                    }



                    for id in grep.iter(){

                        let prefix = encode(*id, wlen, &alpha);
                        let enc = str::from_utf8(&prefix).unwrap();


                        {
                            let mut seq_out = seq_lzt.get_records(&enc);
                            let dis = deindex(&mut seq_out);
                            let mut numcnt  = 0;
                            for p in seq_out.iter(){
                                if *p == 10u8 {
                                    numcnt+=1;
                                }
                            }
                            fdb.set_numrec(numcnt);
                            fdb.set_seq(seq_out);
                            fdb.set_cpcnt(dis);
                        }
                        {
                            let mut head_out = head_lzt.get_records(&enc);
                            let dis = deindex(&mut head_out);
                            fdb.set_head(head_out);
                        }
                        if q {
                            let mut qual_out = qual_lzt.get_records(&enc);
                            let dis = deindex(&mut qual_out);
                            fdb.set_qual(qual_out);

                            if let Some(y) = cli.value_of("cmode") {

                                if y == "lossy"{
                                    fdb.expand();
                                }
                            }else{
                                panic!("Decompression compromised!");
                            }
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
                _=> {
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
