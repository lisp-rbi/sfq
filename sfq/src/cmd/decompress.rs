use std::str;
use std::str::FromStr;
use clap::*;
use crate::util::common::*;
use std::time::Instant;
use std::path::Path;
use std::ffi::OsStr;
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
    eprintln!("Decompressing...");
    let before = Instant::now();


    let mut fdb = Fdb::new(cli.value_of("infmt").unwrap());

    let memmod : bool = if let Some(x) = cli.value_of("mem-mod") {
        if x == "R" {true}
        else {false}
    } else {true};

    //let (wlen,alpha) = parse_codex(cli.value_of("codex").unwrap());

    // if keyword -o is defined, use that name,
    // otherwise send result to stdout
    let output: &str = match cli.value_of("output") {
        Some(_x) => { cli.value_of("output").unwrap() }
        None => { "stdout" }
    };

    // exponent of the alphabet length; input file is read in batches
    // of records len(alphabet)^exponent
    let exponent: usize = match cli.value_of("decompress-exponent") {
        Some(x) => {usize::from_str(x).unwrap()}
        None => {6}
    };

    if let Some(y) = cli.value_of("input").unwrap().find(".L") {
        let input: &str = cli.value_of("input").unwrap();
        fdb.lossy = usize::from_str(input.get((y+2 as usize)..(y+3 as usize)).unwrap()).unwrap();
    }

    if fdb.rm_file(output) == false {panic!("cannot rm file ");}

    match cli.value_of("infmt") {

        Some(x) => {
            match x {
                "fasta" | "fastq" => {

                    let q = if x == "fastq" {true} else{false};

                    let mut head = String::new();
                    let mut seq = String::new();
                    let mut qual = String::new();

                    if let Some(x) = cli.value_of("input") {
                        let stem_name = String::from(Path::new(cli.value_of("input").unwrap()).file_stem().and_then(OsStr::to_str).unwrap());
                        head = format!("{}/{}.{}",x,stem_name,"head.sfastq");
                        seq  = format!("{}/{}.{}",x,stem_name,"seq.sfastq");
                        qual = format!("{}/{}.{}",x,stem_name,"qual.sfastq");
                    }

                    let ( mut count, mut alpha, mut wlen) = (0,Vec::new(),0);

                    let mut seq_lzt  = FFI::open(&seq,memmod);
                    let mut qual_lzt = if q {FFI::open(&qual,memmod)} else {FFI::empty()};

                    let seq_stats = get_stats( &seq_lzt.get_records("~~~~~^",&-1));

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
                        head_stats = get_stats(&head_lzt.get_records("~~~~~^",&-1));
                        assert_eq!(seq_stats,head_stats);
                    }

                    let pow : usize = if wlen <= exponent {(wlen as usize)-1} else {exponent};
                    let inc = alpha.len().pow(pow as u32); // set to 5th iteration

                    //let (mut i, mut j, mut pp) = (0,inc-1, 0);
                    let (mut i, mut j) = (0,inc-1);

                    while i < count {

                        let enc_start = encode(i, wlen, &alpha);
                        let enc_stop  = encode(j, wlen, &alpha);

                        let mut e = 0;

                        // see at which digit start and stop when prefixes begin to differ
                        for k in 0..enc_start.len() {
                            if enc_start[k] == enc_stop[k] {e+=1;} 
                            else {break;}
                        }

                        if j > count {j = count;}

                        // take start prefix up to the point start and stop are equal
                        let prefix = enc_start[..e].to_vec();
                        let enc = str::from_utf8(&prefix).unwrap();

                        let mut cpcnt: Vec<usize> = Vec::new();
                        {
                            //eprint!("Seq ... ");
                            //let st = Instant::now();
                            let mut seq_out: Vec<u8> = seq_lzt.get_records(&enc,&-1);
                            //let ms: u64 = (st.elapsed().as_millis() +1) as u64;
                            //eprintln!("LZT  {:?}", String::from_utf8(seq_out.clone()).unwrap());

                            let dis = deindex(&mut seq_out);
                            if fdb.lossy > 2 {cpcnt = remove_cpcnt(&mut seq_out);}
                            //eprintln!("LZT  {:?} \ndis:{:?}", String::from_utf8(seq_out.clone()).unwrap(),dis);
                            /*let mut sum: u64 = 0;
                            for nj in 0..cpcnt.len() {
                                sum += cpcnt[nj] as u64;
                            }*/

                            let mut numcnt  = 0;
                            for p in seq_out.iter(){
                                if *p == 10u8 {
                                    numcnt+=1;
                                }
                            }
                            //pp=numcnt.clone();
                            //eprintln!("Rec/sec: {:.2?}", (((pp) as u64)/ms) * 1000);
                            fdb.set_numrec(numcnt);
                            fdb.set_seq(seq_out);
                            if fdb.lossy > 2 { fdb.set_cpcnt(cpcnt.clone()); }
                            else { fdb.set_cpcnt(dis); }

                        }
                        {
                            //eprint!("Head ... ");
                            //let st = Instant::now();
                            if fdb.lossy < 2 {
                                let mut head_out = head_lzt.get_records(&enc,&-1);
                                //eprintln!("Rec/sec: {:.2?}", (((pp) as u64)/((st.elapsed().as_millis() +1) as u64 ))*1000);

                                let _dis = deindex(&mut head_out);
                                fdb.set_head(head_out);
                            } else {
                                let mut head_out = head_lzt.generate_header(i,j,fdb.paired,cpcnt); 
                                fdb.set_head(head_out);
                            }

                        }
                        if q {
                            //eprintln!("Qual ... ");
                            //let st = Instant::now();
                            let mut qual_out = qual_lzt.get_records(&enc,&-1);
                            //eprintln!("Rec/sec: {:.2?}", (((pp) as u64)/((st.elapsed().as_millis() +1) as u64 ))*1000 );
                            let _dis = deindex(&mut qual_out);
                            //eprintln!("LZT  {:?} \ndis:{:?}", String::from_utf8(qual_out.clone()).unwrap(),_dis);

                            fdb.set_qual(qual_out);

                        }else{
                            let qvec = vec!['\n' as u8; fdb.get_numrec()];
                            fdb.set_qual(qvec);
                        }

                        j += inc;
                        i += inc;


                        fdb.save_append(output, cli.value_of("outfmt").unwrap());
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
