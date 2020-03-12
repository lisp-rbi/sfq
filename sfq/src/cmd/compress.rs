use clap::*;
use sys_info::{mem_info};
use crate::util::common::{
    *
};

use std::time::Instant;
use std::str::FromStr;

use seq::{
    Fdb,
    Get,
    Load,
    Set
};
use lzt::{
    FFI,
    Drop
};




pub fn compress (cli: ArgMatches<'static>) -> bool {

    eprint!("Reading data ... ");

    let before = Instant::now();

    let mut fdb = Fdb::new(cli.value_of("infmt").unwrap());
    fdb.load(cli.value_of("input").unwrap(), true);

    if let Some(x) = cli.value_of("input-rev") {
        fdb.load(cli.value_of("input-rev").unwrap(), false);
    }

    let mymem : usize = if let Some(x) = cli.value_of("max-mem") {
        if x == "Max" {
            (mem_info().unwrap().total/1000) as usize
        }else{
            usize::from_str(x).unwrap()
        }
    }else{
        1000
    };

eprintln!("MMMM{}", mymem);

    if let Some(x) = cli.value_of("cmode") {

        if x == "lossy"{
            fdb.colaps();
        }else{
            fdb.sort("h");
            fdb.set_cpcnt(vec![1;(fdb.get_numrec()+2)*2]);
        }
    }else{
        panic!("Compression compromised!");
    }


    eprintln!(" {:.2?}", before.elapsed());

    eprint!("Compressing ... ");

    let before = Instant::now();

    let j = match cli.value_of("infmt") {
        Some(x) => {
            match x {
                "fastq" => 3,
                "fasta" => 2,
                _       => 0
            }
        },
        None  => {
            panic!("File type not defined");
        }
    };
    let mut i = 0;

    // loop is good for the memory
    while i < j {
        let mut out = String::new();

        let mut x = match i {
            0 => {
                out = format!("{}.{}",cli.value_of("output").unwrap(),"head.sfq");
                let cpcnt = fdb.get_cpcnt();
                hindex(&fdb.get_head(),&cpcnt)

            },
            1 => {
                out = format!("{}.{}",cli.value_of("output").unwrap(),"seq.sfq");
                let cpcnt = fdb.get_cpcnt();
                index(&fdb.get_tsv("h+s"),&cpcnt)

            },
            _ => {
                out = format!("{}.{}",cli.value_of("output").unwrap(),"qual.sfq");
                let cpcnt = fdb.get_cpcnt();
                index(&fdb.get_tsv("h+q"),&cpcnt)

            }
        };
        //stats processing
        x.0.push(10u8);
        x.0.extend( make_stats(x.1,x.3,x.2, fdb.get_model()));



        // make zthis more consise
        for i in 0..x.0.len(){
          if x.0[i] == 10u8{
              x.0[i]=0u8;
          }
        }

        let mut lzt = FFI::new(
            &out,
            &mut x.0,
            mymem
        );
        lzt.drop();

        i+=1;
    }

    eprintln!(" {:.2?}", before.elapsed());
    true




}
