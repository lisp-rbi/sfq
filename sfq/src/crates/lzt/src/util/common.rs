
use crate::ffi::LztObj;


// C function signatires -----------------//

extern "C" {
    pub fn open_lzt(
        pth: *const libc::c_uchar,
        len: libc::c_int
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
