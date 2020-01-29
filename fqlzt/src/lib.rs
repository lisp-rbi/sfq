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
    fn load(&mut self, path: &str) -> &mut Self;
    fn save(path: &str) -> bool;
}

pub trait GetSet {
    fn get(&mut self, format: &str)-> Vec<Vec<u8>>;
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


    pub fn open( path: &str) -> Self {

        unsafe {
            FFI {
                raw:  open_lzt(
                    path.as_ptr(),
                    path.len() as libc::c_int,
                )
            }
        }

    }

    pub fn query(&self, pattern: &str)-> Vec<u8> {

        let mut qres : Vec<u8> = Vec::new();  // FXME: maybe avoid string -> for future

        println!("{}", pattern.len());

         unsafe{
            let size = query_lzt(
                self.raw,
                pattern.as_ptr(),
                pattern.len() as libc::c_ulong,
            ) as usize;
            println!("here");

            qres = vec![0u8;size];
            println!("here");
            get_query_results(
                self.raw,
                qres.as_mut_ptr()
            );
/*            {
                let mut tmp : String = str::from_utf8(&vec1d).unwrap().to_string();
                trim_nl(&mut tmp);
                qres = Vec::from_iter(
                    tmp.split("\n").map(String::from)
                );
            }
*/
        }
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
    mem: usize
}

impl Fdb {
    pub fn new (rtype: &str, mem: usize)-> Self{

        let ftype : String   = match rtype {
            "fasta" | "fastq" | "raw" => rtype.to_string(),
             _ => panic!("File format {} not supported !",rtype ),
        };

        Fdb{
            ftype: ftype,
            nrec: 0,
            seq: Vec::new(),
            qual: Vec::new(),
            head: Vec::new(),
            mem: mem
        }
    }


    fn sort (&self, invec: &mut Vec<u8>) -> Vec<Vec<u8>>{

        let mut v = String::from_utf8(invec.clone()).unwrap();
        v.pop();
        let vec: Vec<String> = v.split("\n").map(|s| s.to_string()).collect();

        let mut ht = String::from_utf8(self.head.clone()).unwrap();
        ht.pop();
        let mut head: Vec<String> = ht.split("\n").map(|s| s.to_string()).collect();

        for i in 0..vec.len() {
            head[i].push('\t');
            head[i].push_str(&vec[i]);
        }

        head.sort();

        let lpm = self.mem*140;
        let fractions  = ((head.len() as f64 / lpm as f64) as f64).ceil();
        let mut memvec : Vec<Vec<u8>>  = vec![Vec::with_capacity(lpm as usize);fractions as usize];

        let mut j=0;
        let mut l=0;

        for i in head.into_iter() {
            l=l+1;
            memvec[j].extend(i.as_bytes());
            memvec[j].push(0u8);
            //println!("line {} :  {:?}",j, memvec[j] );
            if l == lpm{
                j=j+1;
                l=0;
            }


//            vecu8.extend(i.as_bytes());
//            vecu8.push('\n' as u8);

        }
/*
        for i in 0..vecu8.len(){
            print!("{}", vecu8[i] as char);
        }
*/
//        vecu8

        memvec
    }

    fn isort(
        &self,
        ulist: &mut Vec<u8>,
    ) -> bool{

        let mut hash_prim : FxHashMap<usize, usize> = FxHashMap::default();
        let mut hash_sec  : FxHashMap<usize, usize> = FxHashMap::default();
       // if i can sort on u8 this is unnecessary
        let mut st = String::from_utf8(self.seq.clone()).unwrap();
        st.pop();
        let seq: Vec<String> = st.split("\n").map(|s| s.to_string()).collect();

        let mut qt = String::from_utf8(self.qual.clone()).unwrap();
        qt.pop();
        let qual: Vec<String> = qt.split("\n").map(|s| s.to_string()).collect();

        let mut ht = String::from_utf8(self.head.clone()).unwrap();
        ht.pop();
        let head: Vec<String> = ht.split("\n").map(|s| s.to_string()).collect();


        let mut vec : Vec<String> =  vec!["".to_string(); qual.len()];

        for i in 0..qual.len() {
            vec[i].push_str(&qual[i]);
            vec[i].push('\t');
            vec[i].push_str(&i.to_string());
        }


        vec.sort();
        self.index_prim(ulist, &vec, &mut hash_prim, &mut hash_sec);

        println!("{:?}",hash_prim);
        println!("{:?}",hash_sec);

        vec = vec!["".to_string(); seq.len()];

        for i in 0..seq.len() {
            vec[i].push_str(&seq[i]);
            vec[i].push('\t');
            vec[i].push_str(&i.to_string());
        }


        vec.sort();

        self.index_sec (ulist, &vec, &hash_prim, &hash_sec);


        //make_sort_idx(hash, &mut res);

        true
    }

    // move to sort index

    fn index_prim(
        &self,
        ulist: &mut Vec<u8>,
        vec: &Vec<String>,
        hash_prim: &mut FxHashMap<usize, usize>,
        hash_sec: &mut FxHashMap<usize, usize>,
    ) {
        let mut j = 0;
        let mut c = 1;
        let alpha = 25;
        let poly = vec.len() as f64;
        let mut pvec : Vec<String> =  vec[0].split("\t").map(|s| s.to_string()).collect();

        for i in 1..vec.len() {
            let cvec : Vec<String> = vec[i].split("\t").map(|s| s.to_string()).collect();

            if cvec[0] != pvec[0] {

                ulist.extend(make_hash_key(
                    j,
                    alpha,
                    poly.log(alpha as f64).ceil() as usize
                ).into_bytes());

                ulist.push(b'\t');
                ulist.extend(pvec[0].clone().into_bytes());
                ulist.push(b'\t');
                ulist.extend(c.to_string().into_bytes());
                ulist.push(b'\n');

                hash_prim.insert(pvec[1].parse::<usize>().unwrap(),j);

                j = j+1;
                c=1;

            }else{
                c = c+1;
                hash_sec.insert(pvec[1].parse::<usize>().unwrap(),j);
            }
            pvec = cvec;
        }

        ulist.extend(pvec[0].clone().into_bytes());
        ulist.push(b'\t');
        ulist.extend(c.to_string().into_bytes());
        ulist.push(b'\n');
        hash_prim.insert(pvec[1].parse::<usize>().unwrap(),j);


    }

    fn index_sec(
        &self,
        ulist: &mut Vec<u8>,
        vec: &Vec<String>,
        hash_prim: &FxHashMap<usize, usize>,
        hash_sec: &FxHashMap<usize, usize>,
    ) {

        let mut j = 0;
        let mut c = 1;
        println!("{}", vec[0]   );
        let mut pvec : Vec<String> =  vec[0].split("\t").map(|s| s.to_string()).collect();
        let mut idx  = "".to_string();

        for i in 1..vec.len() {
            println!("{}", vec[i]   );
            let cvec : Vec<String> = vec[i].split("\t").map(|s| s.to_string()).collect();

            if cvec[0] != pvec[0] {
                println!("here {:?}",pvec[1].parse::<usize>());
                if hash_prim.contains_key(&pvec[1].parse::<usize>().unwrap()) {
                    ulist.extend(pvec[0].clone().into_bytes());
                    ulist.push(b'\t');
                    ulist.extend(c.to_string().into_bytes());
                    ulist.push(b'\t');
                    let d =  hash_prim.get(&pvec[1].parse::<usize>().unwrap()).unwrap();
                    ulist.extend(d.to_string().into_bytes());
                    ulist.extend(idx.clone().into_bytes());
                    ulist.push(b'\n');
                    idx="".to_string();
                }else{
                    println!("here else get sec: {}", pvec[1].parse::<usize>().unwrap());

                    ulist.extend(pvec[0].clone().into_bytes());
                    ulist.push(b'\t');
                    ulist.extend(c.to_string().into_bytes());
                    ulist.push(b'\t');
                    let d =  hash_sec.get(&pvec[1].parse::<usize>().unwrap()).unwrap();
                    ulist.extend(d.to_string().into_bytes());
                    ulist.push(b'\n');

                }
                j=j+1;
                c=1;
            }else{
                println!("here  eeee");
                if hash_prim.contains_key(&pvec[1].parse::<usize>().unwrap()) {
                    let d =  hash_prim.get(&pvec[1].parse::<usize>().unwrap()).unwrap();
                    idx.push_str(",");
                    idx.push_str(&d.to_string());

                }
                c= c+1;
            }
            pvec = cvec;
        }

        if hash_prim.contains_key(&pvec[1].parse::<usize>().unwrap()) {
            ulist.extend(pvec[0].clone().into_bytes());
            ulist.push(b'\t');
            ulist.extend(c.to_string().into_bytes());
            ulist.push(b'\t');
            let d =  hash_prim.get(&pvec[1].parse::<usize>().unwrap()).unwrap();
            ulist.extend(d.to_string().into_bytes());
            ulist.extend(idx.clone().into_bytes());
            ulist.push(b'\n');

        }else{
            println!("here else get sec: {}", pvec[1].parse::<usize>().unwrap());

            ulist.extend(pvec[0].clone().into_bytes());
            ulist.push(b'\t');
            ulist.extend(c.to_string().into_bytes());
            ulist.push(b'\t');
            let d =  hash_sec.get(&pvec[1].parse::<usize>().unwrap()).unwrap();
            ulist.extend(d.to_string().into_bytes());
            ulist.push(b'\n');

        }


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
/*
    pub fn format(&mut self, data: Vec<u8>)-> &mut Self {

        for i in data.into_iter() {

        }
        self
    }
*/

}

impl GetSet for Fdb {
    fn get(&mut self, format: &str)-> Vec<Vec<u8>>{

        let mut vec : Vec<u8> = Vec::new();
        let mut vec2d : Vec<Vec<u8>> = Vec::new();
        //let mut hashmap : FxHashMap<usize, Vec<usize>> = FxHashMap::default();

        match format {
            "H+F+R+Fq+Rq" => {
                panic!("Not implemented");
            },
            "Experimental" => {

                //let mut ulist : Vec<u8> = Vec::new();

                self.isort(&mut vec);
//                self.sort(&mut hash_prim, &mut hash_sec, &mut ulist, &mut self.qual.clone());

                for i in vec.iter(){
                    print!("{}", *i as char);
                }
            },

            "H(F,R)" => {

                vec2d = self.sort(&mut self.seq.clone());

            },
            "H(Fq,Rq)" => {

                vec2d = self.sort(&mut self.qual.clone());

            },

            "H(F+Fq,R+Rq)" =>{
                panic!("Not implemented");
            },
            _  => {
                panic!("{} format not supported!", format);
            }
        }

        //println!("{:?}", String::from_utf8_lossy(&vec));
        vec2d
    }

    fn set (data: Vec<u8>)-> bool {
        true
    }
}


impl IO for Fdb {
    fn load(&mut self, file: &str) -> &mut Self{

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
