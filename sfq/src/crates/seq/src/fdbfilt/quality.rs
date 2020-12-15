use crate::{Fdb};

impl Fdb {

    pub(crate) fn qavrg (&self, qual: &Vec<u8>, qavg: &mut Vec<f64>,  i: usize) {

        for (e,q) in qual.iter().enumerate() {
             qavg[e] = ((qavg[e] * (i-1) as f64) + *q as f64)/i as f64;
        }

    }

    pub(crate) fn illumina_8lev_map(&self, qual: &mut Vec<u8>) {
        for i in 0..qual.len() {
            let elem: u8 = qual[i];
            match elem {
                2..=9 => {qual[i] = 6;},
                11..=19 => {qual[i] = 15;},
                20..=24 => {qual[i] = 22;},
                25..=29 => {qual[i] = 27;},
                30..=34 => {qual[i] = 33;},
                35..=39 => {qual[i] = 37;},
                40.. => {qual[i] = 40;},
            }
        }
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
