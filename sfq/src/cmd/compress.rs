use clap::*;
use sys_info::{mem_info};
use crate::util::common::*;

use std::time::Instant;
use std::str::FromStr;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::process;

use seq::{
    Fdb,
    Load,
};
use lzt::{
    FFI,
    Drop
};

pub fn compress (cli: ArgMatches<'static>) -> bool {

    eprintln!("Reading data ... ");

    let before = Instant::now();

    eprintln!("Total RAM at your disposal = {:?} KB", mem_info().unwrap().total);
    // a flag to signal whether we count memory for compression or we take lines
    // from the input
    let mymem : usize = if let Some(x) = cli.value_of("fragment-size") {
        // use all available RAM minus 100MB for safety
        if x == "Max" {(mem_info().unwrap().total - 102400) as usize}
        // user assigns RAM to compress at a time, convert to KB
        else{usize::from_str(x).unwrap() * 1024}
    // if nothing is defined, take 100MB RAM and work with that
    } else {102400};

    let memmod : bool = if let Some(x) = cli.value_of("mem-mod") {
        if x == "R" {true}
        else {false}
    } else {true};

    // take name of the input file, reverse is by defauls an empty string
    let fwd_input: &str = cli.value_of("input").unwrap();
    let mut rev_input: &str = "";
    // if reverse file is given, take its name
    if let Some(_x) = cli.value_of("input-rev") {
        rev_input = cli.value_of("input-rev").unwrap();
    }

    let mut fdb = Fdb::new(cli.value_of("infmt").unwrap());
    fdb.lossy = if let Some(x) = cli.value_of("cmode") {
        usize::from_str(x).unwrap()
    } else {0 as usize};
    if fdb.lossy > 6 {
        eprintln!("ERROR: allowed values of --lossy, -s are 0-6!");
        eprintln!("Inserted value: {:?}\nAborting...", fdb.lossy);
        process::exit(0);
    }

    // If output name is defined in cli, use that
    // otherwise: if it is paired-end take input-file name - ext + .FR.sfq
    //            if it is not paired-end, take input-file name - ext + .sfq
    //            if lossy > 0, take input-file name - ext + .L<lossy>.sfq
    let mut output = if let Some(_x) = cli.value_of("output") {
        String::from(cli.value_of("output").unwrap())
    } else {
        String::from(Path::new(fwd_input).file_stem().and_then(OsStr::to_str).unwrap())
    };
    if let Some(_y) = cli.value_of("input-rev") {    
        output.push_str(".FR");
    }
    if fdb.lossy > 0 {
        output.push_str(".L");
        output.push_str(&fdb.lossy.to_string());
    }
    let mut outdir = output.clone();
    outdir.push_str(".sfq");

    if make_dir(&outdir) == false{ panic!("Creating output directory failed"); };

    fdb.load(fwd_input,rev_input,&outdir,&output);
    let lossy: usize = fdb.lossy;

    eprintln!(" {:.2?}", before.elapsed());

    eprintln!("Preprocessing the input data ... ");

    let j = match cli.value_of("infmt") {
        Some(x) => {
            match x {
                "fastq" => 3,
                "fasta" => 2,
                _       => 0
            }
        },
        None    => {panic!("File type not defined");}
    };
    // erase the FDB structure to free the RAM
    fdb.clear();
    eprintln!("Time spent on data preprocessing: {:.2?}", before.elapsed());

    let before = Instant::now();
    // repeat the loop to read tmp files and compress the data
    eprintln!("Compressing ... ");
    let mut i = 0;
    while i < j {
        let inner_before = Instant::now();
        let mut out = String::new();
        let mut tmp = String::new();

        let _x = match i {
            0 => {
                out = format!("{}/{}.{}",outdir,output,"seq.sfq");
                tmp = format!("{}/{}.{}",outdir,output,"seq.tmp");
            },                                                          
            1 => {                                                      
                out = format!("{}/{}.{}",outdir,output,"head.sfq");
                tmp = format!("{}/{}.{}",outdir,output,"head.tmp");
                if lossy == 2 {
                    fs::remove_file(&tmp).unwrap();
                    i += 1;
                    continue;
                } else if lossy > 2 {
                    i += 1;
                    continue;
                }
            },                                                          
            _ => {                                                      
                if lossy > 4 {
                    i += 1;
                    continue;
                }
                out = format!("{}/{}.{}",outdir,output,"qual.sfq");
                tmp = format!("{}/{}.{}",outdir,output,"qual.tmp");
            }
        };

        let mut lzt = FFI::new(&out,&tmp,mymem,memmod);
        fs::remove_file(&tmp).unwrap();
        lzt.drop();
        let _x = match i {
            0 => {
                eprintln!("Time spent on sequence compression: {:.2?}", inner_before.elapsed());
            },
            1 => {                                                      
                eprintln!("Time spent on header compression: {:.2?}", inner_before.elapsed());
            },
            _ => {                                                      
                eprintln!("Time spent on quality compression: {:.2?}", inner_before.elapsed());
            }
        };
        i += 1;
    }

    eprintln!("Total time spent on data compression: {:.2?}", before.elapsed());
    true

}
