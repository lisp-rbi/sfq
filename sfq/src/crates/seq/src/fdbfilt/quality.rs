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
                0..=1 => {},
                2..=9 => {red_qual.push(6);},
                10 => {red_qual.push(10);},
                11..=19 => {red_qual.push(15);},
                20..=24 => {red_qual.push(22);},
                25..=29 => {red_qual.push(27);},
                30..=34 => {red_qual.push(33);},
                35..=39 => {red_qual.push(37);},
                40..=255 => {red_qual.push(40);},
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
