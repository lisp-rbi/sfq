use std::io::{self, prelude::*, stdout, Write, Read, BufReader, BufWriter};
use std::fs::{File,metadata,remove_file,remove_dir_all,create_dir};
use std::str;
use std::mem;
use regex::Regex;
use rand::prelude::*;
use fxhash::FxHashSet;

pub fn save_tmp(filename: &str, vec: &mut Vec<u8>) -> bool{

    let mut file = File::create(filename).expect("Unable to create file");
    for elem in vec {
        if *elem == 0u8 { 
            let result = match write!(file,"{}\n",elem){
                Ok(result) => result,
                Err(e) => panic!("Error in writing to file {}", filename),
            };
        } else {
            let result = match write!(file,"{} ",elem){
                Ok(result) => result,
                Err(e) => panic!("Error in writing to file {}", filename),
            };
        }

    }
    true
}

pub fn make_dir(dirname: &str) -> bool{
    if metadata(&dirname).is_ok() == true {
        if metadata(&dirname).unwrap().is_file() == true {
            remove_file(&dirname).unwrap();
        }else {
            remove_dir_all(&dirname).unwrap();
        }
    } else {
        eprintln!("Creating output directory {}.", dirname);
    }
    let result = match create_dir(dirname){
        Ok(result) => result,
        Err(e) => panic!("Error in creating directory {}", dirname),
    };
    true
}

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


pub fn comp_wlen(v: &Vec<usize>, a: String) -> (usize,usize) {
    let alpha = a.len();
    let cnt = *v.iter().max().unwrap() * v.len();
    //let cnt = v.iter().filter(|&r| *r == '\n' as u8).count() +1;
    (cnt ,((cnt+1) as f64).log(alpha as f64).ceil() as usize)
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

    or in case of pair-end (fwd:G rev:A)

    in: bbbbb\nbbbbb\nbbbbb\nbbbbb\n
    out: AAAbbbbb\nAAGbbbbb\nACAbbbbb\nACGbbbbb
*/

// FXME join index and hindex inot one function

pub fn index(v: &Vec<u8>, cp: &Vec<usize>) -> (Vec<u8>, usize, usize,String) {

    let alpha ="ACGT".to_string(); // this should be dynamyc

    let (cnt,wlen)= comp_wlen(&cp, alpha.clone());
        //eprintln!("Bug in common l:89");
    let mut vec = vec![0u8;v.len()+ ((cnt+1)*wlen*3)+(2*cnt)+2];

    let (mut x, mut y, r, f, t ) = (0,0,'A' as u8, 'G' as u8, '^' as u8);

    let nvec :Vec<_> = v.split(|i| *i=='\n' as u8).collect();

    let mut code = encode(x,wlen,&alpha.as_bytes().to_vec());

    for (e,i) in nvec.iter().enumerate() {
        let split :Vec<_> = i.split(|i| *i=='\t' as u8).collect();

        if split[0][split[0].len()-1] == 'R' as u8 {
            for j in code.iter(){
                vec[y] = *j;y+=1;
            }
            vec[y] = r;y+=1;
            vec[y] = t;y+=1;
        }else{
            for j in code.iter(){
                vec[y] = *j;y+=1;
            }
            vec[y] = f; y+=1;
            vec[y] = t; y+=1;
            x+=1;
            code = encode(x,wlen,&alpha.as_bytes().to_vec());
        }

        for j in split[1].iter(){
            vec[y] = *j;y+=1
        }
        if cp[e] > 1 {
            let cdcp = encode(cp[e],wlen,&alpha.as_bytes().to_vec());
            vec[y] = '^' as u8;y+=1;

            for j in cdcp.iter(){
                vec[y] = *j;y+=1
            }
        }
        vec[y] = '\n' as u8;y+=1
    }

    vec.resize(y-1,0x00);
    (vec,x,wlen, alpha)

}


pub fn hindex(v: &Vec<u8>, cp: &Vec<usize>) ->  (Vec<u8>, usize, usize,String)  {

    let alpha ="ACGT".to_string(); // this should be dynamyc
    let (cnt,wlen)= comp_wlen(&cp, alpha.clone());

    //eprintln!("Bug in common l:140 {} {}", cnt, wlen);
    let mut vec = vec![0u8;v.len()*2 + ((cnt+1)*6*wlen)+2*(cnt+2)];
    let (mut x, mut y, r, f, t ) = (0,0,'A' as u8, 'G' as u8, '^' as u8);

    let nvec :Vec<_> = v.split(|i| *i=='\n' as u8).collect();

    let mut code = encode(x,wlen,&alpha.as_bytes().to_vec());

    for (e,i) in nvec.iter().enumerate() {
        if i[i.len()-1] == 'R' as u8 {
            for j in code.iter(){
                vec[y] = *j;y+=1;
            }
            vec[y] = r;y+=1;
            vec[y] = t;y+=1;
        }else{
            for j in code.iter(){
                vec[y] = *j;y+=1;
            }
            vec[y] = f; y+=1;
            vec[y] = t; y+=1;
            x+=1;
            code = encode(x,wlen,&alpha.as_bytes().to_vec());
        }

        for j in i.iter(){
            vec[y] = *j;y+=1
        }
        if cp[e] > 1 {
            let cdcp = encode(cp[e],wlen,&alpha.as_bytes().to_vec());
            vec[y] = '^' as u8;y+=1;

            for j in cdcp.iter(){
                vec[y] = *j;y+=1
            }
        }

        vec[y] = '\n' as u8;y+=1
    }

    vec.resize(y-1,0x00);

    (vec,x,wlen, alpha)

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

        //panic!("does not work on shrinked files!!!! \n change it");

    let (alpha,wlen)= comp_wlen_alpha(&v);
    let cnt = v.iter().filter(|&r| *r == '\n' as u8).count();

    let mut vec = vec![0u8;v.len()+ (cnt*wlen)+(cnt*2)];  // BUG
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
pub fn deindex(v: &mut Vec<u8>) -> Vec<usize>  {

    let (mut j, mut x, mut print, mut singleton,  mut b) = (0,0, false,true, 0);

    let mut vec = vec![0;v.iter().filter(|&r|*r == '\n' as u8).count()+1];

    for (e,i) in v.clone().iter().enumerate(){

        if *i == 10u8 {
            singleton = false;
            if print == true{
                vec[x] = 1;
            }else{
                vec[x] = decode(&v[b..e].to_vec(), &b"ACGT".to_vec());
            }
            x+=1;
            v[j] = *i;j+=1;
            print = false;

        }else if  *i == 94u8 {
            if print == false{
                print = true;
            }else{
                print = false;
                b= e+1;
            }
            continue;
        }

        if print == true {
            v[j] = *i;j+=1;
        }
    }

    //if singleton {eprintln!("Singleton is true");}

    if print == true  || singleton == true{
        vec[x] = 1;
    }else{
        vec[x] = decode(&v[b..v.len()].to_vec(), &b"ACGT".to_vec())+1;
        //j+=1;
    }

    v.resize(j,0x00);
    vec

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


pub fn parse_codex (codex: &str) -> (usize,String) {
    let vec : Vec<_> = codex.split('|').collect();
    (vec[1].parse::<usize>().unwrap(),vec[0].to_string())

}


pub fn get_stats(st: &Vec<u8>) -> (usize,Vec<u8>,usize, bool, u32) {

    let mut stats_vec: Vec<_> = st.split(|i| *i == 94u8).collect();

    let num_of_rec = std::str::from_utf8(stats_vec[1]).unwrap().parse::<usize>().unwrap();
    let alpha      = stats_vec[2].to_owned();
    let padding    = std::str::from_utf8(stats_vec[3]).unwrap().parse::<usize>().unwrap();
    // if unpaired. model is 48u8 (false), if paired 49u8 (true)
    let model = if stats_vec[4][0] == 48u8 {false} else  {true};
    // number of lines in the first LZT, needed for targeted search of multiTrie
    let num_lzt_rec = std::str::from_utf8(stats_vec[5]).unwrap().parse::<usize>().unwrap();

    (num_of_rec,alpha,padding,model,num_lzt_rec as u32)

}

pub fn make_stats(num_of_rec: usize, alpha: String, padding: usize, model: bool) -> Vec<u8> {

    let mut vec : Vec<u8> = Vec::new();

    vec.extend(b"~~~~~^".to_vec());
    vec.extend(num_of_rec.to_string().as_bytes().to_vec());
    vec.push(94u8);
    vec.extend(alpha.as_bytes().to_vec());
    vec.push(94u8);
    vec.extend(padding.to_string().as_bytes().to_vec());
    vec.push(94u8);
    if model ==true {
        vec.push(49u8);
    }else{
        vec.push(48u8);
    }

    vec
}

pub fn parse_conditional(text: &str) -> (String,String){

    let re = Regex::new(r"(\w+)\((.*?)\)").unwrap();
    let cap = re.captures(text).unwrap();

    (cap[1].to_string(),cap[2].to_string())

}

pub fn make_rand_uvec(num: usize, max: usize) -> Vec<usize>{


    let mut  uvec = vec![0;num];
    let mut fx : FxHashSet<usize> = FxHashSet::default();
    let (mut rng, mut i) = (rand::thread_rng(), 0);

    while i < num {
        let r = rng.gen_range(1, max);
        if fx.contains(&r) {
            continue;
        }
        fx.insert(r);
        i+=1;
        uvec.push(r);
    }

    uvec
}
