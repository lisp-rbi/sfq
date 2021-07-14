# SFQ

[![License](https://img.shields.io/crates/l/rustc-serialize/0.3.24)]( )


SFQ is a software that produces and reads sFASTQ format for the compression and online decompression of FASTQ files. The succinct sFASTQ representation of a FASTQ file is stored on disk as a top level directory with subdirectories. The size of sFASTQ is approximately the same as that of the GZIP file. The sFASTQ format supports random access to the specific records, and it can be used in place of a flat FASTQ file as the input for downstream applications.


## Installation protocol


If you’re running macOS, Linux, or another Unix-like OS. To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Current version of the software requires Rust (> v1.38) and g++ (> v4.9.1). `sfq` is being tested on Rust 1.38.0, but it is likely to work on other subsequent versions as well. Once the rust and cargo have been installed, execute the following:

 1 . Download the library:

```
git clone https://github.com/lisp-rbi/sfq.git
```
 2 . Compile binaries:


```
cd ./sfq/sfq/src/

cargo build --release
```

This will compile the source to `../target/release/`

 3 . Execute binary by running:

```
../target/release/sfq -h
```
or by adding it to your PATH

## Usage

```



sfq 0.3.0
Robert Bakaric <rbakaric@irb.hr>, Dalibor Hrsak <dalibor.hrsak@irb.hr>, Damir Korencic <dkorencic@irb.hr>

    ______     ______   ______    	
   /\  ___\   /\  ___\ /\  __ \   	
   \ \___  \  \ \  __\ \ \ \/\_\  	
    \/\_____\  \ \_\    \ \___\_\ 	
     \/_____/   \/_/     \/___/_/ 


            Authors: Bakaric R., Hrsak D., Korencic, D. & Ristov, S.

USAGE:
    sfq [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --action <c|d|g>                   Action: (c) compress, (d) decompress, (g) grep <requires --list >  [default:
                                           c]
    -s, --compression-mode <0-4>           Compression mode. Lossless: 0, lossy: 1 to 4. [default: 0]
    -F, --fragment-size <Max|<integer>>    Amount of RAM in MB allocated for the compression. Max = use all available
                                           RAM. [default: Max]
    -t, --infmt <fastq|fasta>              File types supported [default: fastq]
    -i, --input <FILE>                     Input file (fasta,fastq,sfastq)
    -j, --input-rev <FILE>                 Filename of a reverse file (fastq, fasta)
    -l, --list <filename|rand(10)>         Please provide a list of prefixes (numbers or ranges), in separate lines. SFQ
                                           returns records associated with the input prefixes. Works only with -a g.
                                           [default: rand(10)]
    -m, --memory-mode <D|R>                Memory mode: defines memory type  (D - disc, R - RAM) [default: D]  [possible
                                           values: D, R]
    -f, --outfmt <fq|fa|s|q|h|...>         Output format: 

                                           	fq   	:fastq, 
                                           	fa  	:fasta, 
                                           	s  	:sequence, 
                                           	q  	:quality, 
                                           	h  	:head, 
                                           	s+q  	:sequence quality, 
                                           	h+q  	:head quality, 
                                           	h+s  	:head sequence, 
                                           	h+s+q  	:head sequence quality, 
                                           	s+h+q  	:sequence head quality, 
                                           	...
                                            [default: fq]
    -o, --output <FILE>                    Output file; interleaved if input is two paired end fastq files
    -r, --restart <no|yes>                 Restart compression from temporary files. Works only with -a c. NOTE:
                                           Temporary files must be complete and correct! [default: no]


```

## Examples

Example No.1 - Compress single stranded fastq file

```
sfq -i ./data/fwd.fq -a c -t fastq -o FwdIdx
```

Example No.2 - Compress paired-end fastq file

```
sfq -i ./data/fwd.fq -j ./data/rev.fq -a c -t fastq -o FwdRevIdx
```

Example No.3 - Decompress single stranded fastq files by printing full records

```
sfq -i FwdIdx -a d -f fq -t fastq -o fw.fq
```

Example No.4 - Decompress paired-end fastq files by printing full records

```
sfq -i FwdRevIdx -a d -f fq -t fastq -o fw_rv.fq
```

Example No.5 - Decompress single stranded fastq files by printing fasta records

```
sfq -i FwdIdx -a d -f fa -t fastq -o fw.fa
```

Example No.6 - Decompress paired-end fastq files by printing fasta records

```
sfq -i FwdRevIdx -a d -f fa -t fastq -o fw-re.fa
```

Example No.7 - Decompress single stranded fastq files by printing tsv formatted: head \\tab seq

```
sfq -i FwdIdx -a d -f "h+s" -t fastq -o fw_hs.tsv
```

Example No.8 - Decompress paired-end fastq files by printing tsv formatted: quality \\tab head

```
sfq -i FwdRevIdx -a d -f "q+h" -t fastq -o fw_qh.tsv
```

Example No.9 - Extract a specific set of records listed in list.file as sequence only format

```
sfq -i FwdRevIdx -a g -f "s" -t fastq -o s.out -l list.file
```

Example No.10 - Compress paired-end fasta file while limiting available RAM to 8 GB

```
sfq -i ./data.in/fa.fa -a c -t fasta -o fa.out -F 8000
```

Example No.11 - Extract a random set of 4 records in sequence + head  format

```
sfq -i ./FwdRevIdx -a g -t fastq -o fa.out -l "rand(4)" -f "s+h"
```


