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
 use crate::util::error::Error;
 use std::io::{self, prelude::*, stdout, Write, Read, BufReader, BufWriter};

 impl Fdb{
     pub(crate) fn fastq_up<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

         let mut cnt=0;
         let mut r = 0;

         for line in reader.lines() {
             let  str = line.unwrap();
             if &str[..1] == "@" && cnt == 0 {
                 self.head.extend(str.as_bytes());
                 self.head.extend(b"\n");
                 r = r+1;
             }else if cnt == 1 {
                 self.seq.extend(str.as_bytes());
                 self.seq.extend(b"\n");
             }else if cnt == 3 {
                 self.qual.extend(str.as_bytes());
                 self.qual.extend(b"\n");
                 cnt = 0;
                 continue;
             }
             cnt = cnt+1;
         }
         self.nrec = r;
         Ok(true)
     }
/*
     pub(crate) fn fastq_dw<W: Write> (&self, mut writer:  W)   -> Result<bool,Error>  {

         for pos in self.qres.clone().into_iter() {
             writeln!(writer, "{}", self.head[pos]).unwrap();
             let st = self.mindex[pos];
             let en = if pos < self.mindex.len() -1 {
                 self.mindex[pos+1]
             }else{
                 self.seq.len()
             };

             writer.write_all(&self.seq[st..en]).unwrap();
             writer.write(b"\n+\n").unwrap();
             writer.write_all(&self.qual[st..en]).unwrap();
             writer.write(b"\n").unwrap();
         }
         Ok(true)

     }
*/
 }
