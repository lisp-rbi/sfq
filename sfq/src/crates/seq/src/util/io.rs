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
use std::io::{self, prelude::*, Write, Read, BufReader, BufWriter};
use std::fs::{File,OpenOptions};
use std::fs;

impl Fdb{

    pub fn make_reader(&mut self, file: &str)-> BufReader<Box< dyn Read >> {

        let tmp : Box<dyn Read> = match file {
            "stdin" => {
                Box::new(io::stdin())
            },
            "" => {
                let dummyfile: &str = "dummy.txt";
                File::create(dummyfile)
                    .expect(&(format!("Error creating {} file",dummyfile)));
                Box::new(File::open(dummyfile)
                    .expect(&(format!("Error opening {:?} file",dummyfile))))
            },
            _       => {
                Box::new(File::open(file)
                    .expect(&(format!("Error opening {} file",file))))
            }
        };
        BufReader::new(tmp)
    }


    pub fn make_writer (&mut self, file: &str)-> BufWriter<Box<dyn Write>> {

        let tmp : Box<dyn Write> = match file {
            "stdout" => {
                Box::new(io::stdout())
            },
            _       => {
                Box::new(File::create(file)
                    .expect(&(format!("Error opening {} file",file))))
            }
        };
        BufWriter::new(tmp)
    }

    pub fn make_append_writer (&mut self, file: &str)-> BufWriter<Box<dyn Write>> {

        let tmp : Box<dyn Write> = match file {
            "stdout" => {
                Box::new(io::stdout())
            },
            _       => {
                if fs::metadata(file).is_ok() == false {
                    File::create(file).expect(&(format!("Error opening {} file",file)));
                }
                Box::new(OpenOptions::new().write(true).append(true).open(file).unwrap())
            }
        };
        BufWriter::new(tmp)
    }

    pub fn rm_file (&mut self, file: &str)-> bool {
        if fs::metadata(file).is_ok() == true {
                fs::remove_file(file).unwrap();
        }
        true
    }

    pub fn count_lines(&mut self, path: &str) -> Result<u64, String> {
    
        let mut reader = BufReader::new(Box::new(File::open(path)
                    .expect(&(format!("Error opening {} file",path)))));
        let mut num_of_lines: u64 = 0;
        let mut line = String::from("");
    
        loop{
            match reader.read_line(&mut line) {
                Ok( _ ) => {
                        if line.len() == 0 {break;}
                        if &self.format == "fasta" {
                            if &line[..1] == ">" {num_of_lines += 1;}
                        } else if &self.format == "fastq" {
                            num_of_lines += 1;
                        }
                        line.clear();
                },
                Err(why) => return Err(why.to_string())
            };
        }
        Ok(num_of_lines)
    }

    pub fn sort_file(&mut self, path: &str, outdir: &str) -> Result<bool, String> {
        //let mut file = OpenOptions::new().read(true).write(true).open(path).expect("Error opening file");
        //let mut file = File::open(path).expect("file error");
        //let reader = BufReader::new(&mut file);
        let reader = self.make_reader(&path);
        //let mut lines: Vec<_> = reader.lines().map(|l| l.expect("Couldn't read a line")).collect();
        let mut lines = reader.lines().map(|l| l.unwrap());
        //if lines.len() == 0 { return Err("Empty file".to_string());}
        //eprintln!("lines.len() = {:?}", lines.len());
        //lines.sort();
        //eprintln!("lines.len() = {:?}", lines.len());
        for line in lines {eprintln!("{:?}", line);}
        let mut tmp_path = outdir.to_owned();
        tmp_path.push_str("sorted.tmp");
        /*let mut tmp_file = File::create(tmp_path.clone()).expect("file error");
        for mut line in lines {
            line.push_str("\n");
            tmp_file.write_all(line.as_bytes()).expect("Couldn't write to file");
        }
        fs::rename(tmp_path, path).expect("Error in renaming temporary file.");*/
        Ok(true)
   }
}
