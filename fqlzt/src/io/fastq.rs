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



// is there a need for a db ??
 impl FastqaDb{
     pub(crate) fn fastq_up<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

         // check if fastq

         let mut i= self.head.len();
         let mut cnt=0;
         self.findex.push(i);  // not sure what this is about...

         for line in reader.lines() {
             let  str = line.unwrap();
             if &str[..1] == "@" && cnt == 0 {
                 self.head.push(str.clone());
                 let id = (&str[1..str.find(" ").unwrap()]).to_string();
                 self.id.push(id.clone());
                 self.rindex.entry(id).or_insert(Vec::new()).push(i);
                 i=i+1;
             }else if cnt == 1 {
                 self.mindex.push(self.seq.len());
                 self.seq.extend(str.as_bytes());
             }else if cnt == 3 {
                 self.qual.extend(str.as_bytes());
                 cnt = 0;
                 continue;
             }
             cnt = cnt+1;
         }
         Ok(true)
     }

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

 }
