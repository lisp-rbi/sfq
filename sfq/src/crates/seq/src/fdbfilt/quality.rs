use crate::{Fdb};

impl Fdb {

    pub(crate) fn qavrg (&self, qual: &Vec<u8>, qavg: &mut Vec<f64>,  i: usize) {

        for (e,q) in qual.iter().enumerate() {
             qavg[e] = ((qavg[e] * (i-1) as f64) + *q as f64)/i as f64;
        }

    }
    pub(crate) fn qmax (&self, qual: &Vec<u8>, qmax: &mut Vec<u8> ) {

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

    }

}
