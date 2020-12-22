use crate::{Fdb};

impl Fdb {

    pub(crate) fn qavrg (&self, qual: &Vec<u8>, qavg: &mut Vec<f64>,  i: usize) {

        for (e,q) in qual.iter().enumerate() {
             qavg[e] = ((qavg[e] * (i-1) as f64) + *q as f64)/i as f64;
        }

    }

    pub(crate) fn illumina_8lev_map(&self, qual: &mut Vec<u8>) -> Vec<u8> {
        let mut red_qual: Vec<u8> = Vec::new();
        for i in 0..qual.len() {
            let elem: u8 = qual[i];
            match elem {
                0..=9 => {},
                10 => {red_qual.push(10);},
                11..=34 => {},
                35..=43 => {red_qual.push(39);},
                44..=52 => {red_qual.push(48);},
                53..=57 => {red_qual.push(55);},
                57..=62 => {red_qual.push(60);},
                63..=67 => {red_qual.push(66);},
                68..=72 => {red_qual.push(70);},
                73..=126 => {red_qual.push(73);},
                127u8..=std::u8::MAX => {},
            }
        }
        red_qual
    }

    /*pub(crate) fn qmax (&self, qual: &Vec<u8>, qmax: &mut Vec<u8> ) {

        for (e,q) in qual.iter().enumerate() {
             if *q > qmax[e]{
                  qmax[e] = *q;
             }
        }

    }
    pub(crate) fn qmin (&self, qual: &Vec<u8>, qmin: &mut Vec<u8>) {

        for (e,q) in qual.iter().enumerate() {
             if *q < qmin[e] {
                  qmin[e] = *q;
             }
        }

    }*/

}
