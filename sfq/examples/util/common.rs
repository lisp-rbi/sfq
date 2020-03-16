use std::io::{self, prelude::*, stdout, Write, Read, BufReader, BufWriter};
use std::fs::File;


pub fn make_key(pos: usize, alpha: usize, word: usize) -> String{

    let mut i = pos;
    let mut s = "".to_string();
    let mut j = word;

    while j > 0 {
        let r = i % alpha;
        s.push((r+97) as u8 as char);
        i = (i-r)/alpha;
        j=j-1;
    }

    s

}


/**
    Simple file reader for '\n' seperated list of keys
*/
pub fn get_keys (file:&str) -> Vec<Vec<u8>>{

    let fh = File::open(file).expect(&(format!("Error opening {} file",file)));
    let mut keys : Vec<Vec<u8>> = Vec::new();
    for line in BufReader::new(fh).lines() {
        keys.push(line.unwrap().as_bytes().to_vec());
    }
    keys

}




pub fn comp_wlen_alpha(v: &Vec<u8>) -> (Vec<u8>,usize) {
    let mut counter = vec![0; 127];
    let (mut cnt, mut alpha) = (0,0);
    let mut a: Vec<u8> = Vec::new();

    for i in v.iter() {
        counter[*i as usize] +=1;
    }
    for i in 0..counter.len(){
        if counter[i] > 0 {
            if i == '\n' as usize {
                cnt= counter[i] ;
            }else{
                a.push(i as u8);
                alpha+=1;
            }
        }

    }

    (a,(cnt as f64).log(alpha as f64).ceil() as usize)
}


/** Function takes `Vec<u8>` and an indicator whether it is Pair-end compression or not
    and adds an encoded intex
    Example:
    in: bbbbb\nbbbbb\nbbbbb\nbbbbb\n
    out: AAAbbbbb\nACAbbbbb\nAGAbbbbb\nATAbbbbb

    or in case of pair-end (fwd:A rev:G)

    in: bbbbb\nbbbbb\nbbbbb\nbbbbb\n
    out: AAAbbbbb\nAAGbbbbb\nACAbbbbb\nACGbbbbb
*/
pub fn index(v: &Vec<u8>, p: bool) -> Vec<u8> {

    let (alpha,wlen)= comp_wlen_alpha(&v);
    let cnt = v.iter().position(|&r| r == '\n' as u8).unwrap();

    let mut vec = vec![0u8;v.len()+ (cnt*wlen)+cnt];
    let (mut x, mut e, mut c, mut d) = (0,0,0,'A' as u8);
    let q = if p == true {1}else{2};



    let mut code = encode(x,wlen,&alpha);
    for j in code.iter(){
        vec[e] = *j;
        e+=1;
    }
    vec[e] = d;
    e+=1;


    for (p,i) in v.iter().enumerate() {
        vec[e]= *i;
        e+=1;
        if *i == '\n' as u8 && p < v.len()-1 {
            c+=q;
            if c%2 == 0 {
                x+=1;
                code = encode(x,wlen,&alpha);
                d = 'A' as u8; // fwd
            }else{
                d = 'G' as u8; // rev
            }
            for j in code.iter(){
                vec[e] = *j;
                e+=1;
            }
            vec[e] = d;
            e+=1;
        }
    }
    vec.resize(e,0x00);

    vec

}

/** Function takes `Vec<u8>` and an indicator whether it is Pair-end compression or not
    and adds an encoded intex
    Example:
    in: bbbbb\nbbbbb\nbbbbb\nbbbbb\n
    out: bbbbb\tAAA\nbbbbb\tACA\nbbbbb\tAGA\nbbbbb\tATA\n

    or in case of pair-end (fwd:A rev:G)

    in: bbbbb\nbbbbb\nbbbbb\nbbbbb\n
    out: bbbbb\tAAA\nbbbbb\tAAG\nbbbbb\tACA\nbbbbb\tACG

    NOte: importan is to have '\n' at the end.
*/
pub fn tsv_encode(v: &Vec<u8>, p: bool) -> Vec<u8> {

    let (alpha,wlen)= comp_wlen_alpha(&v);
    let cnt = v.iter().position(|&r| r == '\n' as u8).unwrap();

    let mut vec = vec![0u8;v.len()+ (cnt*wlen)+(cnt*2)];
    let (mut x, mut e, mut c, mut d) = (0,0,1,'A' as u8);
    let q = if p == true {1}else{2};
    let mut code : Vec<u8>= Vec::new();


    for (p,i) in v.iter().enumerate() {

        if *i == '\n' as u8 || p == v.len(){ //
            vec[e] = '\t' as u8;
            e+=1;
            c+=q;
            if c%2 == 0 {
                code = encode(x,wlen,&alpha);
                x+=1;
                d = 'A' as u8; // fwd
            }else{
                d = 'G' as u8; // rev
            }
            for j in code.iter(){
                vec[e] = *j;
                e+=1;
            }
            vec[e] = d;
            e+=1;
        }
        vec[e]= *i;
        e+=1;
    }
    vec.resize(e,0x00);

    vec

}



/** Function takes `Vec<u8>` and the length of an index
    and removes it from the data

    Example:
    in: AAAbbbbb\nACAbbbbb\nAGAbbbbb\nATAbbbbb
    out: bbbbb\nbbbbb\nbbbbb\nbbbbb\n
*/
pub fn deindex(v: &mut Vec<u8>, l:usize)  {

    let (mut j, mut x) = (0,l);
    for i in v.clone().iter(){

        if x == 0 {
            v[j] = *i;
            j+=1;
        }

        if x>0 {x-=1;}

        if *i == '\n' as u8 {
            x=l;

        }
    }
    v.resize(j,0x00);

}


/** Function takes value: `usize`, wordsize: `usize` and alphabet:  `&Vec<u8>` and complutes:

    encode: value ->  wordsize x alphabet

    Example:

    alphabet:  ACGT  as Vec<u8>
    wordsize:  4     as usize
    value:     27    as usize

    result:    ACGT  as Vec<u8>

*/
pub fn encode (num: usize, word: usize, alpha: &Vec<u8> ) -> Vec<u8> {

    let mut v = vec![0u8;word];
    let mut rmd = 0;
    let mut res = num;

    for i in 0..word {
        rmd = res%4;
        res = res/4;
        v[word-(i+1)] = alpha[rmd];
    }
    v
}

/** Function takes code: `&Vec<u8>` and alphabet:  `&Vec<u8>` and recomplutes the coded value (n)
    using : n = (n*alphabet_size) + alphabet(code[i])^(-1)

    Example:

    alphabet: ACGT  as Vec<u8>
    code :    ACGT  as Vec<u8>

    result(n):  27  as usize

*/
pub fn decode (code: &Vec<u8>, alpha: &Vec<u8> ) -> usize {

    let mut n = 0;
    let a = alpha.len();

    for i in 0.. code.len(){
        n = (n * a) + alpha.iter().position(|&r| r == code[i]).unwrap();
    }
    n
}

/** Function takes `Vec<u8>` and extracts bytes between every second '\t' starting with 0
    Example:
    in: hhhhhh\tbbbbbb\thhhhhh\tbbbbbb\thhhhhh\tbbbbbb\n
    out: hhhhhh\thhhhhh\thhhhhh\n
*/
pub fn parse_head(v: Vec<u8>) -> Vec<u8> {

    let mut vec = vec![0u8;v.len()];
    let mut x = 0;

    for (i,val) in v.into_iter().enumerate() {
        if val == '\t' as u8{
            break;
        }
        vec[i] = val;
        x+=1;
    }
    vec.resize(x,0x00);


    vec
}

/** Function takes `Vec<u8>` and extracts bytes between every second '\t' starting with 1
    Example:
    in: hhhhhh\tbbbbbb\thhhhhh\tbbbbbb\thhhhhh\tbbbbbb\n
    out: bbbbbb\tbbbbbb\tbbbbbb\n
*/
pub fn parse_body(v: Vec<u8>) -> Vec<u8> {

    let mut vec = vec![0u8;v.len()];
    let mut sw = false;
    let mut x = 0;

    for val in v.into_iter() {
        if val == '\t' as u8{
            sw = true;
        }
        if sw == true {
            vec[x] = val;
            x+=1;
        }
    }
    vec.resize(x,0x00);


    vec
}
