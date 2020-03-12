use std::fs;
use std::str;
use clap::*;
use crate::util::common::{
    *
};
use std::time::Instant;
use seq::{
    Fdb,
    Set,
    Save,
};
use lzt::{
    FFI,
    Drop
};

use std::str::FromStr;

use std::io::{self, prelude::*, stdout, Write, Read, BufReader, BufWriter};

pub fn export (cli: ArgMatches<'static>) -> bool {

    let before = Instant::now();

    let mut fdb = Fdb::new(cli.value_of("infmt").unwrap());

    if fdb.rm_file(cli.value_of("output").unwrap()) == false {
        panic!("cannot rm file ");
    }

    match cli.value_of("infmt") {
        Some(x) => {
            match x {
                "fasta" => {
                    //let mut seq_lzt = FFI::open(/*seq.sfq*/);

                },
                "fastq" => {

                    let mut head = String::new();
                    let mut qual = String::new();
                    let mut seq = String::new();

                    if let Some(x) = cli.value_of("input") {
                        head = format!("{}.{}",x,"head.sfq");
                        seq  = format!("{}.{}",x,"seq.sfq");
                        qual = format!("{}.{}",x,"qual.sfq");
                    }

                    let mut head_lzt = FFI::open(&head);
                    let mut seq_lzt  = FFI::open(&seq);
                    let mut qual_lzt = FFI::open(&qual);

                    let ( mut count, mut alpha, mut wlen, mut model) = (0,Vec::new(),0, false);

                    {

                        let head_stats  = get_stats(&head_lzt.get_records("~~~~~X"));
                        let seq_stats   = get_stats( &seq_lzt.get_records("~~~~~X"));
                        let qaual_stats = get_stats(&qual_lzt.get_records("~~~~~X"));

                        assert_eq!(seq_stats,qaual_stats);
                        assert_eq!(seq_stats,head_stats);

                        count = seq_stats.0;
                        alpha = seq_stats.1;
                        wlen  = seq_stats.2;
                        fdb.set_model(seq_stats.3);

                    }




                    if let Some(y) = cli.value_of("list") {
                        if fs::metadata(y).is_ok() == true{

                            let reader = fdb.clone().make_reader(y);

                            for line in reader.lines() {

                                let id = line.unwrap().parse::<usize>().unwrap();
                                if id > count {
                                    continue;
                                }

                                let prefix = encode(id, wlen, &alpha);
                                let enc = str::from_utf8(&prefix).unwrap();


                                {
                                    let mut seq_out = seq_lzt.get_records(&enc);
                                    let dis = deindex(&mut seq_out);
                                    fdb.set_seq(seq_out);
                                    fdb.set_cpcnt(dis);
                                }
                                {
                                    let mut head_out = head_lzt.get_records(&enc);
                                    let dis = deindex(&mut head_out);
                                    fdb.set_head(head_out);
                                }
                                {
                                    let mut qual_out = qual_lzt.get_records(&enc);
                                    let dis = deindex(&mut qual_out);
                                    fdb.set_qual(qual_out);
                                }

                                if let Some(x) = cli.value_of("cmode") {

                                    if x == "lossy"{
                                        fdb.expand();
                                    }
                                }else{
                                    panic!("Decompression compromised!");
                                }

                                fdb.save_append(cli.value_of("output").unwrap(),cli.value_of("outfmt").unwrap());

                                fdb.clear();

                            }
                        }
                    }else{
                        panic!("Record list not provided")
                    }
                    head_lzt.drop();
                    qual_lzt.drop();
                    seq_lzt.drop();


                },
                _=> {}
            }
        }
        None    => {
            panic!("Type not set");
        }
    }

    eprintln!(" {:.2?}", before.elapsed());

    true
}
