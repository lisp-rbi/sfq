use crate::{FFI, Drop};
use std::fs;

#[test]
fn compress_decompress() {

    let mut my_data = b"0THIS is my\00data stream that\01I Am compressiong at\0195% compression\01rate".to_vec();
    let my_data_out = b"0THIS is my\n0data stream that\n1I Am compressiong at\n195% compression\n1ratex".to_vec();

    let my_out = "./example/my_data.lzt_dir";

    let mut lzt_s = FFI::new(
        my_out,
        &mut my_data,
        10000
    );

    lzt_s.drop();


    let mut xxx = FFI::open(&my_out);
    //println!("in {:?}", xxx);

    let mut lzt_d: Vec<u8> = xxx.get_records("1");

    //println!("{:?}", String::from_utf8(lzt_d.clone()).unwrap());

    assert_eq!(my_data_out, lzt_d);




}


//#[test]
fn random_access() {

    let mut my_data = b"THIS is my\0data stream that\0I Am compressiong at\095% compression\0rate".to_vec();

    let my_out = "./example/my_data-query.lzt";

    let mut lzt_s = FFI::new(
        my_out,
        &mut my_data,
        10000
    );
    lzt_s.drop();

    let mut lzt = FFI::open(&my_out);

    let this = lzt.get_records("THIS");
    let data = lzt.get_records("data");
    let comp = lzt.get_records("9");



    assert_eq!((
        b"THIS is my".to_vec(),
        b"data stream that".to_vec(),
        b"95% compression".to_vec()
    ),(this,data,comp));
}
