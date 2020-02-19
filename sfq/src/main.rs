
// modules
mod cli;
mod util;


// calls
use cli::parse_cli;
use seq::{Fdb};
use std::time::Instant;
use std::str::FromStr;



fn main() {

    let cli = parse_cli();

    match cli.value_of("action").unwrap() {
        "c" => {
            print!("Compressing...");
            let before = Instant::now();



            // construct Fdb (pass a model) -> reads headers if fast(a/q) -> sort and index in temp memory

            // get seq -> readin the seq into temp vec in a preset order  and Return

            // get qual ->  readin the seq into temp vec in a preset order  and Return

            // get head -> dump head from the temp memory

            // execute compression my passing each element into ffi


            println!(" {:.2?}", before.elapsed());
        },
        "d" => {
            println!("Decompressing...");
            let before = Instant::now();

            println!(" {:.2?}", before.elapsed());
        },
        "g" => {
            println!("Extracting...");
            let before = Instant::now();

            println!(" {:.2?}", before.elapsed());
        },
        _   => {
            panic!("Unrecognized action: -a {}", cli.value_of("action").unwrap());
        }
    }



}
