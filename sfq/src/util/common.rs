use std::fs::{metadata,remove_file,remove_dir_all,create_dir};
use std::str;
use std::mem;
use regex::Regex;
use rand::prelude::*;
use fxhash::FxHashSet;

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
    let _result = match create_dir(dirname){
        Ok(_result) => _result,
        Err(_e) => panic!("Error in creating directory {}", dirname),
    };
    true
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

        }else if *i == 94u8 {
            if print == false{
                print = true;
            } else {
                print = false;
                b = e + 1;
            }
            continue;
        }

        if print == true {
            v[j] = *i;
            j += 1;
        }
    }

    //if singleton {eprintln!("Singleton is true");}

    if print == true || singleton == true {
        vec[x] = 1;
    } else {
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
    //let mut rmd = 0;
    let mut res = num;

    for i in 0..word {
        let rmd = res%4;
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


pub fn get_stats(st: &Vec<u8>) -> (usize,Vec<u8>,usize,bool,usize) {

    let stats_vec: Vec<_> = st.split(|i| *i == 94u8).collect();

    let num_of_rec = std::str::from_utf8(stats_vec[1]).unwrap().parse::<usize>().unwrap();
    let alpha      = stats_vec[2].to_owned();
    let padding    = std::str::from_utf8(stats_vec[3]).unwrap().parse::<usize>().unwrap();
    // if unpaired. model is 48u8 (false), if paired 49u8 (true)
    let model = if stats_vec[4][0] == 48u8 {false} else  {true};
    let mut lossy = 0; 
    if stats_vec.len() == 6 {
        lossy = std::str::from_utf8(stats_vec[5]).unwrap().parse::<usize>().unwrap();
    }

    (num_of_rec,alpha,padding,model,lossy)
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

/*  function to remove the number of repeated sequences in binary
 *  before: ACTTGCb0000000001
 *  after:  ACTTGC
 *  note: 98u8 is ASCII for b, 10u8 is for \n
 */
pub fn remove_cpcnt(v: &mut Vec<u8>) -> Vec<usize> {
    let mut cpcnt: Vec<usize> = Vec::new();
    let mut tmp_count: Vec<u8> = Vec::new();
    let mut result: Vec<u8> = Vec::new();
    let mut copy: bool = true;
    let mut i: usize = 1;
    for element in v.clone() {
        if element == 98u8 {
            copy = false;
            i += 1;
            continue;
        } else if element == 10u8 || i == v.len() { 
            if i == v.len() { tmp_count.push(element);}
            let count = isize::from_str_radix(std::str::from_utf8(&tmp_count).unwrap(),2).unwrap();
            cpcnt.push(count as usize);
            tmp_count = Vec::new();
            if element == 10u8 { copy = true; }
        }
        if copy == true { result.push(element); }
        else { if i < v.len() {tmp_count.push(element);} }
        i += 1;
    }
    let mut old_v = mem::replace(v, result);
    old_v.clear();
    cpcnt
}
