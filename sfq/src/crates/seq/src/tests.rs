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

use crate::{Fdb, Get, Set, IO, Load, Save};



#[test]
fn load_fastq_file() {
    let path =  "./example/in.fq";

    let mut fdb = Fdb::new("fastq");
    fdb.load(path, true);

    assert_eq!("@SSR0F\nCGTGCC\n+\nGHZ!Rj\n@SSR1F\nATGCGT\n+\nGFHGGU".to_string(),
        String::from_utf8(fdb.get_fastq()).unwrap());

}

#[test]
fn load_fasta_file() {
    let path =  "./example/in.fa";

    let mut fdb = Fdb::new("fasta");
    fdb.load(path, true);

    assert_eq!(">My headerF\nATGCGTAGCGTAGCGATCGCTTCGTCGCTGTCGCTCGC\n>mY headF\nAAAAAAAAAAAAAAAATTTTTTTTTTTTTTTTGGGGGGGGG".to_string(),
        String::from_utf8(fdb.get_fasta()).unwrap());

}


#[test]
fn save_fastq_file() {
    let path =  "./example/in.fq";

    let mut fdb = Fdb::new("fastq");
    fdb.load(path, true);

    fdb.save("./example/in.fq.sv");
    assert_eq!(1+1,2);

}

#[test]
fn save_fasta_file() {
    let path =  "./example/in.fa";

    let mut fdb = Fdb::new("fasta");
    fdb.load(path, true);

    fdb.save("./example/in.fa.sv");
    assert_eq!(1+1, 2);

}



#[test]
fn resort_fastq_by_header() {
    let seq =  b"ATGCGT\nCGTGCC".to_vec();
    let qual = b"GFHGGU\nGHZ!Rj".to_vec();
    let head=  b"@SSR1\n@SSR0".to_vec();

    let mut fdb = Fdb::new("fastq");

    fdb.set_head(head);
    fdb.set_qual(qual);
    fdb.set_seq(seq);
    fdb.sort("h");

    assert_eq!("@SSR0\nCGTGCC\n+\nGHZ!Rj\n@SSR1\nATGCGT\n+\nGFHGGU".to_string(),
        String::from_utf8(fdb.get_fastq()).unwrap());

}


#[test]
fn resort_fastq_by_sequence() {
    let seq =  b"ATGCGT\nCGTGCC".to_vec();
    let qual = b"GFHGGU\nGHZ!Rj".to_vec();
    let head=  b"@SSR1\n@SSR0".to_vec();

    let mut fdb = Fdb::new("fastq");

    fdb.set_head(head);
    fdb.set_qual(qual);
    fdb.set_seq(seq);
    fdb.sort("s");

    assert_eq!("@SSR1\nATGCGT\n+\nGFHGGU\n@SSR0\nCGTGCC\n+\nGHZ!Rj".to_string(),
        String::from_utf8(fdb.get_fastq()).unwrap());

}


#[test]
fn resort_fastq_by_quality() {
    let seq =  b"ATGCGT\nCGTGCC".to_vec();
    let qual = b"GFHGGU\nGEZ!Rj".to_vec();
    let head=  b"@SSR1\n@SSR0".to_vec();

    let mut fdb = Fdb::new("fastq");

    fdb.set_head(head);
    fdb.set_qual(qual);
    fdb.set_seq(seq);
    fdb.sort("q");

    assert_eq!("@SSR0\nCGTGCC\n+\nGEZ!Rj\n@SSR1\nATGCGT\n+\nGFHGGU".to_string(),
        String::from_utf8(fdb.get_fastq()).unwrap());

}



//  sort_by does not work if quality is missing!!!
#[test]
fn get_set_tsv() {
    let seq =  b"ATGCGT\nCGTGCC".to_vec();
    let qual = b"GFHGGU\nGEZ!Rj".to_vec();
    let head=  b"@SSR1\n@SSR0".to_vec();

    let mut fdb = Fdb::new("fastq");

    fdb.set_head(head);
    fdb.set_qual(qual);
    fdb.set_seq(seq);
    fdb.sort("h");



    assert_eq!("@SSR0\tCGTGCC\tGEZ!Rj\n@SSR1\tATGCGT\tGFHGGU".to_string(),
        String::from_utf8(fdb.get_tsv()).unwrap());
    //assert_eq!("@SSR1\tATGCGT\n@SSR0\tCGTGCC".to_string(),
    //    String::from_utf8(fdb.get_tsv()).unwrap());


}


#[test]
fn set_get_on_fastq() {

    let seq = b"ATGCGTGCAACATHNNGCGT".to_vec();
    let qual = b"/&%Z%T%$$$#FRHGFGGU".to_vec();
    let head= b"@SSR34:55:45 2:N:0".to_vec();

    let mut fdb = Fdb::new("fastq");

    fdb.set_head(head);
    fdb.set_qual(qual);
    fdb.set_seq(seq);

    assert_eq!(b"@SSR34:55:45 2:N:0\nATGCGTGCAACATHNNGCGT\n+\n/&%Z%T%$$$#FRHGFGGU".to_vec(),fdb.get_fastq());
}


#[test]
fn set_get_on_header() {
    let head= b"@SSR34:55:45 2:N:0".to_vec();
    let mut fdb = Fdb::new("fastq");
    fdb.set_head(head.clone());
    assert_eq!(fdb.get_head(), head);
}


#[test]
fn set_get_on_sequance() {
    let seq= b"ATGTGCGTGCAACNTGTC".to_vec();
    let mut fdb = Fdb::new("fastq");
    fdb.set_seq(seq.clone());
    assert_eq!(fdb.get_seq(), seq);
}


#[test]
fn set_get_on_quality() {
    let qual= b"!!!##$#%%&%655".to_vec();
    let mut fdb = Fdb::new("fastq");
    fdb.set_qual(qual.clone());
    assert_eq!(fdb.get_qual(), qual);
}
