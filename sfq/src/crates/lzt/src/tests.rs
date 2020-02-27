use crate::{FFI, Drop};

#[test]
fn compress_decompress() {

    let mut my_data = b"0THIS is my\00data stream that\00I Am compressiong at\0095% compression\00rate".to_vec();
    let my_data_out = b"0THIS is my\n0data stream that\n0I Am compressiong at\n095% compression\n0rate\n".to_vec();

    let my_out = "./example/my_data.lzt";

    let mut lzt_s = FFI::new(
        my_out,
        &mut my_data,
        10000
    );
    lzt_s.drop();

    let mut lzt_d: Vec<u8> = FFI::open(&my_out).query("0");

    assert_eq!(my_data_out, lzt_d);
}


#[test]
fn query() {

    let mut my_data = b"THIS is my\0data stream that\0I Am compressiong at\095% compression\0rate".to_vec();

    let my_out = "./example/my_data-query.lzt";

    let mut lzt_s = FFI::new(
        my_out,
        &mut my_data,
        10000
    );
    lzt_s.drop();

    let mut lzt = FFI::open(&my_out);

    let this = lzt.query("THIS");
    let data = lzt.query("data");
    let comp = lzt.query("9");

    assert_eq!((
        b"THIS is my\n".to_vec(),
        b"data stream that\n".to_vec(),
        b"95% compression\n".to_vec()
    ),(this,data,comp));
}
