use crate::util::common::{
    encode,
    decode,
    index,
    tsv_encode,
    deindex
};



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

    let inp = b"ATGG\nATGC\nTTTT\nTTCG\n".to_vec();
    let oup = b"AAATGG\nAGATGC\nCATTTT\nCGTTCG\n".to_vec();
    let res = index(inp, true);

    assert_eq!(String::from_utf8(oup).unwrap(),String::from_utf8(res).unwrap());


}


#[test]
fn indexing_b() {

    let inp = b"&%$R\n//Z&\nHHTT\nIUG".to_vec();
    let oup = b"$A&%$R\n$G//Z&\n%AHHTT\n%GIUG".to_vec();
    let res = index(inp, true);

    assert_eq!(String::from_utf8(oup).unwrap(),String::from_utf8(res).unwrap());

}


#[test]
fn index_tab() {

    let inp = b"&%$R\n//Z&\nHHTT\nIUG\n".to_vec();
    let oup = b"&%$R\t$A\n//Z&\t$G\nHHTT\t%A\nIUG\t%G\n".to_vec();
    let res = tsv_encode(inp, true);

    assert_eq!(String::from_utf8(oup).unwrap(),String::from_utf8(res).unwrap());

}


#[test]
fn deindexing() {

    let oup = b"&%$R\n//Z&\nHHTT\nIUG".to_vec();
    let mut inp = b"$A&%$R\n$G//Z&\n%AHHTT\n%GIUG".to_vec();
    deindex(&mut inp,2);

    assert_eq!(String::from_utf8(oup).unwrap(),String::from_utf8(inp).unwrap());

}
