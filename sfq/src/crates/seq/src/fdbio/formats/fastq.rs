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


use crate::{Fdb,Get,Save};
use crate::util::error::Error;
use std::io::{ prelude::*,  Write};
use std::string::String;
use std::str;

// function to read fastq files
impl Fdb{

/*    pub fn fq_write_line(&self, writer: BufWriter, line: str, lt: str, direction: str, r: u64, wlen: usize) {
        let mut line_vec: Vec<u8> = Vec::new();
        if direction == "fwd" {
            line_vec.extend(self.encode(r,wlen));
            line_vec.extend(b"G");
        } else if direction == "rev" {
            line_vec.extend(self.encode(r+1,wlen));
            line_vec.extend(b"A");
        }
        
        line_vec.extend(b"^");
        line_vec.extend(line.as_bytes());
        if (self.paired == true) && (lt == "head") && (direction == "fwd") {line_vec.extend(b"F\0");}
        else if (lt == "head") && (direction == "rev") {line_vec.extend(b"R\0");}
        else {line_vec.extend(b"\0");}
        for elem in line_vec{
            if elem == 0 {write!(writer,"{:?}\n",elem);} 
            else {write!(writer,"{:?} ",elem);}
        }
    }
*/
    pub fn fastq_up<R: BufRead>(&mut self, fwd_reader: R, rev_reader: R, outdir: &str, output: &str) -> Result<bool,Error> {

        let mut cnt=0;
        let mut r: usize = 1;
        let tmp_head = format!("{}/{}.head.tmp", outdir, output);
        let tmp_seq = format!("{}/{}.seq.tmp", outdir, output);
        let tmp_qual = format!("{}/{}.qual.tmp", outdir, output);
        let mut head_writer = self.make_append_writer(&tmp_head);
        let mut seq_writer = self.make_append_writer(&tmp_seq);
        let mut qual_writer = self.make_append_writer(&tmp_qual);

        let mut fwd_lines = fwd_reader.lines().map(|l| l.unwrap());
        let mut rev_lines = rev_reader.lines().map(|l| l.unwrap());
        let (count, wlen) = self.comp_wlen();

        for fwd_line in fwd_lines {
            if  cnt == 0 {
                let mut fwd_head = String::from("");
                fwd_head.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                fwd_head.push_str("G^");
                fwd_head.push_str(&fwd_line);
                if self.paired == true {fwd_head.push_str("F\0\n");}
                else {fwd_head.push_str("\0\n");}
                head_writer.write_all(&fwd_head.as_bytes());
                if self.paired == true {
                    let mut rev_head = String::from("");
                    rev_head.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                    rev_head.push_str("A^");
                    let rev_line = match rev_lines.next() {
                        Some(p) => p,
                        None => "0".to_string(),
                    };
                    rev_head.push_str(&rev_line);
                    rev_head.push_str("R\0\n");
                    head_writer.write_all(&rev_head.as_bytes());
                }
                cnt = cnt+1;
                continue;
            }else if cnt == 1 {
                let mut fwd_seq = String::from("");
                fwd_seq.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                fwd_seq.push_str("G^");
                fwd_seq.push_str(&fwd_line);
                fwd_seq.push_str("\0\n");
                if r == 1 {self.line_length += fwd_seq.len()-1;}
                seq_writer.write_all(&fwd_seq.as_bytes());
                if self.paired == true {
                    let mut rev_seq = String::from("");
                    rev_seq.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                    rev_seq.push_str("A^");
                    let rev_line = match rev_lines.next() {
                        Some(p) => self.revcomp(p),
                        None => "0".to_string(),
                    };
                    rev_seq.push_str(&rev_line);
                    rev_seq.push_str("\0\n");
                    seq_writer.write_all(&rev_seq.as_bytes());
                }
                cnt = cnt+1;
                continue;
            }else if cnt == 2 {
                if self.paired == true {
                    let rev_line = match rev_lines.next() {
                        Some(p) => p,
                        None => "0".to_string(),
                    };
                }
                cnt += 1;
                continue;
            }else if cnt == 3 {
                let mut fwd_qual = String::from("");
                fwd_qual.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                fwd_qual.push_str("G^");
                fwd_qual.push_str(&fwd_line);
                fwd_qual.push_str("\0\n");
                qual_writer.write_all(&fwd_qual.as_bytes());
                if self.paired == true {
                    let mut rev_qual = String::from("");
                    rev_qual.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                    rev_qual.push_str("A^");
                    let rev_line = match rev_lines.next() {
                        Some(p) => p,
                        None => "0".to_string(),
                    };
                    rev_qual.push_str(&rev_line);
                    rev_qual.push_str("\0\n");
                    qual_writer.write_all(&rev_qual.as_bytes());
                } 
                r += 1;
                cnt = 0;
                continue;
            }
        }

        let stats = self.make_stats(wlen);
        head_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes());
        seq_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes());
        qual_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes());

        //println!("{}:{}\n{:?}\n{:?}\n{:?}", self.seq.len(), self.seq[self.seq.len()-1], String::from_utf8(self.seq.clone()), String::from_utf8(self.qual.clone()), String::from_utf8(self.head.clone()));
        if self.paired == false {self.rm_file("dummy.txt");}

        if self.numrec > 0 {Ok(true)}
        else{Ok(false)}
    }


    pub fn fastq_dw<W: Write> (&mut self, mut writer:  W) -> Result<bool,Error>  {
        let (mut sw , mut ssw, mut x, mut y, mut bw)= (0u8, true, 0, 1000, 0);
        let mut buff = vec![0u8; y];

        //writer.write_all(&self.get_fastq());


        for ch in self.get_fastq().iter() {

            match *ch {

                10u8 => {
                    if bw == 1 && ssw == true {
                        buff.resize(x,0x00);
                        writer.write_all(&self.revcomp(String::from_utf8(buff.clone()).unwrap()).as_bytes()).unwrap();
                        y=x;
                        x=0;
                    }
                    write!(writer, "{}", *ch as char).unwrap();
                    bw+=1;
                    if sw == 82u8 {
                        ssw = true;
                    }else{
                        ssw = false;
                    }
                    if bw == 4{bw = 0}
                },
                _   => {
                    if bw == 1 && ssw == true{
                        if x == y{
                            buff.extend(vec![0u8;y]);
                            y=y*2;
                        }
                        buff[x] = *ch;
                        x+=1;

                    }else {
                        write!(writer, "{}", *ch as char).unwrap();
                    }
                }
            }
            sw = *ch;
        }
        write!(writer, "{}", 10u8 as char).unwrap();

        Ok(true)

    }
}
