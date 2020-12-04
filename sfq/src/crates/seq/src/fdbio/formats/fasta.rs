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
use std::io::{prelude::*, Write};
use std::string::String;
use std::str;

impl Fdb{
    pub fn fasta_up<R: BufRead>(&mut self,  fwd_reader:  R, rev_reader: R, outdir: &str, output: &str) -> Result<bool,Error> {

        let mut r: usize = 1;
        let tmp_head = format!("{}/{}.head.tmp", outdir, output);
        let tmp_seq = format!("{}/{}.seq.tmp", outdir, output);
        let mut head_writer = self.make_append_writer(&tmp_head);
        let mut seq_writer = self.make_append_writer(&tmp_seq);
       
        let mut fwd_lines = fwd_reader.lines().map(|l| l.unwrap());
        let mut rev_lines = rev_reader.lines().map(|l| l.unwrap());
        let (_count, wlen) = self.comp_wlen();
        let mut new_record: bool = true;
        let mut fwd_seq = String::from("");
        let mut rev_seq = String::from("");
        let mut tmp_rev_seq = String::from("");
 
        // loop over lines of forward file
        for fwd_line in fwd_lines {
            // header line must begin with ">"
            if  &fwd_line[..1] == ">" {
                // header means we are in a new record
                new_record = true;
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
                        None => "0".to_string(),
                    };
                    rev_head.push_str(&rev_line);
                    rev_head.push_str("R\0\n");
                    head_writer.write_all(&rev_head.as_bytes()).expect("writing error!");
                }
                r += 1;
                continue;
            } else {
                // if we are in a new record
                if new_record == true {
                    // since we don't know when a new record ends, we write sequence backwards
                    if r > 2 {
                        // finish the previous record and write it to tmp file, begin new
                        fwd_seq.push_str("\0\n");
                        if r == 3 {self.line_length = fwd_seq.len()-1;}
                        seq_writer.write_all(&fwd_seq.as_bytes()).expect("writing error!");
                        fwd_seq = String::from("");
                    }
                    fwd_seq.push_str(str::from_utf8(&self.encode(r-1,wlen)).unwrap());
                    fwd_seq.push_str("G^");
                }
                fwd_seq.push_str(&fwd_line);
                // in reverse sequence, we need to convert it to complementary sequence
                // we do it in chunks, one chunk per line in temporary sequence
                if self.paired == true {
                    if new_record == true {
                        if r > 2 {
                            // put temporary sequence into prepared line and write it
                            rev_seq.push_str(&tmp_rev_seq);
                            rev_seq.push_str("\0\n");
                            seq_writer.write_all(&rev_seq.as_bytes()).expect("writing error!");
                            rev_seq = String::from("");
                            tmp_rev_seq = String::from("");
                        }
                        rev_seq.push_str(str::from_utf8(&self.encode(r-1,wlen)).unwrap());
                        rev_seq.push_str("A^");
                    }
                    // compute complementary line
                    let mut rev_line = match rev_lines.next() {
                        Some(p) => self.revcomp(p),
                        None => "0".to_string(),
                    };
                    // append temporary sequence onto the end of the current line
                    rev_line.push_str(&tmp_rev_seq);
                    // this is the new temporary sequence
                    tmp_rev_seq = rev_line;
                }
                new_record = false;
                continue;
            }
        }
        fwd_seq.push_str("\0\n");
        if self.paired == true {
            rev_seq.push_str(&tmp_rev_seq);
            rev_seq.push_str("\0\n");
        }
        seq_writer.write_all(&fwd_seq.as_bytes()).expect("writing error!");
        seq_writer.write_all(&rev_seq.as_bytes()).expect("writing error!");

        let stats = self.make_stats(wlen);
        head_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes()).expect("writing error!");
        seq_writer.write_all(str::from_utf8(&stats).unwrap().as_bytes()).expect("writing error!");

        if self.paired == false {self.rm_file("dummy.txt");}

        if self.numrec > 0 {Ok(true)}
        else{Ok(false)}
    }

    pub fn fasta_dw<W: Write> (&mut self, mut writer:  W) -> Result<bool,Error>  {
        let (mut sw , mut ssw, mut x, mut y, mut bw)= (0u8, true, 0, 1000, 0);
        let mut buff = vec![0u8; y];

        //writer.write_all(&self.get_fastq());
        let length: u32 = self.get_fasta().iter().len() as u32;
        let mut position: u32 = 0;

        // loop over all characters in dataset
        for ch in self.get_fasta().iter() {
            // check he position in the iterator
            position += 1;
            match *ch {
                // we are at the end of the line
                10u8 => {
                    // bw == 1 and ssw == true means this is reverse sequence for interleaved
                    if bw == 1 && ssw == true {
                        buff.resize(x,0x00);
                        writer.write_all(&self.revcomp(String::from_utf8(buff.clone()).unwrap()).as_bytes()).unwrap();
                        y = x;
                        x = 0;
                    }
                    write!(writer, "{}", *ch as char).unwrap();
                    bw += 1;
                    // if previous character is "R", that was the end of reverse header, we are now
                    // in reverse sequence
                    if sw == 82u8 { ssw = true;}
                    else {ssw = false;}

                    if bw == 2 {bw = 0;}
                },
                // we are not at the end of the line, unless position == length
                // then we are at the end of the iterator
                _   => {
                    // for reverse sequence, fill the buffer vector, it will have to be reversed
                    // at the end of the line or iterator and written in one step
                    if bw == 1 && ssw == true {
                        if x == y{
                            buff.extend(vec![0u8;y]);
                            y *= 2;
                        }
                        buff[x] = *ch;
                        x += 1;
                        // bugfix to print the last sequence in reverse file
                        if position == length {
                            buff.resize(x,0x00);
                            writer.write_all(&self.revcomp(String::from_utf8(buff.clone()).unwrap()).as_bytes()).unwrap();
                        }

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
