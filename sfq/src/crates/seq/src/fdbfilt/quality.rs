use crate::{Fdb};

impl Fdb {

    pub(crate) fn qavrg (&self, qual: &Vec<u8>, qavg: &mut Vec<f64>,  i: usize) {

        for (e,q) in qual.iter().enumerate() {
             qavg[e] = ((qavg[e] * (i-1) as f64) + *q as f64)/i as f64;
        }

    }

    pub(crate) fn average_qualities(&self, qual: &Vec<u8>,  num_of_copies: usize) -> Vec<u8> {

        let mut qual_matrix: Vec<_> = qual.split(|i| *i == 0u8).collect();
        qual_matrix.pop();
        assert!(qual_matrix.len() == num_of_copies);
        let mut average_qual: Vec<u8> = Vec::new();
        for i in 0..qual_matrix[0].len() {
            let mut temp: f32 = 0.0;
            for j in 0..qual_matrix.len() {
                temp += qual_matrix[j][i] as f32;
            }
            average_qual.push(((temp/(num_of_copies as f32)).round()) as u8);
        }
        average_qual
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
                58..=62 => {red_qual.push(60);},
                63..=67 => {red_qual.push(66);},
                68..=72 => {red_qual.push(70);},
                73..=126 => {red_qual.push(73);},
                127u8..=std::u8::MAX => {},
            }
        }
        red_qual
    }

}
