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


use crate::{Fdb,Get};
use crate::util::error::Error;
use std::io::{ prelude::*,  Write};
use std::string::String;
use std::str;
use std::fs;

// function to read fastq files
impl Fdb{

    pub fn fastq_up<R: BufRead>(&mut self, fwd_reader: R, rev_reader: R, outdir: &str, output: &str) -> Result<bool,Error> {

        let mut cnt=0;
        let mut r: usize = 1;
        let tmp_head = format!("{}/{}.head.tmp", outdir, output);
        let tmp_seq = format!("{}/{}.seq.tmp", outdir, output);
        let tmp_qual = format!("{}/{}.qual.tmp", outdir, output);
        let mut head_writer = self.make_append_writer(&tmp_head);
        let mut seq_writer = self.make_append_writer(&tmp_seq);
        let mut qual_writer = self.make_append_writer(&tmp_qual);

        let fwd_lines = fwd_reader.lines().map(|l| l.unwrap());
        let mut rev_lines = rev_reader.lines().map(|l| l.unwrap());
        let (_count, wlen) = self.comp_wlen();

        for fwd_line in fwd_lines {
            if  cnt == 0 {
                let mut fwd_head = String::from("");
                fwd_head.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                fwd_head.push_str("G^");
                fwd_head.push_str(&fwd_line);
                if self.paired == true {fwd_head.push_str("F\0\n");}
                else {fwd_head.push_str("\0\n");}
                head_writer.write_all(&fwd_head.as_bytes()).expect("writing error!");
                if self.paired == true {
                    let mut rev_head = String::from("");
                    rev_head.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                    rev_head.push_str("A^");
                    let rev_line = match rev_lines.next() {
                        Some(p) => p,
                        None => {cnt += 1; continue;},
                    };
                    rev_head.push_str(&rev_line);
                    rev_head.push_str("R\0\n");
                    head_writer.write_all(&rev_head.as_bytes()).expect("writing error!");
                }
                cnt += 1;
                continue;
            } else if cnt == 1 {
                let mut fwd_seq = String::from("");
                fwd_seq.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                fwd_seq.push_str("G^");
                fwd_seq.push_str(&fwd_line);
                fwd_seq.push_str("\0\n");
                //if ((fwd_seq.len() - 1) as u32) > max_seq_length {max_seq_length = (fwd_seq.len()-1) as u32;}
                if r == 1 {self.line_length += fwd_seq.len()-1;}
                //lengths.push(fwd_seq.len()-1);
                seq_writer.write_all(&fwd_seq.as_bytes()).expect("writing error!");
                if self.paired == true {
                    let mut rev_seq = String::from("");
                    rev_seq.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                    rev_seq.push_str("A^");
                    let rev_line = match rev_lines.next() {
                        Some(p) => self.revcomp(p),
                        None => {cnt += 1; continue;},
                    };
                    rev_seq.push_str(&rev_line);
                    rev_seq.push_str("\0\n");
                    seq_writer.write_all(&rev_seq.as_bytes()).expect("writing error!");
                }
                cnt += 1;
                continue;
            } else if cnt == 2 {
                if self.paired == true {
                    let _rev_line = match rev_lines.next() {
                        Some(p) => p,
                        None => "0".to_string(),
                    };
                }
                cnt += 1;
                continue;
            } else if cnt == 3 {
                let mut fwd_qual = String::from("");
                fwd_qual.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                fwd_qual.push_str("G^");
                fwd_qual.push_str(&fwd_line);
                fwd_qual.push_str("\0\n");
                qual_writer.write_all(&fwd_qual.as_bytes()).expect("writing error!");
                if self.paired == true {
                    let mut rev_qual = String::from("");
                    rev_qual.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                    rev_qual.push_str("A^");
                    let rev_line = match rev_lines.next() {
                        Some(p) => p,
                        None => {r += 1; cnt = 0; continue;},
                    };
                    rev_qual.push_str(&rev_line);
                    rev_qual.push_str("\0\n");
                    qual_writer.write_all(&rev_qual.as_bytes()).expect("writing error!");
                } 
                r += 1;
                cnt = 0;
                continue;
            }
        }

        // in case fwd file is shorter than rev file, loop over remaining rev file
        if self.paired == true {
            loop {
                match rev_lines.next() {
                    Some(rev_line) => {
                        if  cnt == 0 {
                            let mut rev_head = String::from("");
                            rev_head.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                            rev_head.push_str("A^");
                            rev_head.push_str(&rev_line);
                            rev_head.push_str("R\0\n");
                            head_writer.write_all(&rev_head.as_bytes()).expect("writing error!");
                            cnt += 1;
                            continue;
                        } else if cnt == 1 {
                            let mut rev_seq = String::from("");
                            rev_seq.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                            rev_seq.push_str("A^");
                            rev_seq.push_str(&self.revcomp(rev_line));
                            rev_seq.push_str("\0\n");
                            seq_writer.write_all(&rev_seq.as_bytes()).expect("writing error!");
                            cnt += 1;
                            continue;
                        } else if cnt == 2 {
                            cnt += 1;
                            continue;
                        } else if cnt == 3 {
                            let mut rev_qual = String::from("");
                            rev_qual.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                            rev_qual.push_str("A^");
                            rev_qual.push_str(&rev_line);
                            rev_qual.push_str("\0\n");
                            qual_writer.write_all(&rev_qual.as_bytes()).expect("writing error!");
                            r += 1;
                            cnt = 0;
                            continue;
                        }
                    }
                    None => {break;}
                }
            }
        }

        let stats = self.make_stats(wlen);
        head_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes()).expect("writing error!");
        seq_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes()).expect("writing error!");
        qual_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes()).expect("writing error!");

        //println!("{}:{}\n{:?}\n{:?}\n{:?}", self.seq.len(), self.seq[self.seq.len()-1], String::from_utf8(self.seq.clone()), String::from_utf8(self.qual.clone()), String::from_utf8(self.head.clone()));
        if self.paired == false {self.rm_file("dummy.txt");}

        if self.numrec > 0 {Ok(true)}
        else{Ok(false)}
    }

    pub fn fastq_up_lossy<R: BufRead>(&mut self, fwd_reader: R, rev_reader: R, outdir: &str, output: &str) -> Result<bool,Error> {

        let mut cnt=0;
        let mut r: usize = 1;
        let tmp_lossy = format!("{}/{}.lossy.tmp", outdir, output);
        let mut lossy_writer = self.make_append_writer(&tmp_lossy);

        let fwd_lines = fwd_reader.lines().map(|l| l.unwrap());
        let mut rev_lines = rev_reader.lines().map(|l| l.unwrap());
        let (_count, wlen) = self.comp_wlen();
        let mut line_string = String::from("");

        for fwd_line in fwd_lines {
            if  cnt == 0 {
                cnt += 1;
                continue;
            } else if cnt == 1 {
                line_string = String::from("");
                line_string.push_str(&fwd_line);
                line_string.push_str(" ");
                cnt += 1;
                continue;
            } else if cnt == 2 {
                cnt += 1;
                continue;
            } else if cnt == 3 {
                line_string.push_str(&fwd_line);
                r += 1;
                cnt = 0;
                if self.paired == true {
                    line_string.push_str(" ");
                    loop {
                        match rev_lines.next() {
                            Some(rev_line) => {
                                if  cnt == 0 {
                                    cnt += 1;
                                    continue;
                                } else if cnt == 1 {
                                    line_string.push_str(&self.revcomp(rev_line));
                                    line_string.push_str(" ");
                                    cnt += 1;
                                    continue;
                                } else if cnt == 2 {
                                    cnt += 1;
                                    continue;
                                } else if cnt == 3 {
                                    line_string.push_str(&rev_line);
                                    line_string.push_str("\n");
                                    cnt = 0;
                                    break;
                                }
                            }
                            None => {break;}
                        }
                    }
                } else {line_string.push_str("\n");}
                lossy_writer.write_all(&line_string.as_bytes()).expect("writing error!");
                continue;
            }
        }

        lossy_writer.flush().expect("Error in flushing");
        let stats = self.make_stats(wlen);
        //self.sort_file(&tmp_lossy,&outdir).expect("Error in sorting file!");
        if self.sort_lines(&tmp_lossy,outdir) == false {panic!("Sorting not successful");}
        let tmp_seq_name: &str = &tmp_lossy.replace("lossy","seq");
        let tmp_qual_name: &str = &tmp_lossy.replace("lossy","qual");
        let mut seq_writer = self.make_append_writer(&tmp_seq_name);
        let mut qual_writer = self.make_append_writer(&tmp_qual_name);
        if  self.separate_lossy_tmp(&tmp_lossy,seq_writer,qual_writer,wlen,stats) == false {panic!("Error!");}
        if self.paired == false {self.rm_file("dummy.txt");}

        if self.numrec > 0 {Ok(true)}
        else{Ok(false)}
    }

    pub fn separate_lossy_tmp<W: Write>(&mut self, filename: &str, mut seq_writer: W, mut qual_writer: W, wlen: usize, stats: Vec<u8>) -> bool {
        let mut lossy_reader = self.make_reader(filename);
        let lines = lossy_reader.lines().map(|l| l.unwrap());
        let mut r: usize = 1;
        for line in lines {
            let mut sequence = String::from("");
            let mut quality = String::from("");
            sequence.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
            quality.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
            sequence.push_str("G^");
            quality.push_str("G^");
            let line_components: Vec<&str> = line.split(" ").collect();
            sequence.push_str(&line_components[0]);
            sequence.push_str("\n");
            let mut u8_quality = line_components[1].as_bytes();
            let red_u8_quality = self.illumina_8lev_map(&mut u8_quality.to_vec());
            quality.push_str(str::from_utf8(&red_u8_quality).unwrap());
            quality.push_str("\n");
            if self.paired == true {
                sequence.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                quality.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                sequence.push_str("A^");
                quality.push_str("A^");
                sequence.push_str(&line_components[2]);
                sequence.push_str("\n");
                let mut u8_quality = line_components[3].as_bytes();
                let red_u8_quality = self.illumina_8lev_map(&mut u8_quality.to_vec());
                quality.push_str(str::from_utf8(&red_u8_quality).unwrap());
                quality.push_str("\n");
            }
            seq_writer.write_all(&sequence.as_bytes()).expect("Writing error!");
            qual_writer.write_all(&quality.as_bytes()).expect("Writing error!");
            r += 1;
        }
        seq_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes()).expect("writing error!");
        qual_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes()).expect("writing error!");
        seq_writer.flush().expect("Error in flushing");
        qual_writer.flush().expect("Error in flushing");
        fs::remove_file(filename).expect("Error in removing file!"); 
        true
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
