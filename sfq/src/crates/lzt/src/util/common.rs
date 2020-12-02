use std::io::{self, prelude::*, Read, BufReader};
use std::fs::{File,metadata};
use crate::ffi::LztObj;

// function to read the temporary file in batches of lpm lines
// returns: boolean whether we found the end of the file
// filename: name of the input file
// vec: input/output vector of bytes to send to LZT
// start/end: input, marks the chunk of file to be read and compressed
// num_lzt_rec: input/output, number of records in first batch
pub fn read_tmp(filename: &str, vec: &mut Vec<u8>, start: u64, end: &mut u64, available_mem: &mut u64) -> bool {
    assert!(metadata(filename).unwrap().is_file());
    let file = File::open(filename).expect("Unable to read tmp file");
    let file = BufReader::new(file);
    let mut line_number: u64 = 0;
    let mut last_line: bool = false;
    for line in file.lines(){
        if line_number >= start {
            // if there are more bytes in the vector than available mem, break the loop
            // also make sure we stop at even number of records (mod is 1 bc we start at 0)
            if (vec.len() as u64 > *available_mem) && (*end%2 == 1) {break;}
            // transform the line in a vector of bytes
            let mut u8_line: Vec<u8> = line.unwrap().as_bytes().to_vec();
            // last line (stats) found if it doesn't end in zero
            last_line = u8_line[u8_line.len()-1] != 0u8;
            // add a zero to the first line
            if last_line {u8_line.push(0u8);}
            // append the transformed line to the vector
            vec.extend(u8_line);
            *end += 1;
        };
        line_number += 1;
    }
    last_line
}

// C function signatires -----------------//

extern "C" {
    pub fn open_lzt(
        pth: *const libc::c_uchar,
        len: libc::c_int,
        cashsize: libc::c_int,
        mmode: bool
    ) -> *mut LztObj;

    pub fn make_lzt(
        vst: *const libc::c_uchar,
        vln: libc::c_ulong,
        pst: *const libc::c_uchar,
        pln: libc::c_int,
    )-> bool;

    pub fn delete_lzt (
        obj: *mut LztObj
    );

    pub fn query_lzt (
        obj: *mut LztObj,
        vst: *const libc::c_uchar,
        vln: libc::c_ulong,
    )-> u64;

    pub fn get_query_results (
        obj: *mut LztObj,
        results: *const libc::c_uchar
    );
}
