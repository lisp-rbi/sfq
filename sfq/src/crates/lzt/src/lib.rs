extern crate libc;
mod util;
pub mod ffi;

use std::str;
use std::fs;
use std::convert::TryInto;
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
    raw: Vec<*mut LztObj>,
    pub num_of_lzt: u8,
}

impl FFI {

    pub fn new( path : &str, tmp_path: &str, mem: usize, mmode: bool) -> Self {

        // convert available memory into bytes, one sign is one byte
        // for each byte of data, we need ~52 bytes of RAM, put 55 for safety
        let mut available_mem: u64 = ((mem * 1024) / 55).try_into().unwrap();

        let mut lzt_vec : Vec<*mut LztObj> = Vec::new();
        let mut end_of_file : bool = false;
        let mut start: u64 = 0;
        let mut end: u64 = 0;
        let mut j: u64 = 1;

        while end_of_file == false {
            let mut v : Vec<u8> = Vec::new();
            end_of_file = read_tmp(&tmp_path,&mut v,start,&mut end,&mut available_mem);
            let pth = format!("{}.{}", path, j.to_string());
            if fs::metadata(&pth).is_ok() == true {
                if fs::metadata(&pth).unwrap().is_file() == true {fs::remove_file(&pth).unwrap();}
                else {fs::remove_dir_all(&pth).unwrap();}
            }
            unsafe {
                if make_lzt(
                        v.as_ptr(),
                        v.len() as libc::c_ulong,
                        pth.as_ptr(),
                        pth.len() as libc::c_int,
                ) == false {
                    // FXME: add it to error management
                        panic!("Error with creating lzt index!");
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

            // in new Trie, start where we last stopped
            start = end;
            j += 1;
        }

        FFI {
            raw: lzt_vec,
            num_of_lzt: (j-1) as u8,
        }
    }


    pub fn empty() -> Self {

        FFI {
            raw: Vec::new(),
            num_of_lzt: 0,
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
            raw: lzt_vec,
            num_of_lzt: j-1,
        }
    }

    pub fn get_records(&self, pattern: &str, position: &i32)-> Vec<u8> {

        // remember that here you can use ranges to limit
        // the rearch to a specified
        // set of files  : FXME : this is a stupid approach

        let mut qres : Vec<u8> = Vec::new();

        // if position is -1, search all LZTs for the pattern
        match position {
            -1 => {
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
            }
        // otherwise, search only given LZT for the pattern
            _ => {
                assert!(position >= &1);
                unsafe{
                    let size = query_lzt(
                        self.raw[(*position as usize)-1],
                        pattern.as_ptr(),
                        pattern.len() as libc::c_ulong,
                    ) as usize;
                
                    let mut tq = vec![43u8;size];
                
                    get_query_results(
                        self.raw[(*position as usize)-1],
                        tq.as_mut_ptr()
                    );

                    qres.extend(tq);
                }
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
