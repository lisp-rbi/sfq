use std::io::{self, prelude::*, Read, BufReader};
use std::fs::{File,metadata};
use crate::ffi::LztObj;

pub fn read_tmp(filename: &str, vec: &mut Vec<u8>, start: i64, end: i64) -> bool{
    assert!(metadata(filename).unwrap().is_file());
    let file = File::open(filename).expect("Unable to read tmp file");
    let file = BufReader::new(file);
    let mut line_number = 0;
    let mut last_line: bool = false;
    for line in file.lines(){
        if (line_number >= start) && (line_number <= end) {
            let line = line.unwrap();
            let mut u8_line: Vec<u8> = line.trim().split(" ").map(|x| x.parse::<u8>().unwrap()).collect(); 
            last_line = u8_line[u8_line.len()-1] != 0u8;
            if last_line {u8_line.push(0u8);}
            vec.extend(u8_line);
        } else if line_number > end {
            break;
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
