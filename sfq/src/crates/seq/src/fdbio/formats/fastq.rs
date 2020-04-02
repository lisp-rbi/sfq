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

 impl Fdb{
     pub fn fastq_up<R: BufRead>(&mut self,  reader:  R, direction: bool) -> Result<bool,Error> {

         let mut cnt=0;
         let mut r = 0;

         if direction == false {
             self.paired=true;
         }

         for line in reader.lines() {
             let  str = line.unwrap();
             if  cnt == 0 {
                 self.head.extend(str.as_bytes());
                 if direction == true {
                     self.head.extend(b"F\n");
                 }else{
                     self.head.extend(b"R\n");
                 }
                 r = r+1;
             }else if cnt == 1 {
                 let s = if  direction == false {
                     self.revcomp(str)
                }else {
                    str
                };

                 self.seq.extend(s.as_bytes());
                 self.seq.extend(b"\n");
             }else if cnt == 3 {
                 self.qual.extend(str.as_bytes());
                 self.qual.extend(b"\n");
                 cnt = 0;
                 continue;
             }
             cnt = cnt+1;
         }
         self.numrec = r;

         self.seq.resize(self.seq.len()-1,  0x00);
         self.qual.resize(self.qual.len()-1,  0x00);
         self.head.resize(self.head.len()-1,  0x00);
/*
         println!("{}:{}\n{:?}\n{:?}\n{:?}", self.seq.len(), self.seq[self.seq.len()-1], String::from_utf8(self.seq.clone()), String::from_utf8(self.qual.clone()), String::from_utf8(self.head.clone()));
*/
         if self.numrec > 0 {
             Ok(true)
         }else{
             Ok(false)
         }
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
