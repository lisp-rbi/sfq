/*
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * This software consists of voluntary contributions made by many individuals
 * and is licensed under the MIT license. For more information, see
 * <http://www.doctrine-project.org>.
 */


 /*
 FXME ...

 NOTES: This library should be split i two: fqlzt and seq !

      fqlzt -> contains ABI for lzt

      seq   -> contains API for uldl of fasta and fastq files
 */

extern crate libc;


pub mod io;
pub mod util;
// FXME: move to src/ffi/mod.rs
mod opaque {
    #[repr(C)]
    pub(in super)
    struct LztObj {
        _opaque: [*const u8; 0],
    }

}
#[cfg(test)]
mod tests;



use std::iter::FromIterator;
use opaque::LztObj;
use util::common::{*};
use std::str;
use fxhash::FxHashMap;


/// Traits -------------------------------//
pub trait Drop{
    fn drop(&mut self);
}

pub trait IO{
    fn open(&mut self, path: &str) -> &mut Self;
    fn save(path: &str) -> bool;
}

pub trait GetSet {
    fn get(&mut self, format: &str)-> Vec<u8>;
    fn set(data: Vec<u8>)-> bool;
}


// C function signatires -----------------//
extern "C" {
    fn open_lzt(
        pth: *const libc::c_uchar,
        len: libc::c_int
    ) -> *mut LztObj;

    fn make_lzt(
        vst: *const libc::c_uchar,
        vln: libc::c_ulong,
        pst: *const libc::c_uchar,
        pln: libc::c_int,
    )-> bool;

    fn delete_lzt (
        obj: *mut LztObj
    );

    fn query_lzt (
        obj: *mut LztObj,
        vst: *const libc::c_uchar,
        vln: libc::c_ulong,
    )-> u64;

    fn get_query_results (
        obj: *mut LztObj,
        results: *const libc::c_uchar
    );
}




/// Objects ------------------------------//



/// FFI object -----------------------------
pub struct FFI {
    raw: *mut LztObj,
}

impl FFI {

    pub fn new( path : &str, vec: &mut Vec<u8>) -> Self {

//        let mut join = vec.join("\n");
        vec.pop();
//        trim_nl(&mut vec);
        unsafe {
            if make_lzt(
                vec.as_ptr(),
                vec.len() as libc::c_ulong,
                path.as_ptr(),
                path.len() as libc::c_int,
            ) == false {
                // FXME: add it to errorr management
                    panic!("Error with creating lzt indedx!");
            };

            FFI {
                raw:  open_lzt(
                    path.as_ptr(),
                    path.len() as libc::c_int,
                )
            }
        }

    }


    pub fn open( path: String) -> Self {

        unsafe {
            FFI {
                raw:  open_lzt(
                    path.as_bytes().to_vec().as_ptr(),
                    path.len() as libc::c_int,
                )
            }
        }

    }

    pub fn query(&self, pattern: String)-> Vec<String> {

        let mut qres : Vec<String> = Vec::new();  // FXME: maybe avoid string -> for future

         unsafe{
            let size = query_lzt(
                self.raw,
                pattern.as_bytes().to_vec().as_ptr(),
                pattern.len() as libc::c_ulong,
            ) as usize;

            let mut vec1d : Vec<u8> = vec![0u8;size];
            get_query_results(
                self.raw,
                vec1d.as_mut_ptr()
            );
            {
                let mut tmp : String = str::from_utf8(&vec1d).unwrap().to_string();
                trim_nl(&mut tmp);
                qres = Vec::from_iter(
                    tmp.split("\n").map(String::from)
                );
            }

        };
        qres
    }

}


impl Drop for FFI {
    fn drop(&mut self) {
        unsafe {
            delete_lzt(self.raw);
        }
    }
}



/// Fdb object -----------------------------
///
/// Fdb provides an API for uldl of fast(a/q)
/// formated flat text files.

pub struct Fdb {
    ftype: String,
    nrec: usize,
    seq:  Vec<u8>,
    qual: Vec<u8>,
    head: Vec<u8>,
}

impl Fdb {
    pub fn new (rtype: &str)-> Self{

        let ftype : String   = match rtype {
            "fasta" | "fastq" | "raw" => rtype.to_string(),
             _ => panic!("File format {} not supported !",rtype ),
        };

        Fdb{
            ftype: ftype,
            nrec: 0,
            seq: Vec::new(),
            qual: Vec::new(),
            head: Vec::new()
        }
    }

    fn sort(&self, hash: &mut FxHashMap<usize, Vec<usize>>, vec: &mut Vec<u8>) -> bool{

        let mut st = String::from_utf8(vec.clone()).unwrap();
        st.pop();
        let seq: Vec<String> = st.split("\n").map(|s| s.to_string()).collect();
        let mut ht = String::from_utf8(self.head.clone()).unwrap();
        ht.pop();
        let head: Vec<String> = ht.split("\n").map(|s| s.to_string()).collect();
        let mut res : Vec<String> =  vec!["".to_string();seq.len()];

        for i in 0..seq.len() {
            res[i].push_str(&seq[i]);
            res[i].push('\t');
            res[i].push_str(&head[i].len().to_string());
            res[i].push('\t');
            res[i].push_str(&i.clone().to_string());
        }


        make_sort_idx(hash, &mut res);

        true
    }


    fn make_cvec(&self, dvec: &mut Vec<u8>, hashmap: FxHashMap<usize, Vec<usize>>) -> Vec<u8>{

        let slen = dvec.len();
        let hlen = self.head.len();
        let mut tlen = 0;
        let mut vec = vec![0; (hlen) + 1 + (slen) ];

        let mut s = 0;
        let mut h = 0;

        for i in 0..self.nrec {
            match hashmap.get(&i) {
                Some(x) => {

                    let mut v = x[1];
                    let v_zero = v;
                    let copy =  x[0].to_string();

                    while self.head[h] != 10 {
                        vec[v] = self.head[h];
                        v=v+1;h=h+1;
                    }
                    vec[v] = b'\t';
                    v=v+1;h=h+1;

                    while dvec[s] != 10 {
                        vec[v] = dvec[s];
                        v=v+1;s=s+1;
                    }
                    vec[v] = b'\t';
                    v=v+1;
                    for c in copy.chars() {
                        vec[v] = c as u8;
                        v=v+1;
                    }
                    vec[v] = b'\n';
                    s=s+1;v=v+1;;
                    tlen= tlen + (v - v_zero);
                },
                _       => {
                    while self.head[h] != 10 {h=h+1;}
                    while dvec[s] != 10 {s=s+1;}
                    s=s+1;h=h+1;
                }
            };
        }

        vec.resize(tlen-1,  0x00);
        vec
    }


}

impl GetSet for Fdb {
    fn get(&mut self, format: &str)-> Vec<u8>{

        let mut vec : Vec<u8> = Vec::new();
        let mut hashmap : FxHashMap<usize, Vec<usize>> = FxHashMap::default();

        match format {
            "H+F+R+Fq+Rq" => {
                panic!("Not implemented");
            },
            "H(F,R)" => {

                self.sort(&mut hashmap, &mut self.seq.clone());
                vec = self.make_cvec(&mut self.seq.clone(),hashmap);

            },
            "H(Fq,Rq)" => {

                self.sort(&mut hashmap, &mut self.qual.clone());
                vec = self.make_cvec(&mut self.qual.clone(),hashmap);

            },
            "H(F+Fq,R+Rq)" =>{
                panic!("Not implemented");
            },
            _  => {
                panic!("{} format not supported!", format);
            }
        }

        //println!("{:?}", String::from_utf8_lossy(&vec));
        vec
    }

    fn set (data: Vec<u8>)-> bool {
        true
    }
}


impl IO for Fdb {
    fn open(&mut self, file: &str) -> &mut Self{

        // make reader
        let reader = make_reader(file);
        println!("Uploading ...");

        match &self.ftype[..] {
            "fasta" => {
                panic!("{} format not implemented yet!", self.ftype);
            },
            "fastq" => {

                if let Ok(true) = self.fastq_up(reader) {
                    println!("{} file uploaded !", self.ftype);
                };

            },
            "raw" => {
                panic!("{} format not implemented yet!", self.ftype);
            },
            _      => {
                panic!("Format {} - Not supported!", self.ftype);
            }
        }

        self
    }


    fn save(path: &str)-> bool{
       true
    }
}
