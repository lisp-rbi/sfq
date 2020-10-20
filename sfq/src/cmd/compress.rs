use clap::*;
use sys_info::{mem_info};
use crate::util::common::*;

use std::time::Instant;
use std::str::FromStr;
use std::mem;

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

    eprintln!("Reading data ... ");

    let before = Instant::now();

    // if output directory name exists in cli, use that, 
    // otherwise use the basic output name
    let outdir: &str = if let Some(x) = cli.value_of("output-dir") {
        cli.value_of("output-dir").unwrap()
    } else {
        cli.value_of("output").unwrap()
    };
    if make_dir(outdir) == false{
        panic!("Creating output directory failed");
    };

    let mut fdb = Fdb::new(cli.value_of("infmt").unwrap());
    fdb.load(cli.value_of("input").unwrap(), true);

    if let Some(x) = cli.value_of("input-rev") {
        fdb.load(cli.value_of("input-rev").unwrap(), false);
    }

    // a flag to signal whether we count memory for compression or lines
    let mut use_lines: bool = false;
    let mymem : usize = if let Some(x) = cli.value_of("fragment-size") {
        if x == "Max" {
            // use all available RAM minus 100MB for safety
            (mem_info().unwrap().total - 102400) as usize
        }else{
            // we take the number of lines to compress at a time
            use_lines = true;
            usize::from_str(x).unwrap()
        }
    }else{
        // if nothing is defined, take 100MB RAM and work with that
        102400
    };

    let memmod : bool = if let Some(x) = cli.value_of("mem-mod") {
        if x == "R" {
            true
        }else{
            false
        }
    }else{
        true
    };


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

    eprintln!("Preprocessing the input data ... ");

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
    let mut line_length: usize = 0;
    let mut stop_count: bool = false;

    // start the loop to preprocess the data and save tmp files
    while i < j {
        let mut out = String::new();
        let mut tmp = String::new();

        let mut x = match i {
            0 => {
                tmp = format!("{}/{}.{}",outdir,cli.value_of("output").unwrap(),"seq.tmp");
                let cpcnt = fdb.get_cpcnt();
                index(&fdb.get_tsv("h+s"),&cpcnt)

            },
            1 => {
                tmp = format!("{}/{}.{}",outdir,cli.value_of("output").unwrap(),"head.tmp");
                let cpcnt = fdb.get_cpcnt();
                hindex(&fdb.get_head(),&cpcnt)

            },
            _ => {
                tmp = format!("{}/{}.{}",outdir,cli.value_of("output").unwrap(),"qual.tmp");
                let cpcnt = fdb.get_cpcnt();
                index(&fdb.get_tsv("h+q"),&cpcnt)

            }
        };

        //stats processing
        x.0.push(10u8);
        x.0.extend( make_stats(x.1,x.3.clone(),x.2, fdb.get_model()));

        // make zthis more consise
        for j in 0..x.0.len(){
            //eprint!("{}", x.0[j] as char); // DEBUG!!
            if stop_count == false{
                line_length += 1;
            }
            if x.0[j] == 10u8{
                x.0[j]=0u8;
                stop_count = true;
            }
        }
        if save_tmp(&tmp,&mut x.0) == false {
            panic!("Error saving temporary file {:?}",tmp);
        }
        drop(x);

        i+=1;
    }
    // erase the FDB structure to free the RAM
    fdb.clear();
    eprintln!("Time spent on data preprocessing: {:.2?}", before.elapsed());

    let before = Instant::now();
    // repeat the loop to read tmp files and compress the data
    eprintln!("Compressing ... ");
    i = 0;
    while i < j {
        let mut out = String::new();
        let mut tmp = String::new();

        let mut x = match i {
            0 => {
                out = format!("{}/{}.{}",outdir,cli.value_of("output").unwrap(),"seq.sfq");
                tmp = format!("{}/{}.{}",outdir,cli.value_of("output").unwrap(),"seq.tmp");
            },
            1 => {
                out = format!("{}/{}.{}",outdir,cli.value_of("output").unwrap(),"head.sfq");
                tmp = format!("{}/{}.{}",outdir,cli.value_of("output").unwrap(),"head.tmp");
            },
            _ => {
                out = format!("{}/{}.{}",outdir,cli.value_of("output").unwrap(),"qual.sfq");
                tmp = format!("{}/{}.{}",outdir,cli.value_of("output").unwrap(),"qual.tmp");
            }
        };

        let mut lzt = FFI::new(
            &out,
            &tmp,
            mymem,
            memmod,
            line_length,
            use_lines
        );
        lzt.drop();
        i+=1;
    }

    eprintln!("Time spent on data compression: {:.2?}", before.elapsed());
    true

}
