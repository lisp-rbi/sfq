
mod ffi;

use ffi::LztObj;

pub trait Drop{
    fn drop(&mut self);
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




/// FFI object -----------------------------
pub struct FFI {
    raw: Vec<*mut LztObj>,  // reimplemet this as a vector of objects
}

impl FFI {

    pub fn new( path : &str, vec: &mut Vec<u8>, mem: usize) -> Self {

        let lpm = mem*120;
        let mut l = 0;
        let mut s = 0;
        let mut j = 0;


        // check for null termination
        if vec[vec.len()-1] != 0u8 {vec.push(0u8)};

        let mut lzt_vec : Vec<*mut LztObj> = Vec::new();

        for i in 0..vec.len() {

            if vec[i] == 0u8 || i == vec.len()-1 {
                l = l + 1;
                if l == lpm || i == vec.len()-1 {
                    j=j+1;
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
        print!("\nDecompressing: {} ...", pth);
        while fs::metadata(pth.clone()).is_ok() == true {
            let p = unsafe {
                open_lzt(
                    pth.as_ptr(),
                    pth.len() as libc::c_int,
                )
            };
            lzt_vec.push(p);
            j=j+1;
            pth = format!("{}.{}", path, j.to_string());
            print!("ok!\nDecompressing: {} ...", pth);
        }
        println!("ok!\n");

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
