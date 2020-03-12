use std::str;
use clap::*;
use crate::util::common::{
    *
};
use std::time::Instant;
use seq::{
    Fdb,
    Save,
    Get,
    Set
};
use lzt::{
    FFI,
    Drop
};




pub fn extract(cli: ArgMatches<'static>) -> bool {
    eprint!("Decompressing...");
    let before = Instant::now();


    let mut fdb = Fdb::new(cli.value_of("infmt").unwrap());

    //let (wlen,alpha) = parse_codex(cli.value_of("codex").unwrap());

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


                    // get info :alloc
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

                    let (mut i, mut j) = (0,3);

                    while i < count {

                        let enc_start = encode(i, wlen, &alpha);
                        let enc_stop  = encode(j, wlen, &alpha);

                        j+=alpha.len();
                        i+=alpha.len();

                        if j> count {j=count;}

                        let e = enc_start.iter().zip(enc_stop.iter()).filter(|&(a, b)| a == b).count();

                        let prefix = enc_start[..e].to_vec();
                        let enc = str::from_utf8(&prefix).unwrap();

                        {
                            let mut seq_out: Vec<u8> = seq_lzt.get_records(&enc);
                            let dis = deindex(&mut seq_out);
                            fdb.set_seq(seq_out);
                            fdb.set_cpcnt(dis);
                        }
                        //println!("S{:?}", fdb.get_seq());
                        {
                            let mut head_out = head_lzt.get_records(&enc);
                            let dis = deindex(&mut head_out);
                            fdb.set_head(head_out);
                        }
                        //println!("H{:?}", fdb.get_head());
                        {
                            let mut qual_out = qual_lzt.get_records(&enc);
                            let dis = deindex(&mut qual_out);
                            fdb.set_qual(qual_out);
                        }
                        //println!("Q{:?}", fdb.get_qual());
                        if let Some(x) = cli.value_of("cmode") {

                            if x == "lossy"{
                                fdb.expand();
                            }
                        }else{
                            panic!("Decompression compromised!");
                        }

                        fdb.save_append(cli.value_of("output").unwrap(), cli.value_of("outfmt").unwrap());

                        fdb.clear();
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
