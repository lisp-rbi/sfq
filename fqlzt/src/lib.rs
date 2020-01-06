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

extern crate libc;


mod io;
mod util;
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

use opaque::LztObj;
use util::common::*;
use std::ffi::CStr;
use std::os::raw::c_char;


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
        res: *const libc::c_uchar,
        rln: libc::c_ulong
    );

}

pub struct LztFFI {
    raw: *mut LztObj,
}

impl LztFFI {

    pub fn new( path : String, vec2d: Vec<String>) -> Self {

        let mut join = vec2d.join("\n");
        trim_nl(&mut join);

        unsafe {
            if make_lzt(
                join.as_bytes().to_vec().as_ptr(),
                join.len() as libc::c_ulong,
                path.as_bytes().to_vec().as_ptr(),
                path.len() as libc::c_int,
            ) == false {
                // FXME: add it to errorr management
                    panic!("Error with creating lzt indedx!");
            };

            LztFFI {
                raw:  open_lzt(
                    path.as_bytes().to_vec().as_ptr(),
                    path.len() as libc::c_int,
                )
            }
        }

    }


    pub fn open( path: String) -> Self {

        unsafe {
            LztFFI {
                raw:  open_lzt(
                    path.as_bytes().to_vec().as_ptr(),
                    path.len() as libc::c_int,
                )
            }
        }

    }

    pub fn query(&self, pattern: String)-> Vec<String> {

        let mut res : Vec<u8> = vec![64,102,87,65];
        let mut rln = 4;
        let mut result : Vec<u8> = Vec::new();

        use std::slice;

        let result = unsafe{
            query_lzt(
                self.raw,
                pattern.as_bytes().to_vec().as_ptr(),
                pattern.len() as libc::c_ulong,
                res.as_mut_ptr(),
                rln as libc::c_ulong
            );

            slice::from_raw_parts(res.as_mut_ptr(), rln)

        };
        println!("{}", rln);
        for i in 0..rln {
            println!(">>  {:?}", result[i]);
        }
        println!("{}", rln);

        //out.split("\n").map(String::from).collect()

        let e : Vec<String> = vec!["b".to_string()];
        e
    }

}

pub trait Drop{
    fn drop(&mut self);
}

impl Drop for LztFFI {
    fn drop(&mut self) {
        unsafe {
            delete_lzt(self.raw);
        }
    }
}
