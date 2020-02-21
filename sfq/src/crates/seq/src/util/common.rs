/*
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * This software consists of voluntary contributions made by many individuals
 * and is licensed under the MIT license. For more information, see
 * <http://www.doctrine-project.org>.
 */

use crate::Fdb;


impl Fdb{

    pub fn clear(&mut self) -> &mut Self {
        self.head.resize(0,0x00);
        self.seq.resize(0,0x00);
        self.qual.resize(0,0x00);
        self.numrec = 0;

        self
    }

    pub fn revcomp(&self, s: String) -> String {

        s.chars()
        .map(|x| match x {
            'a' | 'A' => 'T',
            't' | 'T' => 'A',
            'g' | 'G' => 'C',
            'c' | 'C' => 'G',
            _         => x
        }).rev().collect()

    }

    pub fn  encode (
        self,
        a: usize,
        b: usize,
        c: &Vec<u8>) -> Vec<u8>{

        let mut v = vec![0u8;b];
        let mut rmd = 0;
        let mut res = a;

        for i in 0..b {
            rmd = res%4;
            res = res/4;
            v[b-(i+1)] = c[rmd];
        }
        v
    }


    pub fn sort_by (
        &mut self,
        key_vec:  &mut Vec<u8>,
        prim_vec: &mut Vec<u8>,
        sec_vec:  &mut Vec<u8>) -> bool {

        if self.qual.len() == 0 {
            panic!("sort_by() only works when fastq file is loaded properly!");
        }

        let count = self.head.iter().filter(|&n| *n == 10u8).count()+1;
        let wln = (count as f64).log(10.0).ceil() as usize;

        let (mut x, mut y) = (0,0);
        let mut key: Vec<u8> = vec![0u8; key_vec.len()+(count+1)*(wln+1)];


        for i in key_vec.clone().into_iter() {
            if i == 10u8 {
                key[x] = 9u8;
                x=x+1;
                let tmp = y.to_string().into_bytes();
                y+=1;
                for j in tmp.into_iter(){
                    key[x] = j;
                    x=x+1;
                }
            }
            key[x] = i;
            x=x+1;
        }
        key[x] = 9u8;
        x=x+1;
        let tmp = y.to_string().into_bytes();
        for j in tmp.into_iter(){
            key[x] = j;
            x=x+1;
        }

        key.resize(x,0x00);

        let mut key_sorted: Vec<_> = key.split(|i| *i == 10u8).collect();
        key_sorted.sort();

        let pvec = prim_vec.clone();
        let svec = sec_vec.clone();

        let prim_vec_tmp: Vec<_> = pvec.split(|i| *i == 10u8).collect();
        let sec_vec_tmp : Vec<_> = svec.split(|i| *i == 10u8).collect();
        let (mut k, mut p, mut s) = (0,0,0);

        for i in key_sorted.into_iter() {
            let kt: Vec<_> = i.split(|i| *i == 9u8).collect();
            for j in kt[0].into_iter(){
                key_vec[k]=*j;
                k=k+1;
            }
            if k < key_vec.len(){
                key_vec[k] = 10u8;
                k+=1;
            }

            let pos = String::from_utf8(kt[1].to_owned()).unwrap().parse::<usize>().unwrap();
            for j in prim_vec_tmp[pos].into_iter(){
                prim_vec[p]=*j;
                p=p+1;
            }
            if p < prim_vec.len(){
                prim_vec[p] = 10u8;
                p+=1;
            }

            for j in sec_vec_tmp[pos].into_iter(){
                sec_vec[s]=*j;
                s=s+1;
            }
            if s < sec_vec.len(){
                sec_vec[s] = 10u8;
                s+=1;
            }
        }
        true
    }


}
