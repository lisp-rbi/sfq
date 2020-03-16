

mod util;
use seq::{Fdb, Load, Get, Push};
use util::common::{index, tsv_encode};
use clap::*;
use std::str;
use std::str::FromStr;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;




fn parse_cli ()->  clap::ArgMatches<'static> {

    let head : &str =" Program for reformating fast(a/q) files for lzt";

    let  matches = App::new("lztdumper")
          .version("0.01")
          .author("Robert Bakaric <rbakaric@irb.hr>")
          .about(head)

          .arg(Arg::with_name("input")
               .short("i")
               .long("input")
               .required(false)
               .default_value("stdin")
               .value_name("FILE")
               .help("Input file (fasta,fastq,lzt)")
               .takes_value(true))

          .arg(Arg::with_name("input-rev")
               .short("j")
               .long("input-rev")
               .required(false)
               .value_name("FILE")
               .help("Input file of a revers file (fastq)")
               .takes_value(true))

          .arg(Arg::with_name("output")
               .short("o")
               .long("output")
               .required(false)
               .value_name("FILE")
               .default_value("stdout")
               .help("Output file: interleved if paired fastq, if compressed")
               .takes_value(true))

          .arg(Arg::with_name("ftype")
               .short("t")
               .long("ftype")
               .default_value("fastq")
               .required(true)
               .value_name("fastq|fasta|raw")
               .help("Compression supported file types")
               .takes_value(true))

          .arg(Arg::with_name("make-index")
               .short("x")
               .long("make-index")
               .default_value("7")
               .required(true)
               .value_name("hd|>3")
               .help("Make index (4,5,6... - kmer size, hd - high dimensional kmer index)")
               .takes_value(true))

        .get_matches();

    matches
}


fn main(){

    let cli = parse_cli();

    match  cli.value_of("ftype").unwrap() {
        "fastq"  => {
            let mut fdb = Fdb::new("fastq");
            let mut pair = false;

            fdb.load(cli.value_of("input").unwrap(), true);

            if let Some(x) = cli.value_of("input-rev"){
                fdb.load(cli.value_of("input-rev").unwrap(), false);
                pair = true;
            }

            fdb.sort("h");

            let head = fdb.get_head();
            let seq = fdb.get_seq();
            let qual = fdb.get_qual();


            //println!("{}", String::from_utf8(head.clone()).unwrap());
            //println!("{}", String::from_utf8(seq.clone()).unwrap());
            //println!("{}", String::from_utf8(qual.clone()).unwrap());
            let mut h = File::create(
                format!("{}.head",cli.value_of("output").unwrap())
            ).unwrap();
            let mut s = File::create(
                format!("{}.seq",cli.value_of("output").unwrap())
            ).unwrap();
            let mut q = File::create(
                format!("{}.qual",cli.value_of("output").unwrap())
            ).unwrap();

            h.write_all(&tsv_encode(&head,pair)).unwrap();
            s.write_all(&index(&seq,pair)).unwrap();
            q.write_all(&index(&qual,pair)).unwrap();

            h.flush().unwrap();
            s.flush().unwrap();
            q.flush().unwrap();


        },
        "fasta"  => {

        },
        _ => {panic!("File type not dupported")}
    }





}
