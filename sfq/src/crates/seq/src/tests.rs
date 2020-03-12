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

use crate::{
    Fdb,   // main object
    Get,
    Set,
    IO,
    Load,
    Save,
    Push,
};





#[test]
fn contract() {
    let r1 =  "./example/in_cpcnt_R1.fq";
    let r2 =  "./example/in_cpcnt_R2.fq";

    let mut fdb = Fdb::new("fastq");

    fdb.load(r1, true);
    fdb.load(r2, false);
    fdb.colaps();

    assert_eq!("@seq_4R\nTCTACGCACACGCC\n+\n$%$#)(/&%&&&&\n@seq_2R\nTGTACGCACACGCA\n+\n$%$#)(/&%&&&&\n@seq_3R\nTGTACGCACACGCC\n+\n$%$#)(/&%&&&&\n@seq_3F\nATGCGTGTGCGTACA\n+\n#$%$#)(/&%&&&&=\n@seq_8R\nAGTACGCACACGCA\n+\n$%$#)(/&%&&&!\n@seq_8F\nATGCGTGTGCGTACT\n+\n#$%$#)(/&%&&&&!".to_string(),
        String::from_utf8(fdb.get_fastq()).unwrap());

}

#[test]
fn expand() {
    let r1 =  "./example/in_cpcnt_R1.fq";
    let r2 =  "./example/in_cpcnt_R2.fq";

    let mut fdb = Fdb::new("fastq");

    fdb.load(r1, true);
    fdb.load(r2, false);
    fdb.colaps();
    fdb.expand();

    assert_eq!("@seq_21F\nATGCGTGTGCGTACA\n+\n#$%$#)(/&%&&&&=\n@seq_21R\nTGTACGCACACGCA\n+\n$%$#)(/&%&&&&\n@seq_22F\nATGCGTGTGCGTACA\n+\n#$%$#)(/&%&&&&=\n@seq_22R\nTGTACGCACACGCA\n+\n$%$#)(/&%&&&&\n@seq_31F\nATGCGTGTGCGTACA\n+\n#$%$#)(/&%&&&&=\n@seq_31R\nTGTACGCACACGCC\n+\n$%$#)(/&%&&&&\n@seq_41F\nATGCGTGTGCGTACA\n+\n#$%$#)(/&%&&&&=\n@seq_41R\nTCTACGCACACGCC\n+\n$%$#)(/&%&&&&\n@seq_81F\nATGCGTGTGCGTACT\n+\n#$%$#)(/&%&&&&!\n@seq_81R\nAGTACGCACACGCA\n+\n$%$#)(/&%&&&!\n@seq_82F\nATGCGTGTGCGTACT\n+\n#$%$#)(/&%&&&&!\n@seq_82R\nAGTACGCACACGCA\n+\n$%$#)(/&%&&&!\n@seq_83F\nATGCGTGTGCGTACT\n+\n#$%$#)(/&%&&&&!\n@seq_83R\nAGTACGCACACGCA\n+\n$%$#)(/&%&&&!\n@seq_84F\nATGCGTGTGCGTACT\n+\n#$%$#)(/&%&&&&!\n@seq_84R\nAGTACGCACACGCA\n+\n$%$#)(/&%&&&!".to_string(),
        String::from_utf8(fdb.get_fastq()).unwrap());

}



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

    fdb.save("./example/in.fq.sv", "fq");
    assert_eq!(1+1,2);

}

#[test]
fn save_fasta_file() {
    let path =  "./example/in.fa";

    let mut fdb = Fdb::new("fasta");
    fdb.load(path, true);

    fdb.save("./example/in.fa.sv", "fa");
    assert_eq!(1+1, 2);

}

#[test]
fn save_tsv_file_hsq() {
    let path =  "./example/in.fq";

    let mut fdb = Fdb::new("fastq");
    fdb.load(path, true);

    fdb.save("./example/in.tsv_hsq.sv", "h+s+q");
    assert_eq!(1+1, 2);

}

#[test]
fn save_tsv_file_qhs() {
    let path =  "./example/in.fq";

    let mut fdb = Fdb::new("fastq");
    fdb.load(path, true);

    fdb.save("./example/in.tsv_qhs.sv", "q+h+s");
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
fn get_set_tsv_hsq() {
    let seq =  b"ATGCGT\nCGTGCC".to_vec();
    let qual = b"GFHGGU\nGEZ!Rj".to_vec();
    let head=  b"@SSR1\n@SSR0".to_vec();

    let mut fdb = Fdb::new("fastq");

    fdb.set_head(head);
    fdb.set_qual(qual);
    fdb.set_seq(seq);
    fdb.sort("h");
    println!("HHHHHH");



    assert_eq!("@SSR0\tCGTGCC\tGEZ!Rj\n@SSR1\tATGCGT\tGFHGGU".to_string(),
        String::from_utf8(fdb.get_tsv("h+s+q")).unwrap());

}

//  sort_by does not work if quality is missing!!!
#[test]
fn get_set_tsv_sqh() {
    let seq =  b"ATGCGT\nCGTGCC".to_vec();
    let qual = b"GFHGGU\nGEZ!Rj".to_vec();
    let head=  b"@SSR1\n@SSR0".to_vec();

    let mut fdb = Fdb::new("fastq");

    fdb.set_head(head);
    fdb.set_qual(qual);
    fdb.set_seq(seq);
    fdb.sort("h");



    assert_eq!("CGTGCC\tGEZ!Rj\t@SSR0\nATGCGT\tGFHGGU\t@SSR1".to_string(),
        String::from_utf8(fdb.get_tsv("s+q+h")).unwrap());

}

#[test]
fn get_set_tsv_qh() {
    let seq =  b"ATGCGT\nCGTGCC".to_vec();
    let qual = b"GFHGGU\nGEZ!Rj".to_vec();
    let head=  b"@SSR1\n@SSR0".to_vec();

    let mut fdb = Fdb::new("fastq");

    fdb.set_head(head);
    fdb.set_qual(qual);
    fdb.set_seq(seq);
    fdb.sort("h");



    assert_eq!("GEZ!Rj\t@SSR0\nGFHGGU\t@SSR1".to_string(),
        String::from_utf8(fdb.get_tsv("q+h")).unwrap());
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



#[test]
fn push_get_on_header() {
    let head= b"@SSR34:55:45 2:N:0".to_vec();
    let mut fdb = Fdb::new("fastq");
    fdb.push_head(head.clone());
    fdb.push_head(head.clone());
    fdb.push_head(head.clone());
    assert_eq!(fdb.get_head(), b"@SSR34:55:45 2:N:0\n@SSR34:55:45 2:N:0\n@SSR34:55:45 2:N:0".to_vec());
}


#[test]
fn push_get_on_sequance() {
    let seq= b"ATGTGCGTGCAACNTGTC".to_vec();
    let mut fdb = Fdb::new("fastq");
    fdb.push_seq(seq.clone());
    fdb.push_seq(seq.clone());
    fdb.push_seq(seq.clone());
    assert_eq!(fdb.get_seq(), b"ATGTGCGTGCAACNTGTC\nATGTGCGTGCAACNTGTC\nATGTGCGTGCAACNTGTC".to_vec());
}


#[test]
fn push_get_on_quality() {
    let qual= b"!!!##$#%%&%655".to_vec();
    let mut fdb = Fdb::new("fastq");
    fdb.set_qual(qual.clone());
    fdb.push_qual(qual.clone());
    assert_eq!(fdb.get_qual(), b"!!!##$#%%&%655\n!!!##$#%%&%655".to_vec());
}
