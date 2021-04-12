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
use std::io::{prelude::*,  Write};
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
            if cnt == 0 {
                if self.lossy < 2 {
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
                } else if self.lossy == 2 {
                     if self.paired == true {
                         let _rev_line = match rev_lines.next() {
                             Some(p) => p,
                             None => "0".to_string(),
                         };
                     }
                }
                cnt += 1;
                continue;
            } else if cnt == 1 {
                let mut fwd_seq = String::from("");
                fwd_seq.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                fwd_seq.push_str("G^");
                fwd_seq.push_str(&fwd_line);
                fwd_seq.push_str("\0\n");
                if r == 1 {self.line_length += fwd_seq.len()-1;}
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
                if self.lossy > 0 {
                    let mut u8_quality = fwd_line.as_bytes();
                    let red_u8_quality = self.illumina_8lev_map(&mut u8_quality.to_vec());
                    fwd_qual.push_str(str::from_utf8(&red_u8_quality).unwrap());
                } else { fwd_qual.push_str(&fwd_line); }
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
                    if self.lossy > 0 {
                        let mut u8_quality = rev_line.as_bytes();
                        let red_u8_quality = self.illumina_8lev_map(&mut u8_quality.to_vec());
                        rev_qual.push_str(str::from_utf8(&red_u8_quality).unwrap());
                    } else { rev_qual.push_str(&rev_line); }
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
                            if self.lossy < 2 {
                                let mut rev_head = String::from("");
                                rev_head.push_str(str::from_utf8(&self.encode(r,wlen)).unwrap());
                                rev_head.push_str("A^");
                                rev_head.push_str(&rev_line);
                                rev_head.push_str("R\0\n");
                                head_writer.write_all(&rev_head.as_bytes()).expect("writing error!");
                            }
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
                            if self.lossy > 0 {
                                let mut u8_quality = rev_line.as_bytes();
                                let red_u8_quality = self.illumina_8lev_map(&mut u8_quality.to_vec());
                                rev_qual.push_str(str::from_utf8(&red_u8_quality).unwrap());
                            } else { rev_qual.push_str(&rev_line); }
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
        if self.lossy < 2 {head_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes()).expect("writing error!");}
        seq_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes()).expect("writing error!");
        qual_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes()).expect("writing error!");
        head_writer.flush().expect("Error in flushing");
        seq_writer.flush().expect("Error in flushing");
        qual_writer.flush().expect("Error in flushing");

        //println!("{}:{}\n{:?}\n{:?}\n{:?}", self.seq.len(), self.seq[self.seq.len()-1], String::from_utf8(self.seq.clone()), String::from_utf8(self.qual.clone()), String::from_utf8(self.head.clone()));
        if self.paired == false {self.rm_file("dummy.txt");}

        if self.numrec > 0 {Ok(true)}
        else{Ok(false)}
    }

    pub fn fastq_up_lossy<R: BufRead>(&mut self, fwd_reader: R, rev_reader: R, outdir: &str, output: &str) -> Result<bool,Error> {

        let mut cnt=0;
        let tmp_lossy = format!("{}/{}.lossy.tmp", outdir, output);
        let mut lossy_writer = self.make_append_writer(&tmp_lossy);

        let fwd_lines = fwd_reader.lines().map(|l| l.unwrap());
        let mut rev_lines = rev_reader.lines().map(|l| l.unwrap());
        let (_count, wlen) = self.comp_wlen();
        let mut line_string = String::from("");

        for fwd_line in fwd_lines {
            if  cnt == 0 {
                if self.paired == true {
                    let _rev_line = match rev_lines.next() {
                        Some(p) => p,
                        None => "0".to_string(),
                    };
                }
                cnt += 1;
                continue;
            } else if cnt == 1 {
                line_string = String::from("");
                line_string.push_str(&fwd_line);
                line_string.push_str(" ");
                if self.paired == true {
                    let rev_line = match rev_lines.next() {
                        Some(p) => self.revcomp(p),
                        None => {cnt += 1; continue;},
                    };
                    line_string.push_str(&rev_line);
                    line_string.push_str(" ");
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
                line_string.push_str(&fwd_line);
                if self.paired == true {
                    line_string.push_str(" ");
                    let rev_line = match rev_lines.next() {
                        Some(p) => p,
                        None => {cnt = 0; continue;},
                    };
                    line_string.push_str(&rev_line);
                }
                line_string.push_str("\n");
                lossy_writer.write_all(&line_string.as_bytes()).expect("writing error!");
                cnt = 0;
                continue;
            }
        }

        lossy_writer.flush().expect("Error in flushing");
        if self.sort_lines(&tmp_lossy,outdir) == false {panic!("Sorting not successful");}
        let tmp_seq_name: &str = &tmp_lossy.replace("lossy","seq");
        let tmp_qual_name: &str = &tmp_lossy.replace("lossy","qual");
        let mut seq_writer = self.make_append_writer(&tmp_seq_name);
        if self.lossy == 3 || self.lossy == 4 { 
            let mut qual_writer = self.make_append_writer(&tmp_qual_name);
            if self.separate_lossy_tmp(&tmp_lossy,seq_writer,qual_writer,wlen) == false {panic!("Error!");}
        }
        else if self.lossy > 4 {
            if self.prepare_very_lossy_tmp(&tmp_lossy,seq_writer,tmp_seq_name,outdir,wlen) == false {panic!("Error!");}
        }
        if self.paired == false {self.rm_file("dummy.txt");}

        if self.numrec > 0 {Ok(true)}
        else{Ok(false)}
    }

    pub fn separate_lossy_tmp<W: Write>(&mut self, filename: &str, mut seq_writer: W, mut qual_writer: W, wlen: usize) -> bool {
        let mut lossy_reader = self.make_reader(filename);
        let lines = lossy_reader.lines().map(|l| l.unwrap());
        let mut r: usize = 0;
        let mut first_line: usize = 1;
        let mut old_fwd_seq: Vec<u8> = Vec::new();
        let mut old_rev_seq: Vec<u8> = Vec::new();
        let mut fwd_qualities: Vec<u8> = Vec::new();
        let mut rev_qualities: Vec<u8> = Vec::new();
        let mut num_of_copies: usize = 0;
        for line in lines {
            let line_components: Vec<&str> = line.split(" ").collect();
            let fwd_seq_vector = line_components[0].as_bytes().to_vec();
            let mut rev_seq_vector: Vec<u8> = Vec::new();
            if self.paired == true {rev_seq_vector = line_components[1].as_bytes().to_vec();}
            let mut avrg_fwd_qual: Vec<u8> = Vec::new();
            let mut avrg_rev_qual: Vec<u8> = Vec::new();
            let repeated_sequence: bool = match self.paired {
                true  => {self.compare_vslice(&old_fwd_seq,&fwd_seq_vector) == true &&
                          self.compare_vslice(&old_rev_seq,&rev_seq_vector) == true},
                false => {self.compare_vslice(&old_fwd_seq,&fwd_seq_vector) == true},
            };
            if repeated_sequence == true || first_line == 1 {
                num_of_copies += 1;
                let mut current_fwd_quality = line_components[2].as_bytes().to_vec();
                current_fwd_quality.push(0u8);
                fwd_qualities.extend(current_fwd_quality);
                old_fwd_seq = fwd_seq_vector.to_vec();
                if self.paired == true {
                    let mut current_rev_quality = line_components[3].as_bytes().to_vec();
                    current_rev_quality.push(0u8);
                    rev_qualities.extend(current_rev_quality);
                    old_rev_seq = rev_seq_vector.to_vec();
                }
                if first_line == 1 {first_line = 0;}
                continue;
            } else {
                let mut sequence = String::from("");
                let mut quality = String::from("");
                sequence.push_str(str::from_utf8(&self.encode(r+1,wlen)).unwrap());
                quality.push_str(str::from_utf8(&self.encode(r+1,wlen)).unwrap());
                sequence.push_str("G^");
                quality.push_str("G^");
                sequence.push_str(str::from_utf8(&old_fwd_seq).unwrap());
                sequence.push_str("b");
                sequence.push_str(&*format!("{:010b}",num_of_copies));
                sequence.push_str("\0\n");
                avrg_fwd_qual = self.average_qualities(&fwd_qualities,num_of_copies);
                if self.lossy == 3 {
                    quality.push_str(str::from_utf8(&avrg_fwd_qual).unwrap());
                } else if self.lossy == 4 {
                    let red_u8_quality = self.illumina_8lev_map(&mut avrg_fwd_qual);
                    quality.push_str(str::from_utf8(&red_u8_quality).unwrap());
                }
                quality.push_str("\0\n");
                if self.paired == true {
                    sequence.push_str(str::from_utf8(&self.encode(r+1,wlen)).unwrap());
                    quality.push_str(str::from_utf8(&self.encode(r+1,wlen)).unwrap());
                    sequence.push_str("A^");
                    quality.push_str("A^");
                    sequence.push_str(str::from_utf8(&old_rev_seq).unwrap());
                    sequence.push_str("b");
                    sequence.push_str(&*format!("{:010b}",num_of_copies));
                    sequence.push_str("\0\n");
                    avrg_rev_qual = self.average_qualities(&rev_qualities,num_of_copies);
                    if self.lossy == 3 {
                        quality.push_str(str::from_utf8(&avrg_rev_qual).unwrap());
                    } else if self.lossy == 4 {
                        let red_u8_quality = self.illumina_8lev_map(&mut avrg_rev_qual);
                        quality.push_str(str::from_utf8(&red_u8_quality).unwrap());
                    }
                    quality.push_str("\0\n");
                }
                num_of_copies = 1;
                fwd_qualities = Vec::new();
                rev_qualities = Vec::new();
                let mut current_fwd_quality = line_components[2].as_bytes().to_vec();
                current_fwd_quality.push(0u8);
                fwd_qualities.extend(current_fwd_quality);
                old_fwd_seq = fwd_seq_vector.to_vec();
                if self.paired == true {
                    let mut current_rev_quality = line_components[3].as_bytes().to_vec();
                    current_rev_quality.push(0u8);
                    rev_qualities.extend(current_rev_quality);
                    old_rev_seq = rev_seq_vector.to_vec();
                }
                seq_writer.write_all(&sequence.as_bytes()).expect("Writing error!");
                qual_writer.write_all(&quality.as_bytes()).expect("Writing error!");
                r += 1;
            }
        }
        {
            let mut avrg_fwd_qual: Vec<u8> = Vec::new();
            let mut avrg_rev_qual: Vec<u8> = Vec::new();
            let mut sequence = String::from("");
            let mut quality = String::from("");
            sequence.push_str(str::from_utf8(&self.encode(r+1,wlen)).unwrap());
            quality.push_str(str::from_utf8(&self.encode(r+1,wlen)).unwrap());
            sequence.push_str("G^");
            quality.push_str("G^");
            sequence.push_str(str::from_utf8(&old_fwd_seq).unwrap());
            sequence.push_str("b");
            sequence.push_str(&*format!("{:010b}",num_of_copies));
            sequence.push_str("\0\n");
            avrg_fwd_qual = self.average_qualities(&fwd_qualities,num_of_copies);
            if self.lossy == 3 {
                quality.push_str(str::from_utf8(&avrg_fwd_qual).unwrap());
            } else if self.lossy == 4 {
                let red_u8_quality = self.illumina_8lev_map(&mut avrg_fwd_qual);
                quality.push_str(str::from_utf8(&red_u8_quality).unwrap());
            }
            quality.push_str("\0\n");
            if self.paired == true {
                sequence.push_str(str::from_utf8(&self.encode(r+1,wlen)).unwrap());
                quality.push_str(str::from_utf8(&self.encode(r+1,wlen)).unwrap());
                sequence.push_str("A^");
                quality.push_str("A^");
                sequence.push_str(str::from_utf8(&old_rev_seq).unwrap());
                sequence.push_str("b");
                sequence.push_str(&*format!("{:010b}",num_of_copies));
                sequence.push_str("\0\n");
                avrg_rev_qual = self.average_qualities(&rev_qualities,num_of_copies);
                if self.lossy == 3 {
                    quality.push_str(str::from_utf8(&avrg_rev_qual).unwrap());
                } else if self.lossy == 4 {
                    let red_u8_quality = self.illumina_8lev_map(&mut avrg_rev_qual);
                    quality.push_str(str::from_utf8(&red_u8_quality).unwrap());
                }
                quality.push_str("\0\n");
            }
            seq_writer.write_all(&sequence.as_bytes()).expect("Writing error!");
            qual_writer.write_all(&quality.as_bytes()).expect("Writing error!");
            r += 1;
        }
        self.numrec = r;
        let stats = self.make_stats(wlen);
        seq_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes()).expect("writing error!");
        qual_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes()).expect("writing error!");
        seq_writer.flush().expect("Error in flushing");
        qual_writer.flush().expect("Error in flushing");
        fs::remove_file(filename).expect("Error in removing file!"); 
        true
    }

    pub fn prepare_very_lossy_tmp<W: Write>(&mut self, filename: &str, mut seq_writer: W, tmp_seq_name: &str, outdir: &str, wlen: usize) -> bool {
        let mut lossy_reader = self.make_reader(filename);
        let lines = lossy_reader.lines().map(|l| l.unwrap());
        let mut r: usize = 0;
        let mut first_line: usize = 1;
        let mut old_fwd_seq: Vec<u8> = Vec::new();
        let mut old_rev_seq: Vec<u8> = Vec::new();
        let mut fwd_qualities: Vec<u8> = Vec::new();
        let mut rev_qualities: Vec<u8> = Vec::new();
        let mut num_of_copies: usize = 0;
        for line in lines {
            let line_components: Vec<&str> = line.split(" ").collect();
            let fwd_seq_vector = line_components[0].as_bytes().to_vec();
            let mut rev_seq_vector: Vec<u8> = Vec::new();
            if self.paired == true {rev_seq_vector = line_components[1].as_bytes().to_vec();}
            let mut avrg_fwd_qual: Vec<u8> = Vec::new();
            let mut avrg_rev_qual: Vec<u8> = Vec::new();
            let repeated_sequence: bool = match self.paired {
                true  => {self.compare_vslice(&old_fwd_seq,&fwd_seq_vector) == true &&
                          self.compare_vslice(&old_rev_seq,&rev_seq_vector) == true},
                false => {self.compare_vslice(&old_fwd_seq,&fwd_seq_vector) == true},
            };
            if repeated_sequence == true || first_line == 1 {
                num_of_copies += 1;
                let mut current_fwd_quality = line_components[2].as_bytes().to_vec();
                current_fwd_quality.push(0u8);
                fwd_qualities.extend(current_fwd_quality);
                old_fwd_seq = fwd_seq_vector.to_vec();
                if self.paired == true {
                    let mut current_rev_quality = line_components[3].as_bytes().to_vec();
                    current_rev_quality.push(0u8);
                    rev_qualities.extend(current_rev_quality);
                    old_rev_seq = rev_seq_vector.to_vec();
                }
                if first_line == 1 {first_line = 0;}
                continue;
            } else {
                let mut sequence = String::from("");
                sequence.push_str(str::from_utf8(&old_fwd_seq).unwrap());
                sequence.push_str("b");
                sequence.push_str(&*format!("{:010b}",num_of_copies));
                sequence.push_str("q");
                if self.paired == true {
                    sequence.push_str(str::from_utf8(&old_rev_seq).unwrap());
                    sequence.push_str("q");
                }
                avrg_fwd_qual = self.average_qualities(&fwd_qualities,num_of_copies);
                if self.lossy == 5 {
                    sequence.push_str(str::from_utf8(&avrg_fwd_qual).unwrap());
                } else if self.lossy == 6 {
                    let red_u8_quality = self.illumina_8lev_map(&mut avrg_fwd_qual);
                    sequence.push_str(str::from_utf8(&red_u8_quality).unwrap());
                }
                if self.paired == true {
                    sequence.push_str("q");
                    avrg_rev_qual = self.average_qualities(&rev_qualities,num_of_copies);
                    if self.lossy == 5 {
                        sequence.push_str(str::from_utf8(&avrg_rev_qual).unwrap());
                    } else if self.lossy == 6 {
                        let red_u8_quality = self.illumina_8lev_map(&mut avrg_rev_qual);
                        sequence.push_str(str::from_utf8(&red_u8_quality).unwrap());
                    }
                }
                sequence.push_str("\0\n");
                num_of_copies = 1;
                fwd_qualities = Vec::new();
                rev_qualities = Vec::new();
                let mut current_fwd_quality = line_components[2].as_bytes().to_vec();
                current_fwd_quality.push(0u8);
                fwd_qualities.extend(current_fwd_quality);
                old_fwd_seq = fwd_seq_vector.to_vec();
                if self.paired == true {
                    let mut current_rev_quality = line_components[3].as_bytes().to_vec();
                    current_rev_quality.push(0u8);
                    rev_qualities.extend(current_rev_quality);
                    old_rev_seq = rev_seq_vector.to_vec();
                }
                seq_writer.write_all(&sequence.as_bytes()).expect("Writing error!");
                r += 1;
            }
        }
        {
            let mut avrg_fwd_qual: Vec<u8> = Vec::new();
            let mut avrg_rev_qual: Vec<u8> = Vec::new();
            let mut sequence = String::from("");
            sequence.push_str(str::from_utf8(&old_fwd_seq).unwrap());
            sequence.push_str("b");
            sequence.push_str(&*format!("{:010b}",num_of_copies));
            sequence.push_str("q");
            if self.paired == true {
                sequence.push_str(str::from_utf8(&old_rev_seq).unwrap());
                sequence.push_str("q");
            }
            avrg_fwd_qual = self.average_qualities(&fwd_qualities,num_of_copies);
            if self.lossy == 5 {
                sequence.push_str(str::from_utf8(&avrg_fwd_qual).unwrap());
            } else if self.lossy == 6 {
                let red_u8_quality = self.illumina_8lev_map(&mut avrg_fwd_qual);
                sequence.push_str(str::from_utf8(&red_u8_quality).unwrap());
            }
            if self.paired == true {
                sequence.push_str("q");
                avrg_rev_qual = self.average_qualities(&rev_qualities,num_of_copies);
                if self.lossy == 5 {
                    sequence.push_str(str::from_utf8(&avrg_rev_qual).unwrap());
                } else if self.lossy == 6 {
                    let red_u8_quality = self.illumina_8lev_map(&mut avrg_rev_qual);
                    sequence.push_str(str::from_utf8(&red_u8_quality).unwrap());
                }
            }
            sequence.push_str("\0\n");
            seq_writer.write_all(&sequence.as_bytes()).expect("Writing error!");
            r += 1;
        }
        seq_writer.flush().expect("Error in flushing");
        self.numrec = r;
        let stats = self.make_stats(wlen);
        seq_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes()).expect("writing error!");
        seq_writer.flush().expect("Error in flushing");
        fs::remove_file(filename).expect("Error in removing file!"); 
        true
    }

    pub fn fastq_dw<W: Write> (&mut self, mut writer:  W) -> Result<bool,Error>  {
        let (mut sw , mut ssw, mut x, mut y, mut bw)= (0u8, true, 0, 1000, 0);
        let mut buff = vec![0u8; y];

        for ch in self.get_fastq().iter() {
            //eprintln!("{:?} \n", *ch);

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
                    } else {
                        ssw = false;
                    }
                    if bw == 4{bw = 0}
                },
                _   => {
                    if bw == 1 && ssw == true{
                        if x == y {
                            buff.extend(vec![0u8;y]);
                            y *= 2;
                        }
                        buff[x] = *ch;
                        x+=1;

                    } else {
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
