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

use std::fs;
use crate::Fdb;
use std::process::Command;
use fs2;
use std::env;
use sys_info::mem_info;


impl Fdb{

    pub fn clear(&mut self) -> &mut Self {
        self.head = Vec::new();
        self.seq = Vec::new();
        self.qual = Vec::new();
        self.cpcnt = Vec::new();
        self.numrec = 0;

        self
    }

    pub fn clear_head(&mut self) -> &mut Self {
        self.head.resize(0,0x00);
        self
    }
    pub fn clear_seq(&mut self) -> &mut Self {
        self.seq.resize(0,0x00);
        self
    }
    pub fn clear_qual(&mut self) -> &mut Self {
        self.qual.resize(0,0x00);
        self
    }

    pub fn revcomp(&self, s: String) -> String {

        s.chars()
        .map(|x| match x {
            'a' | 'A' => 'T',
            't' | 'T' => 'A',
            'g' | 'G' => 'C',
            'c' | 'C' => 'G',
            _         => x
        }).rev().collect()

    }

    pub fn  encode (&mut self, a: usize, b: usize) -> Vec<u8> {

        let c = self.alpha.as_bytes().to_vec();
        let (mut v, mut res) = (vec![0u8;b],a);

        for i in 0..b {
            let rmd = res%4;
            res = res/4;
            v[b-(i+1)] = c[rmd];
        }
        v
    }

    pub fn comp_wlen(& self) -> (usize,usize) {
        let alpha = self.alpha.len();
        let cnt = self.cpcnt.iter().max().unwrap() * self.cpcnt.len();
        (cnt ,((cnt+1) as f64).log(alpha as f64).ceil() as usize)
    }

    pub fn make_stats(&self, padding: usize) -> Vec<u8> {

        let mut vec : Vec<u8> = Vec::new();
 
        vec.extend(b"~~~~~^".to_vec());
        vec.extend(self.numrec.to_string().as_bytes().to_vec());
        vec.push(94u8);
        vec.extend(self.alpha.as_bytes().to_vec());
        vec.push(94u8);
        vec.extend(padding.to_string().as_bytes().to_vec());
        vec.push(94u8);
        if self.paired == true {vec.push(49u8);}
        else {vec.push(48u8);}
        vec.push(94u8);
        vec.extend(self.lossy.to_string().as_bytes().to_vec());
        vec
    }

    pub(crate) fn compare_vslice(&self, va: &[u8], vb: &[u8]) -> bool {

        (va.len() == vb.len()) && va.iter()
            .zip(vb)
            .all(|(a,b)| *a == *b)

    }

    /* sorts lines of a file through external bash commands
       1) separates file into subfiles; according to available RAM and disk
       2) sort each subfile into a copy and move the copy back to subfile
       3) merge-sort subfiles into the original file */
    pub fn sort_lines(&self,filename: &str, outdir: &str) -> bool {
        // list the current directory and see available memory
        let current_dir = env::current_dir().unwrap();
        let free_disk_space = fs2::free_space(current_dir).unwrap() as f32;
        // check size of the file
        let file_size = fs::metadata(filename).unwrap().len() as f32;
        // if file takes more than 1/3 of disk, it may clogg up the disk
        let ratio = (free_disk_space / file_size).ceil() as u32;
        if ratio < (4.0 as u32) {panic!("Not enough disk space for sorting!");}
        // check available RAM, take half of it
        let avail_ram = ((mem_info().unwrap().total * 1024) / 2) as f32;
        let ram_ratio = (avail_ram / file_size) as f32;
        // how many lines will be in sub-files for sorting
        let mut num_of_lines: String = "-l ".to_owned();
        num_of_lines.push_str(&((ram_ratio * (self.numrec as f32)).ceil() as u64).to_string());
        let mut tmp_filename: String = "".to_owned();
        tmp_filename.push_str(filename);
        tmp_filename.push_str("_");
        // use bash to split original file into subfiles for sorting
        let _split_file = Command::new("split").args(&["-a 10", &num_of_lines])
            .arg(filename).arg(tmp_filename.clone())
            .status().expect("Error in splitting file");
        // list created temporary files, it returns vector of ASCII u8
        let list_tmp = Command::new("ls").arg(outdir).output().expect("error!");
        let tmp_files: Vec<_> = list_tmp.stdout.split(|i| *i == 10u8).collect();
        // create a vector of temporary file names
        let mut tmp_filename_list = Vec::<String>::new();
        // loop over temporary files
        for tmp_file in tmp_files {
            let mut tmp_filename = outdir.to_string();
            tmp_filename.push_str("/");
            tmp_filename.push_str(&String::from_utf8(tmp_file.to_vec()).unwrap());
            // sort only subfiles containing "tmp_"
            if tmp_filename.contains("tmp_") {
                let mut tmp_tmp_filename: String = "-o".to_owned();
                tmp_tmp_filename.push_str(&tmp_filename);
                tmp_tmp_filename.push_str(".cp");
                tmp_filename_list.push(tmp_filename.clone());
                // sort the subfiles, send output to tmp_tmp_filename
                let _sort_tmp = Command::new("sort").arg(tmp_filename.clone()).arg(tmp_tmp_filename.clone()).output().expect("Error in sorting.");
                tmp_tmp_filename.replace_range(..2,"");
                // overwrite original temporary filenames with sorted ones
                let _overwrite = Command::new("mv").arg(&tmp_tmp_filename).arg(&tmp_filename).status().expect("Error in overwriting!");
            }
        }
        assert!(tmp_filename_list.len() > 0);
        // end_filename should be equal to the original input-file (with -o)
        let mut end_filename: String = "-o".to_owned();
        end_filename.push_str(filename);
        // merge sorted subfiles into the original file
        let _sort_tmp = Command::new("sort").arg("-m")
            .args(&tmp_filename_list.clone()).arg(end_filename)
            .status().expect("Error in merging!");
        // delete the subfiles
        let _delete = Command::new("rm").args(&tmp_filename_list).status().expect("Error in deleting!");
        true
    }

    /*pub fn sort_lines_simple(&self,filename: &str, outdir: &str) -> bool {
        let mut tmp_filename: String = "-o".to_owned();
        tmp_filename.push_str(&outdir.to_string());
        tmp_filename.push_str("/sorted.tmp");
        let _sort_file = Command::new("sort").arg(filename.clone()).arg(tmp_filename.clone()).status().expect("Error in sorting.");
        //let _sort_file = Command::new("sort").arg(filename.clone()).status().expect("Error in sorting.");
        tmp_filename.replace_range(..2,"");
        let mut end_filename: String = "-o".to_owned();
        end_filename.push_str(filename);
        let _overwrite = Command::new("mv").arg(&tmp_filename).arg(&filename).status().expect("Error in overwriting!");
        true
    }*/
}
