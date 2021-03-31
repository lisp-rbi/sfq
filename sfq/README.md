# SFQ

[![License](https://img.shields.io/badge/license-MIT-blue.svg)]( )



With the advent of high xx throughput sequencing (HTS), challenges associate with storage, transmission, and analysis of generated HTS data has became a major stepping stone for fast pace processing as that required in clinical diagnostics. Terabytes of uncompressed data per individual/experiment have deprecated transmission protocols down to transactions involving cheap HDD’s and classic post-office delivery. This holds true even for compressed records!

Compression is a process of downsizing the information content of a given record down to its bare minimum, sufficient for full (optimal) reconstruction of the original source. As such its primary focus is on models and functions that can be utilized to achieve this goal. While the functionality is sufficient for designing and implementing general purpose data storage solutions, HTS data facility usually requires more that that. Frequent access to reads from various experiments/samples require specific data sets typically to be kept separately as ZIP-ed files preventing redundancy between files to be utilized in compression. sfq is a succinct data structure for representing fast(a/q) flat file formatted data which is designed not to only store files, but to provide an option to randomly access stored records without "a prior" decompressing the stored file. This is a crucial feature for targeted bioinformatics analyses where only specific records need to be retrieved in order to be processed. Consider an analysis where samples are multiplexed. Usually the first step in handling such data is to extract each derived fastq file, demultiplex samples and compress them individually. With sfq one has the opportunity to capitalize on redundancy between samples, given no separation step is required thus increasing a compression rate while containing the entire batch associated to one experiment in a single file. Moreover, such feature provides an option to directly access and transmit the information over a network form a single source, thus simplifying the design and cost of maintaining HTS storage facilities, while at the same time increasing the yeald of transmitted useful information.

sfq in size rivals even the most advanced compression strategies outperforming current compression algorithms by more than 15% with no memory overhead associated to retrieval and decompression of targeted records and O(N) compression/decompression time (N - the size the input file).

As such sfq represents a novel solution and a leap in data storage, transmission and analysis, of HTS information.


## Installation protocol


If you’re running macOS, Linux, or another Unix-like OS. To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Current version of the software requires Rust (> v1.38) and g++ (> v4.9.1). `sfq` is being tested on Rust 1.38.0, but it is likely to work on other subsequent versions as well. Once the rust and cargo have been installed, execute the following:

 1 . Download the library:

```
git clone https://www.bitbucket.org/mirda_root/fastqlzt.git
```
 2 . Compile binaries:


```
cd ./fastqlzt/sfq/src/

cargo build --release
```

This will compile the source to `./target/release/`

 3 . Execute binary by running:

```
./target/release/sfq -h
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
    -a, --action <c|d|g>                 Action: (c) compress, (d) decompress, (g) grep <requires --list >  [default: c]
    -s, --compression-mode <0-4>         Compression mode [default: 0]
    -F, --fragment-size <Max|integer>    Amount of RAM in MB allocated for the compression. Max = use all available RAM.
                                         [default: Max]
    -t, --infmt <fastq|fasta>            File types supported [default: fastq]
    -i, --input <FILE>                   Input file (fasta,fastq,sfastq)
    -j, --input-rev <FILE>               Filename of a reverse file (fastq, fasta)
    -l, --list <filename|rand(10)>       Please provide a list of prefixes (numbers or ranges), in separate lines. SFQ
                                         returns records associated with the input prefixes. Works only with -a g.
                                         [default: rand(10)]
    -m, --memory-mode <D|R>              Memory mode: defines memory type  (D - disc, R - RAM) [default: D]  [possible
                                         values: D, R]
    -f, --outfmt <fq|fa|s|q|h|...>       Output format: 
                                          
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
    -o, --output <FILE>                  Output file; interleaved if input is two paired end fastq files
    -r, --restart <no|yes>               Restart compression from temporary files. Works only with -a c. NOTE: Temporary
                                         files must be complete and correct! [default: no]


```

## Examples

Example No.1 - Compress single stranded fastq file

```
sfq -i ./data/fwd.fq -a c -t fastq -o FwdIdx
```

Example No.2 - Compress pair-end fastq file

```
sfq -i ./data/fwd.fq -j ./data/rev.fq -a c -t fastq -o FwdRevIdx
```

Example No.3 - Decompress single stranded fastq files by printing full records

```
sfq -i FwdIdx -a d -f fq -t fastq -o fw.fq
```

Example No.4 - Decompress pair-end fastq files by printing full records

```
sfq -i FwdRevIdx -a d -f fq -t fastq -o fw_rv.fq
```

Example No.5 - Decompress single stranded fastq files by printing fasta records

```
sfq -i FwdIdx -a d -f fa -t fastq -o fw.fa
```

Example No.6 - Decompress pair-end fastq files by printing fasta records

```
sfq -i FwdRevIdx -a d -f fa -t fastq -o fw-re.fa
```

Example No.7 - Decompress single stranded fastq files by printing tsv formatted: head \\tab seq

```
sfq -i FwdIdx -a d -f "h+s" -t fastq -o fw_hs.tsv
```

Example No.8 - Decompress pair-end fastq files by printing tsv formatted: quality \\tab head

```
sfq -i FwdRevIdx -a d -f "q+h" -t fastq -o fw_qh.tsv
```

Example No.9 - Extract a particular set of records listed in list.file as sequence only format

```
sfq -i FwdRevIdx -a g -f "s" -t fastq -o s.out -l list.file
```

Example No.10 - Compress pair-end fasta file by limiting memory to 8GB (experimental)

```
sfq -i ./data.in/fa.fa -a c -t fasta -o fa.out -M 8000
```

Example No.11 - Extract a random set of 4 records in sequence + head  format

```
sfq -i ./FwdRevIdx -a g -t fastq -o fa.out -l "rand(4)" -f "s+h"
```


