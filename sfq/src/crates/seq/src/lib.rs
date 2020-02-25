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

#[cfg(test)]
mod tests;

mod util;
mod fdbio;



pub trait IO: Get+Set+Load+Save {}

pub trait Get {

    fn get_head (self)  -> Vec<u8>;
    fn get_seq  (self)  -> Vec<u8>;
    fn get_qual (self)  -> Vec<u8>;
    fn get_fasta(&self) -> Vec<u8>;
    fn get_fastq(&self) -> Vec<u8>;
    fn get_tsv  (&self, model: &str) -> Vec<u8>;

}

pub trait Set {

    fn set_head (&mut self, data: Vec<u8>)-> &mut Self ;
    fn set_seq (&mut self, data: Vec<u8>)-> &mut Self ;
    fn set_qual (&mut self, data: Vec<u8>)-> &mut Self ;

}

pub trait Push {

    fn push_head (&mut self, data: Vec<u8>)-> &mut Self ;
    fn push_seq (&mut self, data: Vec<u8>)-> &mut Self ;
    fn push_qual (&mut self, data: Vec<u8>)-> &mut Self ;

}

pub trait Load {

    fn load(&mut self, path: &str, direction: bool) -> &mut Self;

}

pub trait Save {

    fn save(&mut self, path: &str) -> bool;

}


/// File database object

#[derive(Debug, Clone)]
pub struct Fdb {
    format: String,
    numrec: usize,
    head: Vec<u8>,
    seq: Vec<u8>,
    qual: Vec<u8>
}


impl Fdb {
    pub fn new (filetype: &str)-> Self{

        let ftype : String   = match filetype {
            "fasta" | "fastq" => filetype.to_string(),
             _ => panic!("File format {} not supported !", filetype ),
        };

        Fdb{
            format: ftype,
            numrec:  0,
            head: Vec::new(),
            seq: Vec::new(),
            qual: Vec::new(),
        }
    }



    pub fn sort (&mut self, key: &str) ->  &mut Self  {

        let (mut a, mut b, mut c) = (self.head.clone(), self.seq.clone(), self.qual.clone());

        match key {
            "h" => {
                if self.sort_by(&mut a, &mut b, &mut c) == false {
                    panic!("Sorting scr...");
                };
            },
            "q" => {
                if self.sort_by(&mut c, &mut a, &mut b) == false {
                    panic!("Sorting scr...");
                };
            },
            "s" => {
                if self.sort_by(&mut b, &mut a, &mut c) == false {
                    panic!("Sorting scr...");
                };
            },
            _  => {
                panic!("Nope, bad key! \n Only h=head, q=quality, s=sequence are supported!\n");
            }
        }
        self.head = a;
        self.seq  = b;
        self.qual = c;

        self
    }
}
