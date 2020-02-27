
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


#[cfg(test)]
mod tests;



pub trait Drop{
    fn drop(&mut self);
}


pub struct FFI {
    raw: Vec<*mut LztObj>
}

impl FFI {

    pub fn new( path : &str, vec: &mut Vec<u8>, mem: usize) -> Self {

        let lpm = mem*100; // number of lines per MB
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
                    let v =  &vec[s..i+1].to_vec();
                    //println!("{:?} -- {} {}", vec[i], v.len(), v[v.len()-1]);
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
                            )
                        );
                    }
                    s = i+1;
                    l = 0;
                }
            }
        }

        FFI {
            raw: lzt_vec
        }
    }


    pub fn open( path: &str) -> Self {

        let mut lzt_vec : Vec<*mut LztObj> = Vec::new();
        let mut j=1;
        let mut pth = format!("{}.{}", path, j.to_string());

        while fs::metadata(pth.clone()).is_ok() == true {
            let p = unsafe {
                open_lzt(
                    pth.as_ptr(),
                    pth.len() as libc::c_int,
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

    pub fn query(&self, pattern: &str)-> Vec<u8> {

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

               let mut tq = vec![0u8;size];

               get_query_results(
                   self.raw[i],
                   tq.as_mut_ptr()
               );

               qres.extend(tq);
           }
        }
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
