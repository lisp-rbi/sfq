use crate::util::common::{
    encode,
    decode,
    index,
    hindex,
    make_stats,
    get_stats,
    tsv_encode,
    deindex
};
use seq::{Fdb, Set, Get, Push};
use lzt::{FFI,Drop};


#[test]
fn codec() {
    let alpha = vec!['A' as u8, 'C' as u8, 'G' as u8, 'T' as u8];
    let word = vec![4,7,10];
    let num = vec![27,45,12,67,4];
    let mut res = vec![vec![0u8;word[0]*num.len()],
                   vec![0u8;word[1]*num.len()],
                   vec![0u8;word[2]*num.len()]];

    let (mut a, mut b) = (0,0);

    for i in word.iter() {
        for j in num.iter(){
            let vec=encode(*j,*i,&alpha);
            for l in vec.into_iter(){
                res[a][b]= l;
                b+=1;
            }
        }
        b=0;
        a+=1;
    }


    a=0;b=0;

    let mut final_out = vec![vec![0;5];3];
    let final_in = vec![num.clone(), num.clone(), num];


    for x in res.iter() {
        let mut v : Vec<u8> = Vec::new();
        for y in x.iter() {
            v.push(*y);
            if v.len() == word[a]{
                final_out[a][b] = decode(&v, &alpha);
                v.resize(0,0x00);
                b+=1;
            }

        }
        a+=1;
        b=0;
    }

    assert_eq!(final_in,final_out);

}

#[test]
fn indexing_a() {
    let inh = b"@a_1R\n@a_1F\n@a_2R\n@a_2F".to_vec();
    let inp = b"ATGG\nATGC\nTTTT\nTTCG".to_vec();
    let oup = b"AAXATGG\nAGXATGC\nCAXTTTT\nCGXTTCG".to_vec();
    let mut fdb = Fdb::new("fastq");
    fdb.set_seq(inp);
    fdb.set_head(inh);
    let get = fdb.get_tsv("h+s");
    let cp = vec![1,1,1,1];
    let res = index(&get, &cp);

    assert_eq!(String::from_utf8(oup).unwrap(),String::from_utf8(res.0).unwrap());


}


#[test]
fn indexing_b() {

    let inh = b"@a_1R\n@a_1F\n@a_2R\n@a_2F".to_vec();
    let inp = b"&%$R\n//Z&\nHHTT\nIUG".to_vec();
    let oup = b"AAX&%$R\nAGX//Z&\nCAXHHTT\nCGXIUG".to_vec();
    let mut fdb = Fdb::new("fastq");
    fdb.set_head(inh);
    fdb.set_qual(inp);
    let get = fdb.get_tsv("h+q");
    let cp = vec![1,1,1,1];
    let res = index(&get,&cp);


    assert_eq!(String::from_utf8(oup).unwrap(),String::from_utf8(res.0).unwrap());

}


#[test]
fn index_tab() {

    let inp = b"&%$R\n//Z&\nHHTT\nIUG\n".to_vec();
    let oup = b"&%$R\t$A\n//Z&\t$G\nHHTT\t%A\nIUG\t%G\n".to_vec();
    let res = tsv_encode(&inp, true);

    assert_eq!(String::from_utf8(oup).unwrap(),String::from_utf8(res).unwrap());

}


#[test]
fn deindexing() {

    let oup = b"&%$R\n//Z&\nHHTT\nIUG".to_vec();
    let mut inp = b"$A&%$R\n$G//Z&\n%AHHTT\n%GIUG".to_vec();
    deindex(&mut inp);

    assert_eq!(String::from_utf8(oup).unwrap(),String::from_utf8(inp).unwrap());

}



#[test]
fn lossy_compress_decompress() {

    let out_q = "./data/qual-l.sfastq";
    let out_s = "./data/seq-l.sfastq";
    let out_h = "./data/head-l.sfastq";

    let head  = b"@h_1F\n@h_1R\n@h_2F\n@h_2R\n@h_3F\n@h_3R\n@h_4F\n@h_4R".to_vec();
    let seq   = b"ATGC\nATGT\nATGC\nATGG\nAGGG\nTGGG\nAGGG\nTGGG".to_vec();
    let qual  = b"$%&/\n$%//\n$%&(\n$%&A\n$%&H\n$%&O\n$%&U\n$%&U".to_vec();

    let mut fdb = Fdb::new("fastq");

    fdb.set_head(head.clone());
    fdb.set_seq(seq.clone());
    fdb.set_qual(qual.clone());
    fdb.set_model(true);

    fdb.colaps();

    let cpcnt = fdb.get_cpcnt();

    let s = fdb.get_tsv("h+s");
    let h = fdb.get_head();
    let q = fdb.get_tsv("h+q");

    let mut  is = index(&s,&cpcnt);
    let qv = vec![1;cpcnt.len()];
    let mut  iq = index(&q,&qv);
    let mut  ih = hindex(&h,&qv);

//    println!("A   {:?}", String::from_utf8(is.clone()).unwrap());
//    println!("Q   {:?}", String::from_utf8(iq.clone()).unwrap());
//    println!("H   {:?}", String::from_utf8(ih.clone()).unwrap());

    is.0.push(0u8);
    is.0.extend( make_stats(is.1,is.3,is.2,fdb.get_model()));
    iq.0.push(0u8);
    iq.0.extend( make_stats(iq.1,iq.3,iq.2,fdb.get_model()));
    ih.0.push(0u8);
    ih.0.extend( make_stats(ih.1,ih.3,ih.2,fdb.get_model()));

    println!("{:?}", String::from_utf8(ih.0.clone()).unwrap());

    let mut lzt_s = FFI::new(out_s, &mut is.0, 100, false);
    lzt_s.drop();

    let mut lzt_q = FFI::new(out_q,&mut iq.0,100, false);
    lzt_q.drop();

    let mut lzt_h = FFI::new(out_h,&mut ih.0,100, false);
    lzt_h.drop();
    let mut sts: Vec<u8> = FFI::open(&out_s,false).get_records("~~~~");

    let mut st: Vec<u8> = FFI::open(&out_s,false).get_records("A");
    let mut ht: Vec<u8> = FFI::open(&out_h,false).get_records("A");
    let mut qt: Vec<u8> = FFI::open(&out_q,false).get_records("A");



    let dis = deindex(&mut st);
    let dih = deindex(&mut ht);
    let diq = deindex(&mut qt);

    // compression done and successfull!! //
    fdb.set_head(ht);
    fdb.set_seq(st);
    fdb.set_qual(qt);
    fdb.set_cpcnt(dis);

    //println!("0000  {:?}", String::from_utf8(fdb.get_fastq()).unwrap());
    fdb.expand();
    //println!("XXXX  {:?}", String::from_utf8(fdb.get_fastq()).unwrap());
    assert_eq!(
        (   String::from_utf8(seq).unwrap(),
            qual.len()
        ),(
            String::from_utf8(fdb.get_seq()).unwrap(),
            fdb.get_qual().len()
        )
    );

}


#[test]
fn full_compress_decompress() {

    let out_q = "./data/qual-f.sfastq";
    let out_s = "./data/seq-f.sfastq";
    let out_h = "./data/head-f.sfastq";

    let head  = b"@h_1F\n@h_1R\n@h_2F\n@h_2R\n@h_3F\n@h_3R\n@h_4F\n@h_4R".to_vec();
    let seq   = b"ATGC\nATGT\nATGC\nATGG\nAGGG\nTGGG\nAGGG\nTGGG".to_vec();
    let qual  = b"$%&/\n$%//\n$%&(\n$%&A\n$%&H\n$%&O\n$%&U\n$%&U".to_vec();


    let mut fdb = Fdb::new("fastq");

    fdb.set_head(head.clone());
    fdb.set_seq(seq.clone());
    fdb.set_qual(qual.clone());
    fdb.set_model(true);

    let cpcnt = vec![1;8];

    let s = fdb.get_tsv("h+s");
    let h = fdb.get_head();
    let q = fdb.get_tsv("h+q");

    let mut  is = index(&s,&cpcnt);
    let qv = vec![1;cpcnt.len()];
    let mut  iq = index(&q,&qv);
    let mut  ih = hindex(&h,&qv);

//    println!("S   {:?}", String::from_utf8(is.clone()).unwrap());
//    println!("Q   {:?}", String::from_utf8(iq.clone()).unwrap());
//    println!("H   {:?}", String::from_utf8(ih.clone()).unwrap());

    is.0.push(0u8);
    is.0.extend( make_stats(is.1,is.3,is.2,fdb.get_model()));
    iq.0.push(0u8);
    iq.0.extend( make_stats(iq.1,iq.3,iq.2,fdb.get_model()));
    ih.0.push(0u8);
    ih.0.extend( make_stats(ih.1,ih.3,ih.2,fdb.get_model()));

    let mut lzt_s = FFI::new(out_s, &mut is.0, 100, false);
    lzt_s.drop();

    let mut lzt_q = FFI::new(out_q,&mut iq.0,100, false);
    lzt_q.drop();

    let mut lzt_h = FFI::new(out_h,&mut ih.0,100, false);
    lzt_h.drop();

    let mut st: Vec<u8> = FFI::open(&out_s, false).get_records("A");
    let mut ht: Vec<u8> = FFI::open(&out_h,false).get_records("A");
    let mut qt: Vec<u8> = FFI::open(&out_q,false).get_records("A");

    let dis = deindex(&mut st);
    let dih = deindex(&mut ht);
    let diq = deindex(&mut qt);

    // compression done and successfull!! //
    fdb.set_head(ht);
    fdb.set_seq(st);
    fdb.set_qual(qt);
    fdb.set_cpcnt(dis);

//    println!("XXXX  {:?}", String::from_utf8(fdb.get_fastq()).unwrap());

    assert_eq!(
        (   String::from_utf8(seq).unwrap(),
            qual.len()
        ),(
            String::from_utf8(fdb.get_seq()).unwrap(),
            fdb.get_qual().len()
        )
    );

}


#[test]
fn grep_by_index (){
    let out_s = "./data/seq-f-f.sfastq";
    let mut string = FFI::open(&out_s,true).get_records("A");
    let mut fdb = Fdb::new("fastq");
    fdb.set_model(true);

    let dis = deindex(&mut string);
    fdb.set_seq(string);
    assert_eq!(2,1+1);

}


#[test]
fn random_queries (){

    assert_eq!(2,1);

}
