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
    pub fn fastq_up<R: BufRead>(&mut self, fwd_reader: R, rev_reader: R, output: &str) -> Result<bool,Error> {

        let mut cnt=0;
        let mut r: usize = 0;
        let tmp_head = format!("{}/{}.head.tmp", output, output);
        let tmp_seq = format!("{}/{}.seq.tmp", output, output);
        let tmp_qual = format!("{}/{}.qual.tmp", output, output);
        let mut head_writer = self.make_append_writer(&tmp_head);
        let mut seq_writer = self.make_append_writer(&tmp_seq);
        let mut qual_writer = self.make_append_writer(&tmp_qual);

        let mut fwd_lines = fwd_reader.lines().map(|l| l.unwrap());
        let mut rev_lines = rev_reader.lines().map(|l| l.unwrap());
        let (count, wlen) = self.comp_wlen();

        for fwd_line in fwd_lines {
            if  cnt == 0 {
                let mut fwd_head: Vec<u8> = Vec::new();
                fwd_head.extend(self.encode(r,wlen));
                fwd_head.extend(b"G");
                fwd_head.extend(b"^");
                fwd_head.extend(fwd_line.as_bytes());
                if self.paired == true {fwd_head.extend(b"F\0");}
                else {fwd_head.extend(b"\0");}
                for elem in fwd_head{
                    if elem == 0 {write!(head_writer,"{:?}\n",elem);} 
                    else {write!(head_writer,"{:?} ",elem);}
                }
                if self.paired == true {
                    let mut rev_head: Vec<u8> = Vec::new();
                    rev_head.extend(self.encode(r+1,wlen));
                    rev_head.extend(b"A");
                    rev_head.extend(b"^");
                    let rev_line = match rev_lines.next() {
                        Some(p) => p,
                        None => "0".to_string(),
                    };
                    rev_head.extend(rev_line.as_bytes());
                    rev_head.extend(b"R\0");
                    for elem in rev_head{
                        if elem == 0 {write!(head_writer,"{:?}\n",elem);} 
                        else {write!(head_writer,"{:?} ",elem);}
                    }
                }
                cnt = cnt+1;
                continue;
            }else if cnt == 1 {
                let mut fwd_seq: Vec<u8> = Vec::new();
                fwd_seq.extend(self.encode(r,wlen));
                fwd_seq.extend(b"G");
                fwd_seq.extend(b"^");
                fwd_seq.extend(fwd_line.as_bytes());
                fwd_seq.extend(b"\0");
                for elem in fwd_seq{
                    if r == self.numrec-1 {self.line_length += 1;}
                    if elem == 0 {write!(seq_writer,"{:?}\n",elem);} 
                    else {write!(seq_writer,"{:?} ",elem);}
                }
                if self.paired == true {
                    let mut rev_seq: Vec<u8> = Vec::new();
                    rev_seq.extend(self.encode(r+1,wlen));
                    rev_seq.extend(b"A");
                    rev_seq.extend(b"^");
                    let rev_line = match rev_lines.next() {
                        Some(p) => self.revcomp(p),
                        None => "0".to_string(),
                    };
                    rev_seq.extend(rev_line.as_bytes());
                    rev_seq.extend(b"\0");
                    for elem in rev_seq{
                        if elem == 0 {write!(seq_writer,"{:?}\n",elem);} 
                        else {write!(seq_writer,"{:?} ",elem);}
                    }
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
                let mut fwd_qual: Vec<u8> = Vec::new();
                fwd_qual.extend(self.encode(r,wlen));
                fwd_qual.extend(b"G");
                fwd_qual.extend(b"^");
                fwd_qual.extend(fwd_line.as_bytes());
                fwd_qual.extend(b"\0");
                for elem in fwd_qual{
                    if elem == 0 {write!(qual_writer,"{:?}\n",elem);} 
                    else {write!(qual_writer,"{:?} ",elem);}
                }
                if self.paired == true {
                    let mut rev_qual: Vec<u8> = Vec::new();
                    rev_qual.extend(self.encode(r+1,wlen));
                    rev_qual.extend(b"A");
                    rev_qual.extend(b"^");
                    let rev_line = match rev_lines.next() {
                        Some(p) => p,
                        None => "0".to_string(),
                    };
                    rev_qual.extend(rev_line.as_bytes());
                    rev_qual.extend(b"\0");
                    for elem in rev_qual{
                        if elem == 0 {write!(qual_writer,"{:?}\n",elem);} 
                        else {write!(qual_writer,"{:?} ",elem);}
                    }
                    r += 2;
                } else {r += 1;}
                cnt = 0;
                continue;
            }
        }

        let stats = self.make_stats(wlen);
        for stat in stats{
            write!(head_writer,"{:?} ",stat);
            write!(seq_writer,"{:?} ",stat);
            write!(qual_writer,"{:?} ",stat);
        }

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
