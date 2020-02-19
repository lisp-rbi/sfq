
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
