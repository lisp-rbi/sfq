# SFQ

[![License](https://img.shields.io/badge/license-MIT-blue.svg)]( )

With the advent of next generation sequencing, challenges associate with storage, transmission, and analysis of the generated HTS data have became a major stepping stone, preventing the fast pace research within the field.  Terabytes of uncompressed data per individual per experiment have deprecated transmission protocols down to shipping WGS data via cheap HDD through regular mail. This holds even in cases when files are being compressed with state of the art compression algorithms.

Though compressing the data alleviates storage associated difficulties and downscale the cost of transmission, it does nothing when it comes to analyses. Especially in situations when only a fraction of data is required for executing an analysis. In such cases, each compression strategy requires for an entire dataset to be, at least temporarily, extracted. This introduces two new challenges:

  a) a lower limit on computational resources required when conducting a given analysis   
  b) a breach of regulatory requirements as set by GDPR

SFQ is a succinct data structure for fast(a/q) flat file formatted data sets. In size it rivals even the most advanced compression data models while at the same time supports random access to individual records. Random access is a main prerequisite for achieving strict GDPR compliance and thus SFQ presents the only tool on the market able to support GDPR complaint queries over personal gnomic information. Moreover, the same feature allows for downstream analysis to be executed without "a priori" data assembly as done by most of the compression solutions currently available.

As such SFQ represents a novel solution and a leap in data storage, transmission and analysis, of HTS information.


## Installation protocol


If you’re running macOS, Linux, or another Unix-like OS. To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Current version of the software requires Rust (> v1.38) and g++ (> v4.9.1). `sfq` is being tested on Rust 1.38.0, but it is likely to work on other subsequent versions as well. Once the rust and cargo have been installed, execute the following:

 1 . Download the library:

```
git clone path...
```
 2 . Compile binaries:


```
cd ./sfq/

cargo build --release
```

This will compile the source to `./target/release/`

 3 . Execute binary by running:

```
./target/release/sfq -h
```

## Usage

```



sfq 0.01
Robert Bakaric <rbakaric@irb.hr>, Damir Korencic<dkorencic@irb.hr>

    ______     ______   ______    	
   /\  ___\   /\  ___\ /\  __ \   	
   \ \___  \  \ \  __\ \ \ \/\_\  	
    \/\_____\  \ \_\    \ \___\_\ 	
     \/_____/   \/_/     \/___/_/ 	


            Auth: Bakaric R. Korencic, D. & Ristov, S.

USAGE:
    sfq [OPTIONS]
              --action <c|d|q>
              --compression-mode <complete|lossy>
              --infmt <fastq|fasta|raw>
              --list <file.csv|rand(10)>
              --make-index <hd|>3>
              --max-memory-used <Max|3600,5000>
              --memory-mode <D|R>
              --outfmt <fq|fa|s|q|h|...>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --action <c|d|q>                       Action: (c) compress, (d) decompress, (g) get <requires --list >
                                               [default: c]
    -s, --compression-mode <complete|lossy>    Compression mode [default: complete]
    -t, --infmt <fastq|fasta|raw>              File types supported [default: fastq]
    -i, --input <FILE>                         Input file (fasta,fastq,lzt) [default: stdin]
    -j, --input-rev <FILE>                     Input file of a revers file (fastq)
    -l, --list <file.csv|rand(10)>             Please provide a list of prefixes, records of which are to be extracted
                                               (works only with -a g) [default: rand(10)]
    -y, --make-index <hd|>3>                   Make index (4,5,6,... - kmer size, hd - high dimensional kmer index)
                                               [default: 7]
    -M, --max-memory-used <Max|3600,5000>      Max Memory to be used (in MB, Max - use all available) [default: Max]
    -m, --memory-mode <D|R>                    Memory mode: defines memory type  (D - disc, R - RAM) [default: R]
    -f, --outfmt <fq|fa|s|q|h|...>             Output format:

                                               	fq   	  :fastq,
                                               	fa  	  :fasta,
                                               	s         :sequence,
                                               	q         :quality,
                                               	h  	      :head,
                                               	s+q   	  :sequence quality,
                                               	h+q   	  :head quality,
                                               	h+s  	    :head sequence,
                                               	h+s+q  	  :head sequence quality,
                                               	s+h+q  	  :sequence head quality,
                                               	...
                                                [default: fa]
    -o, --output <FILE>                        Output file: interleved if paired fastq, dict.lzt if compressed [default:
                                               stdout]


```

## Examples

Example No.1 - Compress single stranded fastq file

```
./target/release/sfq -i ./data/fwd.fq -a c -t fastq -o FwdIdx
```

Example No.2 - Compress pair-end fastq file

```
./target/release/sfq -i ./data/fwd.fq -j ./data/rev.fq -a c -t fastq -o FwdRevIdx
```

Example No.3 - Decompress single stranded fastq files by printing full records

```
./target/release/sfq -i FwdIdx -a d -f fq -t fastq -o fw.fq
```

Example No.4 - Decompress pair-end fastq files by printing full records

```
./target/release/sfq -i FwdRevIdx -a d -f fq -t fastq -o fw_rv.fq
```

Example No.5 - Decompress single stranded fastq files by printing fasta records

```
./target/release/sfq -i FwdIdx -a d -f fa -t fastq -o fw.fa
```

Example No.6 - Decompress pair-end fastq files by printing fasta records

```
./target/release/sfq -i FwdRevIdx -a d -f fa -t fastq -o fw-re.fa
```

Example No.7 - Decompress single stranded fastq files by printing tsv formatted: head \\tab seq

```
./target/release/sfq -i FwdIdx -a d -f "h+s" -t fastq -o fw_hs.tsv
```

Example No.8 - Decompress pair-end fastq files by printing tsv formatted: quality \\tab head

```
./target/release/sfq -i FwdRevIdx -a d -f "q+h" -t fastq -o fw_qh.tsv
```

Example No.9 - Extract a particular set of records listed in list.file as sequence only format

```
./target/release/sfq -i FwdRevIdx -a g -f "s" -t fastq -o s.out -l list.file
```

Example No.10 - Compress pair-end fasta file by limiting memory to 8GB (experimental)

```
./target/release/sfq -i ./data.in/fa.fa -a c -t fasta -o fa.out -M 8000
```

Example No.11 - Extract a random set of 4 records in sequence + head  format

```
./target/release/sfq -i ./FwdRevIdx -a g -t fastq -o fa.out -l "rand(4)" -f "s+h"
```

Example No.12 - Extract longest 2  records in sequence + head format

```
```

Example No.13 - Extract shortest 6  records in sequence + quality format

```
```

## Benchmarks

Tools:
SPRING \n
sfq

Input files (./data/):

nova.R1.fq (1.7GB)
nova.R2.fq (1.7GB)
cvrg = ~15x


L2_R1.fq (14.1GB)
L2_R2.fq (14.1GB)
cvrg = ~117x


nova.R1.fq (14.7GB)
nova.R2.fq (14.7GB)
cvrg = ~136x

Prepare data:

```
mkdir In
mkdir Out

perl -e '$x = 400; $in_1="./data/nova_R1.fq"; $in_2="./data/nova_R2.fq";for(1..5){ $x = $x*10; system("head -n $x $in_1 > In/$_\_$in_1; head -n $x $in_2 > In/$_\_$in_2;")}'

perl -e '$x = 4000; $in_1="./data/L2_R1.fq"; $in_2="./data/L2_R2.fq";for(1..5){ $x = $x*10; system("head -n $x $in_1 > In/$_\_$in_1; head -n $x $in_2 > In/$_\_$in_2;")}'

perl -e '$x = 4000; $in_1="./data/H2_R1.fq"; $in_2="./data/H2_R2.fq";for(1..5){ $x = $x*10; system("head -n $x $in_1 > In/$_\_$in_1; head -n $x $in_2 > In/$_\_$in_2;")}'
```

Benchmark:


```
##  NovaSeq cov = 15x (12.71x per base)
#
#  [SPRING] measuring runtime, memory and filesize
#
# Compressing
#
spring -c -i ./In/1_nova_R1.fq ./In/1_nova_R2.fq -o ./Out/1_nova.spring -t 1
spring -c -i ./In/2_nova_R1.fq ./In/2_nova_R2.fq -o ./Out/2_nova.spring -t 1
spring -c -i ./In/3_nova_R1.fq ./In/3_nova_R2.fq -o ./Out/3_nova.spring -t 1
spring -c -i ./In/4_nova_R1.fq ./In/4_nova_R2.fq -o ./Out/4_nova.spring -t 1
spring -c -i ./In/5_nova_R1.fq ./In/5_nova_R2.fq -o ./Out/5_nova.spring -t 1
#
# Decompressing
#
spring -d -i ./Out/1_nova.spring -o ./Out/1_nova.fastq
spring -d -i ./Out/2_nova.spring -o ./Out/2_nova.fastq
spring -d -i ./Out/3_nova.spring -o ./Out/3_nova.fastq
spring -d -i ./Out/4_nova.spring -o ./Out/4_nova.fastq
spring -d -i ./Out/5_nova.spring -o ./Out/5_nova.fastq
#
#  [sfq] measuring runtime, memory and filesize
#
# Compressing
#
sfq -i ./In/1_nova_R1.fq -j ./In/1_nova_R2.fq -a c -t fastq -o ./Out/1_nova.sfq
sfq -i ./In/2_nova_R1.fq -j ./In/2_nova_R2.fq -a c -t fastq -o ./Out/2_nova.sfq
sfq -i ./In/3_nova_R1.fq -j ./In/3_nova_R2.fq -a c -t fastq -o ./Out/3_nova.sfq
sfq -i ./In/4_nova_R1.fq -j ./In/4_nova_R2.fq -a c -t fastq -o ./Out/4_nova.sfq
sfq -i ./In/5_nova_R1.fq -j ./In/5_nova_R2.fq -a c -t fastq -o ./Out/5_nova.sfq
#
# Decompressing
#
sfq -i ./Out/1_nova.sfq -a d -t fastq -f fq -o ./Out/1_nova_R1R2.fq
sfq -i ./Out/2_nova.sfq -a d -t fastq -f fq -o ./Out/2_nova_R1R2.fq
sfq -i ./Out/3_nova.sfq -a d -t fastq -f fq -o ./Out/3_nova_R1R2.fq
sfq -i ./Out/4_nova.sfq -a d -t fastq -f fq -o ./Out/4_nova_R1R2.fq
sfq -i ./Out/5_nova.sfq -a d -t fastq -f fq -o ./Out/5_nova_R1R2.fq
#
##  NovaSeq cov = 116.65x (12.67x per base)
#
#  [SPRING] measuring runtime, memory and filesize
#
# Compressing
#
spring -c -i ./In/1_L2_R1.fq ./In/1_L2_R2.fq -o ./Out/1_L2.spring -t 1
spring -c -i ./In/2_L2_R1.fq ./In/2_L2_R2.fq -o ./Out/2_L2.spring -t 1
spring -c -i ./In/3_L2_R1.fq ./In/3_L2_R2.fq -o ./Out/3_L2.spring -t 1
spring -c -i ./In/4_L2_R1.fq ./In/4_L2_R2.fq -o ./Out/4_L2.spring -t 1
spring -c -i ./In/5_L2_R1.fq ./In/5_L2_R2.fq -o ./Out/5_L2.spring -t 1
#
# Decompressing
#
spring -d -i ./Out/1_L2.spring -o ./Out/1_L2.fastq
spring -d -i ./Out/2_L2.spring -o ./Out/2_L2.fastq
spring -d -i ./Out/3_L2.spring -o ./Out/3_L2.fastq
spring -d -i ./Out/4_L2.spring -o ./Out/4_L2.fastq
spring -d -i ./Out/5_L2.spring -o ./Out/5_L2.fastq
#
#  [sfq] measuring runtime, memory and filesize
#
# Compressing
#
sfq -i ./In/1_L2_R1.fq -j ./In/1_L2_R2.fq -a c -t fastq -o ./Out/1_L2.sfq
sfq -i ./In/2_L2_R1.fq -j ./In/2_L2_R2.fq -a c -t fastq -o ./Out/2_L2.sfq
sfq -i ./In/3_L2_R1.fq -j ./In/3_L2_R2.fq -a c -t fastq -o ./Out/3_L2.sfq
sfq -i ./In/4_L2_R1.fq -j ./In/4_L2_R2.fq -a c -t fastq -o ./Out/4_L2.sfq
sfq -i ./In/5_L2_R1.fq -j ./In/5_L2_R2.fq -a c -t fastq -o ./Out/5_L2.sfq
#
# Decompressing
#
sfq -i ./Out/1_L2.sfq -a d -t fastq -f fq -o ./Out/1_L2_R1R2.fq
sfq -i ./Out/2_L2.sfq -a d -t fastq -f fq -o ./Out/2_L2_R1R2.fq
sfq -i ./Out/3_L2.sfq -a d -t fastq -f fq -o ./Out/3_L2_R1R2.fq
sfq -i ./Out/4_L2.sfq -a d -t fastq -f fq -o ./Out/4_L2_R1R2.fq
sfq -i ./Out/5_L2.sfq -a d -t fastq -f fq -o ./Out/5_L2_R1R2.fq
#
#  HiSeq 2500 cov = 136.15x (variable string length)
#
#  [SPRING] measuring runtime, memory and filesize
#
# Compressing
#
spring -c -i ./In/1_H2_R1.fq ./In/1_H2_R2.fq -o ./Out/1_H2.spring -t 1
spring -c -i ./In/2_H2_R1.fq ./In/2_H2_R2.fq -o ./Out/2_H2.spring -t 1
spring -c -i ./In/3_H2_R1.fq ./In/3_H2_R2.fq -o ./Out/3_H2.spring -t 1
spring -c -i ./In/4_H2_R1.fq ./In/4_H2_R2.fq -o ./Out/4_H2.spring -t 1
spring -c -i ./In/5_H2_R1.fq ./In/5_H2_R2.fq -o ./Out/5_H2.spring -t 1
#
# Decompressing
#
spring -d -i ./Out/1_H2.spring -o ./Out/1_H2.fastq
spring -d -i ./Out/2_H2.spring -o ./Out/2_H2.fastq
spring -d -i ./Out/3_H2.spring -o ./Out/3_H2.fastq
spring -d -i ./Out/4_H2.spring -o ./Out/4_H2.fastq
spring -d -i ./Out/5_H2.spring -o ./Out/5_H2.fastq
#
#  [sfq] measuring runtime, memory and filesize
#
# Compressing
#
sfq -i ./In/1_H2_R1.fq -j ./In/1_H2_R2.fq -a c -t fastq -o ./Out/1_H2.sfq
sfq -i ./In/2_H2_R1.fq -j ./In/2_H2_R2.fq -a c -t fastq -o ./Out/2_H2.sfq
sfq -i ./In/3_H2_R1.fq -j ./In/3_H2_R2.fq -a c -t fastq -o ./Out/3_H2.sfq
sfq -i ./In/4_H2_R1.fq -j ./In/4_H2_R2.fq -a c -t fastq -o ./Out/4_H2.sfq
sfq -i ./In/5_H2_R1.fq -j ./In/5_H2_R2.fq -a c -t fastq -o ./Out/5_H2.sfq
#
# Decompressing
#
sfq -i ./Out/1_H2.sfq -a d -t fastq -f fq -o ./Out/1_H2_R1R2.fq
sfq -i ./Out/2_H2.sfq -a d -t fastq -f fq -o ./Out/2_H2_R1R2.fq
sfq -i ./Out/3_H2.sfq -a d -t fastq -f fq -o ./Out/3_H2_R1R2.fq
sfq -i ./Out/4_H2.sfq -a d -t fastq -f fq -o ./Out/4_H2_R1R2.fq
sfq -i ./Out/5_H2.sfq -a d -t fastq -f fq -o ./Out/5_H2_R1R2.fq
```




### Runtime analysis


InputSize X Time

### Memory measurments  

InputSize X memory


### Disk usage

 InputSize X CompressedSize


## CLI Testing

testing was done utilizing all possible combination of options (limited in -f [i figured it it works for bordering cases it should work for all])



```
#Testing was done utilizing :
perl sfq_bash_test.pl

Testing: sfq -i  in_r1.fq -o  my_out.fa -a  c -M  1000 -t  fasta -m  D   
Testing: sfq -i  in_r1.fq -o  my_out.fa -j  in_r2.fq -a  c -M  1000 -t  fasta -m  D   
Testing: sfq -i  in_r1.fq -o  my_out.fa -a  c -M  1000 -t  fasta -m  R   
Testing: sfq -i  in_r1.fq -o  my_out.fa -j  in_r2.fq -a  c -M  1000 -t  fasta -m  R   
Testing: sfq -i  in_r1.fq -o  my_out.fq -a  c -M  1000 -t  fastq -m  D   
Testing: sfq -i  in_r1.fq -o  my_out.fq -j  in_r2.fq -a  c -M  1000 -t  fastq -m  D   
Testing: sfq -i  in_r1.fq -o  my_out.fq -a  c -M  1000 -t  fastq -m  R   
Testing: sfq -i  in_r1.fq -o  my_out.fq -j  in_r2.fq -a  c -M  1000 -t  fastq -m  R   
Testing: sfq -i  in_r1.fq -o  my_out.fa -a  c -M  10000 -t  fasta -m  D   
Testing: sfq -i  in_r1.fq -o  my_out.fa -j  in_r2.fq -a  c -M  10000 -t  fasta -m  D   
Testing: sfq -i  in_r1.fq -o  my_out.fa -a  c -M  10000 -t  fasta -m  R   
Testing: sfq -i  in_r1.fq -o  my_out.fa -j  in_r2.fq -a  c -M  10000 -t  fasta -m  R   
Testing: sfq -i  in_r1.fq -o  my_out.fq -a  c -M  10000 -t  fastq -m  D   
Testing: sfq -i  in_r1.fq -o  my_out.fq -j  in_r2.fq -a  c -M  10000 -t  fastq -m  D   
Testing: sfq -i  in_r1.fq -o  my_out.fq -a  c -M  10000 -t  fastq -m  R   
Testing: sfq -i  in_r1.fq -o  my_out.fq -j  in_r2.fq -a  c -M  10000 -t  fastq -m  R   
Testing: sfq -i  in_r1.fq -o  my_out.fa -a  c -M  Max -t  fasta -m  D   
Testing: sfq -i  in_r1.fq -o  my_out.fa -j  in_r2.fq -a  c -M  Max -t  fasta -m  D   
Testing: sfq -i  in_r1.fq -o  my_out.fa -a  c -M  Max -t  fasta -m  R   
Testing: sfq -i  in_r1.fq -o  my_out.fa -j  in_r2.fq -a  c -M  Max -t  fasta -m  R   
Testing: sfq -i  in_r1.fq -o  my_out.fq -a  c -M  Max -t  fastq -m  D   
Testing: sfq -i  in_r1.fq -o  my_out.fq -j  in_r2.fq -a  c -M  Max -t  fastq -m  D   
Testing: sfq -i  in_r1.fq -o  my_out.fq -a  c -M  Max -t  fastq -m  R   
Testing: sfq -i  in_r1.fq -o  my_out.fq -j  in_r2.fq -a  c -M  Max -t  fastq -m  R   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  Fa -m  D   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  Fa -m  D -l  "rand(3)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  Fa -m  D -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  Fq -m  D   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  Fq -m  D -l  "rand(12)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  Fq -m  D -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "h" -m  D   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "h" -m  D -l  "rand(17)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "h" -m  D -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "s" -m  D   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "s" -m  D -l  "rand(19)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "s" -m  D -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "q" -m  D   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "q" -m  D -l  "rand(22)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "q" -m  D -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "h+s" -m  D   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "h+s" -m  D -l  "rand(31)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "h+s" -m  D -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "s+h" -m  D   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "s+h" -m  D -l  "rand(15)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "s+h" -m  D -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "h+s+q" -m  D   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "h+s+q" -m  D -l  "rand(32)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "h+s+q" -m  D -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "q+s+h" -m  D   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "q+s+h" -m  D -l  "rand(29)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "q+s+h" -m  D -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "s+s" -m  D   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "s+s" -m  D -l  "rand(6)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "s+s" -m  D -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  Fa -m  D   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  Fa -m  D -l  "rand(4)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  Fa -m  D -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  Fq -m  D   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  Fq -m  D -l  "rand(7)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  Fq -m  D -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "h" -m  D   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "h" -m  D -l  "rand(34)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "h" -m  D -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "s" -m  D   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "s" -m  D -l  "rand(10)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "s" -m  D -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "q" -m  D   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "q" -m  D -l  "rand(14)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "q" -m  D -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "h+s" -m  D   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "h+s" -m  D -l  "rand(33)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "h+s" -m  D -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "s+h" -m  D   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "s+h" -m  D -l  "rand(23)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "s+h" -m  D -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "h+s+q" -m  D   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "h+s+q" -m  D -l  "rand(3)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "h+s+q" -m  D -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "q+s+h" -m  D   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "q+s+h" -m  D -l  "rand(7)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "q+s+h" -m  D -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "s+s" -m  D   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "s+s" -m  D -l  "rand(2)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "s+s" -m  D -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  Fa -m  R   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  Fa -m  R -l  "rand(9)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  Fa -m  R -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  Fq -m  R   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  Fq -m  R -l  "rand(24)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  Fq -m  R -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "h" -m  R   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "h" -m  R -l  "rand(11)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "h" -m  R -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "s" -m  R   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "s" -m  R -l  "rand(26)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "s" -m  R -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "q" -m  R   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "q" -m  R -l  "rand(37)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "q" -m  R -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "h+s" -m  R   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "h+s" -m  R -l  "rand(15)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "h+s" -m  R -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "s+h" -m  R   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "s+h" -m  R -l  "rand(3)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "s+h" -m  R -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "h+s+q" -m  R   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "h+s+q" -m  R -l  "rand(21)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "h+s+q" -m  R -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "q+s+h" -m  R   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "q+s+h" -m  R -l  "rand(23)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "q+s+h" -m  R -l  list.list   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  d -t  fasta -f  "s+s" -m  R   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "s+s" -m  R -l  "rand(34)"   
Testing: sfq -i  my_out.fa -o  my_out.fa -a  g -t  fasta -f  "s+s" -m  R -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  Fa -m  R   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  Fa -m  R -l  "rand(8)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  Fa -m  R -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  Fq -m  R   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  Fq -m  R -l  "rand(27)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  Fq -m  R -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "h" -m  R   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "h" -m  R -l  "rand(1)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "h" -m  R -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "s" -m  R   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "s" -m  R -l  "rand(8)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "s" -m  R -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "q" -m  R   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "q" -m  R -l  "rand(6)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "q" -m  R -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "h+s" -m  R   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "h+s" -m  R -l  "rand(3)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "h+s" -m  R -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "s+h" -m  R   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "s+h" -m  R -l  "rand(4)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "s+h" -m  R -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "h+s+q" -m  R   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "h+s+q" -m  R -l  "rand(2)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "h+s+q" -m  R -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "q+s+h" -m  R   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "q+s+h" -m  R -l  "rand(17)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "q+s+h" -m  R -l  list.list   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  d -t  fastq -f  "s+s" -m  R   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "s+s" -m  R -l  "rand(7)"   
Testing: sfq -i  my_out.fq -o  my_out.fq -a  g -t  fastq -f  "s+s" -m  R -l  list.list   
```
