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

use crate::{Load, Fdb};


impl Load for Fdb {

    fn load(&mut self, fwd_path: &str, rev_path: &str, outdir: &str, output: &str) -> &mut Self{

        let fwd_reader = self.make_reader(fwd_path);
        let rev_reader = self.make_reader(rev_path);
        let mut direction: bool = true;
        if rev_path.len() > 0 {self.paired = true;}
        let num_of_lines = self.count_lines(&fwd_path).unwrap();
        let mut rev_num_of_lines: u64 = 0;
        if self.paired == true {rev_num_of_lines = self.count_lines(&rev_path).unwrap();}
        if (self.paired == true) && num_of_lines > rev_num_of_lines {
            eprintln!("WARNING: Numrec in reverse file < numrec in forward!");
            eprintln!("I will proceed anyways....");
        } else if (self.paired == true) && num_of_lines < rev_num_of_lines {
            eprintln!("WARNING: Numrec in reverse file > numrec in forward!");
            eprintln!("You will lose excess reverse records!");
            eprintln!("I will proceed anyways....");
        }

        match &self.format[..] {
            "fastq" => {
                self.numrec = (num_of_lines as usize) / 4;
                self.cpcnt = vec![1;(self.numrec+2)*2];
                if let Ok(false) = self.fastq_up(fwd_reader,rev_reader,outdir,output) {
                    panic!("{} file not uploaded !", self.format);
                };

            },
            "fasta" => {
                self.numrec = num_of_lines as usize;
                self.cpcnt = vec![1;(self.numrec+2)*2];
                if let Ok(false) = self.fasta_up(fwd_reader,rev_reader,outdir,output) {
                    panic!("{} file not uploaded !", self.format);
                };
            },
            _       => {
                panic!("format not supported!");
            }
        }
        self
    }
}
