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

use crate::{Get, Fdb};
use std::str;

impl Get for Fdb {

    fn  get_head(&self)-> Vec<u8> {
        if self.head.len()> 0{
            self.head.clone()
        }else{
            panic!("error");
        }
    }

    fn get_seq(&self) -> Vec<u8> {
        if self.seq.len()> 0{
            self.seq.clone()
        }else{
            panic!("error");
        }
    }

    fn get_qual(&self) -> Vec<u8> {
        if self.qual.len()> 0{
            self.qual.clone()
        }else{
            panic!("error");
        }
    }

    fn get_model(&self) -> bool {
        self.paired
    }

    fn get_fastq(&self) -> Vec<u8> {

        let (hlt, slt, qlt) = (self.head.len(),self.seq.len(),self.qual.len());
        let (mut i, mut q, mut s, mut h, mut x, mut c) = (0,0,0,0,0,0u8);

        let count = self.head.iter().filter(|&n| *n == 10u8).count() +1;
        let len = hlt + slt + qlt + (count*2) + 20;
        let mut vec: Vec<u8> = vec![0u8; len];


        loop{
            match i {
                0 => {
                    c = self.head[h];h+=1;
                },
                1 => {
                    c = self.seq[s];s+=1;
                },
                2 => {
                    c = self.qual[q];q+=1;
                }
                _ => {
                    break;
                }
            }

            if c == 10u8 || h == hlt || s == slt || q == qlt {
                if h == hlt || s == slt || q == qlt {
                    vec[x] = c;
                    x += 1;
                    if h == hlt {h = 0;};
                    if s == slt {s = 0;};
                    if q == qlt {i = 9;};
                    c=10u8;

                }
                if i < 2 || i == 9 {
                    i += 1;
                } else {
                    i = 0;
                }
                if i == 2 {
                    vec[x] = 10u8;x+=1;
                    vec[x] = '+' as u8;x+=1;
                }
            }
            vec[x] = c;
            x += 1;
        }

        vec.resize(x-1, 0x00);  // -1 rm \n
        vec
    }

    fn get_fasta(&self) -> Vec<u8> {


        let ( hlt,  slt) = (self.head.len(),self.seq.len());
        let (mut i, mut s, mut h, mut x,  mut c) = (0,0,0,0,0u8);

        let len = hlt+slt+2;
        let mut vec : Vec<u8> = vec![0u8; len];


        loop{
            match i {
                0 => {
                    c = self.head[h];h+=1;
                },
                1 => {
                    c = self.seq[s];s+=1;
                },
                _ => {
                    break;
                }
            }
            if c == 10u8  || h == hlt || s == slt{
                if h == hlt || s == slt {
                    vec[x] = c;
                    x+=1;
                    if h == hlt {h=0;};
                    if s == slt {i=9;};
                    c=10u8;

                }
                if i  < 1 || i==9 {
                    i+=1;
                }else{
                    i=0;
                }
            }
            vec[x] = c;
            x+=1;
        }
        vec.resize(x-1, 0x00);// -1 rm \n

        vec
    }

    fn get_tsv(&self, model: &str) -> Vec<u8> {

        let (mut hlt, mut slt, mut qlt) = (1,1,1);
        let (mut i, mut q, mut s, mut h, mut x,  mut c) = (0,0,0,0,0,0u8);

        let mvec :Vec<_> = model.split(|i| i=='+').collect();
        let mut codec = vec![0;mvec.len()];

        for (e,i) in mvec.iter().enumerate() {

            match *i {
                "q" => {
                    qlt=self.qual.len();
                    codec[e] = 2;
                },
                "h" => {
                    hlt = self.head.len();
                    codec[e] = 0;
                },
                "s" =>{
                    slt=self.seq.len();
                    codec[e] = 1;
                },
                _ => {
                    panic!{"{} symbol not recognized\n", i};
                }

            }
        }

        let len = slt+hlt+qlt;
        //eprintln!("get.rs: 183-FXME: {}", len);
        let mut vec : Vec<u8> = vec![0u8; len*4];


        loop{
            match codec[i] {
                0 => {
                    c = self.head[h];h+=1;
                },
                1 => {
                    c = self.seq[s];s+=1;
                },
                2 => {
                    c = self.qual[q];q+=1;
                }
                _ => {
                    break;
                }
            }

            if c == 10u8  || h == hlt || s == slt || q == qlt{
                if h == hlt || s == slt || q == qlt {
                    if i > 0 {
                        codec[i-1] = 10;
                    }else if i==0 {
                        codec[i] = 10;
                    }
                    vec[x] = c;//'-' as u8 ; // c
                    x+=1;
                    if h == hlt {h=0};
                    if s == slt {s=0};
                    if q == qlt {q=0};
                }
                if i  < codec.len()-1  {
                    i+=1;
                    c=9u8;
                }else{
                    i=0;
                }
            }
            vec[x] = c;
            x+=1;
        }


        vec.resize(x-1,  0x00);// -1 rm \n
        vec

    }


    fn get_cpcnt (&self) -> Vec<usize> {
        self.cpcnt.clone()
    }

    fn get_numrec(&self)-> usize {
        self.numrec
    }


}
