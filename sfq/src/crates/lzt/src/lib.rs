
extern crate libc;
mod util;
pub mod ffi;

use std::str;
use std::fs;
use ffi::LztObj;
use util::common::{
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

    pub fn new( path : &str, vec: &mut Vec<u8>, mem: usize, mmode: bool) -> Self {

        let lpm = mem*50; // number of lines per MB
        let (mut l, mut s, mut j) = (0,0,0);


        // check for null termination
        if vec[vec.len()-1] != 0u8 {vec.push(0u8)};

        let mut lzt_vec : Vec<*mut LztObj> = Vec::new();

        for i in 0..vec.len() {

            if vec[i] == 0u8 || i == vec.len()-1 {
                l+=1;
                if l == lpm || i == vec.len()-1 {
                    j+=1;
                    let pth = format!("{}.{}", path, j.to_string());

                    if fs::metadata(&pth).is_ok() == true {
                        if fs::metadata(&pth).unwrap().is_file() == true {
                            fs::remove_file(&pth).unwrap();
                        }else {
                            fs::remove_dir_all(&pth).unwrap();
                        }
                    }

                    let v =  &vec[s..i+1].to_vec();
                    //println!("{:?} -- {} {}", vec[i], v.len(), v[v.len()-1]);
                    unsafe {
                        {
                            if make_lzt(
                                v.as_ptr(),
                                v.len() as libc::c_ulong,
                                pth.as_ptr(),
                                pth.len() as libc::c_int,
                            ) == false {
                            // FXME: add it to errorr management
                                panic!("Error with creating lzt indedx!");
                            };

                        }

                        /*
                        lzt_vec.push(
                            open_lzt(
                                pth.as_ptr(),
                                pth.len() as libc::c_int,
                                CASHSIZE as libc::c_int,
                                mmode
                            )
                        );
                        */
                    }
                    s = i+1;
                    l = 0;
                }
            }
        }

        FFI {
            raw: Vec::new()
        }
/*
        FFI {
            raw: lzt_vec
        }
*/
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
