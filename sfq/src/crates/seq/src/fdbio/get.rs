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


impl Get for Fdb {

    fn  get_head(self)-> Vec<u8> {
        if self.head.len()> 0{
            self.head.clone()
        }else{
            panic!("error");
        }
    }

    fn get_seq(self) -> Vec<u8> {
        if self.seq.len()> 0{
            self.seq.clone()
        }else{
            panic!("error");
        }
    }

    fn get_qual(self) -> Vec<u8> {
        if self.qual.len()> 0{
            self.qual.clone()
        }else{
            panic!("error");
        }
    }

    fn get_fastq(&self) -> Vec<u8> {

        let count = self.head.iter().filter(|&n| *n == 10u8).count() +1;
        let len = self.head.len()+self.seq.len()+self.qual.len()+ (count*2)+2;
        let mut vec : Vec<u8> = vec![0u8; len];
        let (mut i, mut q, mut s, mut h, mut sw,  mut c) = (0,0,0,0,0,0u8);


        loop {
            match sw {
                0 =>{
                    c = self.head[h];
                    h+=1;
                    if h == self.head.len() {
                        vec[i] = c;
                        i+=1;
                        c = 10u8;
                    }
                },
                1 => {
                    c= self.seq[s];
                    s+=1;
                    if s == self.seq.len() {
                        vec[i] = c;
                        i+=1;
                        c = 10u8;
                    }
                },
                _ => {
                    c= self.qual[q];
                    q+=1;
                    if q == self.qual.len() {
                        vec[i] = c;
                        break;
                    }
                }
            }

            if  c  == 10u8 {
                if sw == 1 {
                    vec[i]= c;
                    i+=1;
                    vec[i]='+' as u8;
                    i+=1;
                };
                sw+=1;
                if sw == 3 {sw = 0};
            }
            vec[i] = c;
            i+=1;
        }
        vec
    }


    fn get_fasta(&self) -> Vec<u8> {

        let len = self.head.len()+self.seq.len() + 1 ;
        let mut vec : Vec<u8> = vec![0u8; len];
        let (mut i, mut s, mut h, mut sw,  mut c) = (0,0,0,0,0u8);

        loop {
            match sw {
                0 =>{
                    c = self.head[h];
                    h+=1;
                    if h == self.head.len() {
                        vec[i] = c;
                        i+=1;
                        c = 10u8;
                    }
                },
                _ => {
                    c= self.seq[s];
                    s+=1;
                    if s == self.seq.len() {
                        vec[i] = c;
                        break;
                    }
                },
            }

            if  c  == 10u8 {
                sw+=1;
                if sw == 2 {sw = 0};
            }
            vec[i] = c;
            i+=1;
        }
        vec
    }

    fn get_tsv(&self, model: &str) -> Vec<u8> {

        let (mut hlt, mut slt, mut qlt) = (0,0,0);
        let (mut i, mut q, mut s, mut h, mut sw,  mut c) = (0,0,0,0,0,0u8);



        if model.contains("q"){
            qlt=self.qual.len();
            sw = 2;
        }
        if model.contains("s"){
            slt=self.seq.len();
            sw = 1;
        }
        if model.contains("h") {
            hlt = self.head.len();
            sw = 0;
        }


        let len = slt+hlt+qlt+2;
        let mut vec : Vec<u8> = vec![0u8; len];


        loop {
            match sw {
                0 =>{

                        c = self.head[h];
                        h+=1;
                        if h == hlt {
                            vec[i] = c;
                            i+=1;
                            if qlt > 0  || slt > 0{
                                c = 10u8;
                            }else{
                                break;
                            }
                        }

                },
                1 => {

                        c= self.seq[s];
                        s+=1;
                        if s == slt {
                            vec[i] = c;
                            i+=1;
                            if qlt > 0 {
                                c = 10u8;
                            }else{
                                break;
                            }
                        }

                },
                _ => {

                        c= self.qual[q];
                        q+=1;
                        if q == qlt {
                            vec[i] = c;
                            i+=1;
                            break;
                        }

                }
            }
            if  c  == 10u8  {

                if slt == 0 {sw+=2} else{sw+=1};
                //println!("{}  -  {} {} {} {} ",c as char, sw , qlt , slt , hlt );
                if (sw >= 3 && qlt > 0 ) || (sw == 2 && qlt == 0) {
                    if hlt == 0 {sw = 1} else{sw=0};
                    c= 10u8;

                }else{
                    c= '\t' as u8;
                };

            }
            //println!("{},{}.", i,c as char );
            vec[i] = c;
            i+=1;
        }
        vec.resize(i,  0x00);
        vec

    }
}
