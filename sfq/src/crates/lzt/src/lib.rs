
extern crate libc;
mod util;
pub mod ffi;

use std::str;
use std::fs;
use ffi::LztObj;
use util::common::{
    read_tmp,
    make_lzt,
    open_lzt,
    delete_lzt,
    query_lzt,
    get_query_results
};


// global variable -> maybe move into objects ?
static CASHSIZE: usize = 10000;


#[cfg(test)]
mod tests;

pub trait Drop{
    fn drop(&mut self);
}

#[derive(Debug, Clone)]
pub struct FFI {
    raw: Vec<*mut LztObj>
}

impl FFI {

    pub fn new( path : &str, tmp_path: &str, mem: usize, mmode: bool, line_length: usize, numrec: usize, use_lines: bool) -> Self {

        let lpm: usize = match use_lines {
            true => {mem}, 
            // number of lines to read at time
            false => {(mem * 1024) / (line_length * 55)}
        };

        let mut lzt_vec : Vec<*mut LztObj> = Vec::new();
        let mut end_of_file : bool = false;
        let mut start: i64 = 0;
        let mut end: i64 = (lpm as i64) - 1;
        let mut j: i64 = 1;
        while end_of_file == false {
            // if there is only one line left at the end, add to the last trie
            if (end == (numrec-1) as i64) && (numrec%lpm == 0) {end += 1;}
            let mut v : Vec<u8> = Vec::new();
            end_of_file = read_tmp(&tmp_path,&mut v,start,end);
            let pth = format!("{}.{}", path, j.to_string());
            if fs::metadata(&pth).is_ok() == true {
                if fs::metadata(&pth).unwrap().is_file() == true {
                    fs::remove_file(&pth).unwrap();
                }else {
                    fs::remove_dir_all(&pth).unwrap();
                }
            }
            unsafe {
                if make_lzt(
                        v.as_ptr(),
                        v.len() as libc::c_ulong,
                        pth.as_ptr(),
                        pth.len() as libc::c_int,
                ) == false {
                    // FXME: add it to errorr management
                        panic!("Error with creating lzt indedx!");
                };


                lzt_vec.push(
                    open_lzt(
                        pth.as_ptr(),
                        pth.len() as libc::c_int,
                        CASHSIZE as libc::c_int,
                        mmode
                    )
                );

            }

            start += lpm as i64; end += lpm as i64;
            j += 1;
        }

        FFI {
            raw: lzt_vec
        }

    }


    pub fn empty() -> Self {

        FFI {
            raw: Vec::new()
        }
    }


    pub fn open( path: &str, memmod: bool) -> Self {

        let mut lzt_vec : Vec<*mut LztObj> = Vec::new();
        let mut j=1;
        let mut pth = format!("{}.{}", path, j.to_string());

        while fs::metadata(pth.clone()).is_ok() == true {
            let p = unsafe {
                open_lzt(
                    pth.as_ptr(),
                    pth.len() as libc::c_int,
                    CASHSIZE as libc::c_int,
                    memmod
                )
            };
            lzt_vec.push(p);
            j+=1;
            pth = format!("{}.{}", path, j.to_string());
        }

        FFI {
            raw: lzt_vec
        }
    }


    pub fn get_records(&self, pattern: &str)-> Vec<u8> {

        // remember that here you can use ranges to limit
        // the rearch to a specified
        // set of files  : FXME : this is a stupid approach

        let mut qres : Vec<u8> = Vec::new();

        for i in 0..self.raw.len() {
            unsafe{
               let size = query_lzt(
                   self.raw[i],
                   pattern.as_ptr(),
                   pattern.len() as libc::c_ulong,
               ) as usize;

               let mut tq = vec![43u8;size];

               get_query_results(
                   self.raw[i],
                   tq.as_mut_ptr()
               );

               qres.extend(tq);
           }
        }
        if qres[qres.len()-1] == '\n' as u8{
            qres.resize(qres.len()-1, 0x00);
        }

//        println!("{:?}", String::from_utf8(qres.clone()).unwrap()  );

        qres
    }

}


impl Drop for FFI {
    fn drop(&mut self) {
        for i in 0..self.raw.len() {
            unsafe {
                delete_lzt(self.raw[i]);
            }


        }

    }
}
