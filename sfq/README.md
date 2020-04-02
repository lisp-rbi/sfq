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
SPRING   
sfq   

Input files (./data/):   

L2_R1.fq (14.1GB)   
L2_R2.fq (14.1GB)   
cvrg = ~117x   


H2_R1.fq (14.7GB)   
H2_R2.fq (14.7GB)   
cvrg = ~136x   

Prepare data:

```
mkdir In
mkdir Out

##  change $xx = $x*2 to *3 *4 *5

perl -e '$x = 4000; $in_1="../NovaSeq/L2_R1.fq"; $in_2="../NovaSeq/L2_R2.fq";for(1..4){ $x = $x*10; $xx= $x*2; system("head -n $xx $in_1 | tail -n $x > In/$_\_L2_R1.fq; head -n $xx $in_2  | tail -n $x > In/$_\_L2_R2.fq;")}'

perl -e '$x = 4000; $in_1="../HiSeq/H2_R1.fq"; $in_2="../HiSeq/H2_R2.fq";for(1..4){ $x = $x*10;     $xx= $x*2; system("head -n $xx $in_1 | tail -n $x > In/$_\_H2_R1.fq; head -n $xx $in_2 | tail -n $x > In/$_\_H2_R2.fq;")}'

perl -e '$x = 4000; $in_1="../NovaSeq/nova.R1.fq"; $in_2="../NovaSeq/nova.R2.fq";for(1..4){ $x = $x*10; $xx= $x*2; system("head -n $xx $in_1 | tail -n $x > In/$_\_nova_R1.fq; head -n $xx $in_2 | tail -n $x > In/$_\_nova_R2.fq;")}'



## analyisi protocols
# bench full
bench.pl -i sfq_vs_spring.bench  -d 10 -i Run_complete -b 1
# bench lossy
bench.pl -i sfq_lossy.bench  -d 10 -i Run_lossy_1 -b 1


```

Benchmark (sfq_vs_spring.bench):


```

## Plotting:

%TagClasses:     Tool   ,       Data   ,   Process   ,  Mode   ,     Size     ,    IO
#------------------------------------------------------------------------------#
%PlotRuntime: sfq/spring, HiSeq/NovaSeq,   Compress  ,    -    ,    NoLimit   ,     -     : Compression_NoMemLimit
%PlotRuntime: sfq/spring, HiSeq/NovaSeq,  Decompress ,   Ram   ,    NoLimit   ,     -     : Decompression_RAM
%PlotRuntime: sfq/spring, HiSeq/NovaSeq,  Decompress ,   Disc  ,    NoLimit   ,     -     : Decompression_Disc
%PlotRuntime: sfq/spring, HiSeq/NovaSeq,   Compress  ,    -    ,     20GB     ,     -     : Compression_20GBMemLimit
%PlotRuntime: sfq/spring, HiSeq/NovaSeq,  Decompress ,   Ram   ,     20GB     ,     -     : Decompression_Ram_20GBMemLimit
%PlotRuntime: sfq/spring, HiSeq/NovaSeq,  Decompress ,   Disc  ,     20GB     ,     -     : Decompression_Disc_20GBMemLimit
#------------------------------------------------------------------------------#
%PlotDisc:        sfq   ,   NovaSeq    ,  Decompress ,    -    ,       -      ,35/350/3500: SFQ_Compretion_Ratio
%PlotDisc:       spring ,   NovaSeq    ,  Decompress ,    -    ,       -      ,35/350/3500: SPRING_Compretion_Ratio
#------------------------------------------------------------------------------#
%PlotMemory:  sfq/spring, HiSeq/NovaSeq,   Compress  ,    -    ,    NoLimit   ,     -     : Compression_NoMemLimit
%PlotMemory:  sfq/spring, HiSeq/NovaSeq,  Decompress ,   Ram   ,    NoLimit   ,     -     : Decompression_RAM
%PlotMemory:  sfq/spring, HiSeq/NovaSeq,  Decompress ,   Disc  ,    NoLimit   ,     -     : Decompression_Disc
%PlotMemory:  sfq/spring, HiSeq/NovaSeq,   Compress  ,    -    ,     20GB     ,     -     : Compression_NoMemLimit
%PlotMemory:  sfq/spring, HiSeq/NovaSeq,  Decompress ,   Ram   ,     20GB     ,     -     : Decompression_Ram_20GBMemLimit
%PlotMemory:  sfq/spring, HiSeq/NovaSeq,  Decompress ,   Disc  ,     20GB     ,     -     : Decompression_Disc_20GBMemLimit


%FlagClasses: Output, Input
################################################################################
# NovaSeq
################################################################################
#------------------------------------------------------------------------------#
##  NovaSeq cov = 116.65x (12.67x per base)
#
#  [SPRING] measuring runtime, memory and filesize
#
# Compressing
#
%Tags:    spring, NovaSeq, Compress , -, NoLimit,  -
spring -c -i ./In/1_L2_R1.fq ./In/1_L2_R2.fq -o ./Out/1_L2.spring -t 1
%Tags:    spring, NovaSeq, Compress , -, NoLimit,  -
spring -c -i ./In/2_L2_R1.fq ./In/2_L2_R2.fq -o ./Out/2_L2.spring -t 1
%Tags:    spring, NovaSeq, Compress , -, NoLimit,  -
spring -c -i ./In/3_L2_R1.fq ./In/3_L2_R2.fq -o ./Out/3_L2.spring -t 1
%Tags:    spring, NovaSeq, Compress , -, NoLimit,  -
spring -c -i ./In/4_L2_R1.fq ./In/4_L2_R2.fq -o ./Out/4_L2.spring -t 1
#
# Decompressing
#
%Tags:    spring, NovaSeq, Decompress , -, NoLimit,  -
spring -d -i ./Out/1_L2.spring -o ./Out/1_L2.fastq
%Tags:    spring, NovaSeq, Decompress , -, NoLimit,  35
%Flags:   -o,          -i
spring -d -i ./Out/2_L2.spring -o ./Out/2_L2.fastq
%Tags:    spring, NovaSeq, Decompress , -, NoLimit,  350
%Flags:   -o,          -i
spring -d -i ./Out/3_L2.spring -o ./Out/3_L2.fastq
%Tags:    spring, NovaSeq, Decompress , -, NoLimit,  3500
%Flags:   -o,          -i
spring -d -i ./Out/4_L2.spring -o ./Out/4_L2.fastq


#----------------------------------------------#
#  [sfq] measuring runtime, memory and filesize
#----------------------------------------------#
#
# Compressing
#
%Tags:    sfq, NovaSeq, Compress , -, NoLimit,  -
sfq -i ./In/1_L2_R1.fq -j ./In/1_L2_R2.fq -a c -t fastq -o ./Out/1_L2.sfq
%Tags:    sfq, NovaSeq, Compress , -, NoLimit,  -
sfq -i ./In/2_L2_R1.fq -j ./In/2_L2_R2.fq -a c -t fastq -o ./Out/2_L2.sfq
%Tags:    sfq, NovaSeq, Compress , -, NoLimit,  -
sfq -i ./In/3_L2_R1.fq -j ./In/3_L2_R2.fq -a c -t fastq -o ./Out/3_L2.sfq
%Tags:    sfq, NovaSeq, Compress , -, NoLimit,  -
sfq -i ./In/4_L2_R1.fq -j ./In/4_L2_R2.fq -a c -t fastq -o ./Out/4_L2.sfq
#
# Decompressing
#
%Tags:    sfq, NovaSeq, Decompress , Ram, NoLimit,  -
sfq -i ./Out/1_L2.sfq -a d -t fastq -f fq -m R -o ./Out/1_r-L2_R1R2.fq
%Tags:    sfq, NovaSeq, Decompress , Ram, NoLimit,  35
%Flags:   -o,          -i
sfq -i ./Out/2_L2.sfq -a d -t fastq -f fq -m R -o ./Out/2_r-L2_R1R2.fq
%Tags:    sfq, NovaSeq, Decompress , Ram, NoLimit,  350
%Flags:   -o,          -i
sfq -i ./Out/3_L2.sfq -a d -t fastq -f fq -m R -o ./Out/3_r-L2_R1R2.fq
%Tags:    sfq, NovaSeq, Decompress , Ram, NoLimit,  3500
%Flags:   -o,          -i
sfq -i ./Out/4_L2.sfq -a d -t fastq -f fq -m R -o ./Out/4_r-L2_R1R2.fq
#
#
%Tags:    sfq, NovaSeq, Decompress , Disc, NoLimit,  -
sfq -i ./Out/1_L2.sfq -a d -t fastq -f fq -m D -o ./Out/1_d-L2_R1R2.fq
%Tags:    sfq, NovaSeq, Decompress , Disc, NoLimit,  -
sfq -i ./Out/2_L2.sfq -a d -t fastq -f fq -m D -o ./Out/2_d-L2_R1R2.fq
%Tags:    sfq, NovaSeq, Decompress , Disc, NoLimit,  -
sfq -i ./Out/3_L2.sfq -a d -t fastq -f fq -m D -o ./Out/3_d-L2_R1R2.fq
%Tags:    sfq, NovaSeq, Decompress , Disc, NoLimit,  -
sfq -i ./Out/4_L2.sfq -a d -t fastq -f fq -m D -o ./Out/4_d-L2_R1R2.fq



################################################################################
# HiSeq
################################################################################
#------------------------------------------------------------------------------#
#  HiSeq 2500 cov = 136.15x (variable string length)
#
#  [SPRING] measuring runtime, memory and filesize
#
# Compressing
#
%Tags:    spring, HiSeq, Compress , -, NoLimit,  -
spring -c -i ./In/1_H2_R1.fq ./In/1_H2_R2.fq -o ./Out/1_H2.spring -t 1
%Tags:    spring, HiSeq, Compress , -, NoLimit,  -
spring -c -i ./In/2_H2_R1.fq ./In/2_H2_R2.fq -o ./Out/2_H2.spring -t 1
%Tags:    spring, HiSeq, Compress , -, NoLimit,  -
spring -c -i ./In/3_H2_R1.fq ./In/3_H2_R2.fq -o ./Out/3_H2.spring -t 1
%Tags:    spring, HiSeq, Compress , -, NoLimit,  -
spring -c -i ./In/4_H2_R1.fq ./In/4_H2_R2.fq -o ./Out/4_H2.spring -t 1
#
# Decompressing
#
%Tags:    spring, HiSeq, Decompress , -, NoLimit,  -
spring -d -i ./Out/1_H2.spring -o ./Out/1_H2.fastq
%Tags:    spring, HiSeq, Decompress , -, NoLimit,  35
%Flags:   -o,          -i
spring -d -i ./Out/2_H2.spring -o ./Out/2_H2.fastq
%Tags:    spring, HiSeq, Decompress , -, NoLimit,  350
%Flags:   -o,          -i
spring -d -i ./Out/3_H2.spring -o ./Out/3_H2.fastq
%Tags:    spring, HiSeq, Decompress , -, NoLimit,  3500
%Flags:   -o,          -i
spring -d -i ./Out/4_H2.spring -o ./Out/4_H2.fastq



#----------------------------------------------#
#  [sfq] measuring runtime, memory and filesize
#----------------------------------------------#
#
# Compressing
#
%Tags:    sfq, HiSeq, Compress , -, NoLimit,  -
sfq -i ./In/1_H2_R1.fq -j ./In/1_H2_R2.fq -a c -t fastq -o ./Out/1_H2.sfq
%Tags:    sfq, HiSeq, Compress , -, NoLimit,  -
sfq -i ./In/2_H2_R1.fq -j ./In/2_H2_R2.fq -a c -t fastq -o ./Out/2_H2.sfq
%Tags:    sfq, HiSeq, Compress , -, NoLimit,  -
sfq -i ./In/3_H2_R1.fq -j ./In/3_H2_R2.fq -a c -t fastq -o ./Out/3_H2.sfq
%Tags:    sfq, HiSeq, Compress , -, NoLimit,  -
sfq -i ./In/4_H2_R1.fq -j ./In/4_H2_R2.fq -a c -t fastq -o ./Out/4_H2.sfq
#
# Decompressing
#
%Tags:    sfq, HiSeq, Decompress , Ram, NoLimit,  -
sfq -i ./Out/1_H2.sfq -a d -t fastq -f fq -m R -o ./Out/1_r-H2_R1R2.fq
%Tags:    sfq, HiSeq, Decompress , Ram, NoLimit,  35
%Flags:   -o,          -i
sfq -i ./Out/2_H2.sfq -a d -t fastq -f fq -m R -o ./Out/2_r-H2_R1R2.fq
%Tags:    sfq, HiSeq, Decompress , Ram, NoLimit,  350
%Flags:   -o,          -i
sfq -i ./Out/3_H2.sfq -a d -t fastq -f fq -m R -o ./Out/3_r-H2_R1R2.fq
%Tags:    sfq, HiSeq, Decompress , Ram, NoLimit,  3500
%Flags:   -o,          -i
sfq -i ./Out/4_H2.sfq -a d -t fastq -f fq -m R -o ./Out/4_r-H2_R1R2.fq
#
#
%Tags:    sfq, HiSeq, Decompress , Disc, NoLimit,  -
sfq -i ./Out/1_H2.sfq -a d -t fastq -f fq -m D -o ./Out/1_d-H2_R1R2.fq
%Tags:    sfq, HiSeq, Decompress , Disc, NoLimit,  -
sfq -i ./Out/2_H2.sfq -a d -t fastq -f fq -m D -o ./Out/2_d-H2_R1R2.fq
%Tags:    sfq, HiSeq, Decompress , Disc, NoLimit,  -
sfq -i ./Out/3_H2.sfq -a d -t fastq -f fq -m D -o ./Out/3_d-H2_R1R2.fq
%Tags:    sfq, HiSeq, Decompress , Disc, NoLimit,  -
sfq -i ./Out/4_H2.sfq -a d -t fastq -f fq -m D -o ./Out/4_d-H2_R1R2.fq

```

Benchmark (sfq_lossy.bench):


```

## Plotting:

%TagClasses:     Tool   ,       Data   ,   Process   ,  Mode   ,     Size     ,    IO
#------------------------------------------------------------------------------#
%PlotRuntime:    sfq    , HiSeq/NovaSeq,   Compress  ,    -    ,    NoLimit   ,     -     : Compression_NoMemLimit
%PlotRuntime:    sfq    , HiSeq/NovaSeq,  Decompress ,   Ram   ,    NoLimit   ,     -     : Decompression_RAM
%PlotRuntime:    sfq    , HiSeq/NovaSeq,  Decompress ,   Disc  ,    NoLimit   ,     -     : Decompression_Disc
#------------------------------------------------------------------------------#
%PlotDisc:       sfq    ,   NovaSeq    ,  Decompress ,    -    ,       -      ,35/350/3500: SFQ_Compretion_Ratio
#------------------------------------------------------------------------------#
%PlotMemory:     sfq    , HiSeq/NovaSeq,   Compress  ,    -    ,    NoLimit   ,     -     : Compression_NoMemLimit
%PlotMemory:     sfq    , HiSeq/NovaSeq,  Decompress ,   Ram   ,    NoLimit   ,     -     : Decompression_RAM
%PlotMemory:     sfq    , HiSeq/NovaSeq,  Decompress ,   Disc  ,    NoLimit   ,     -     : Decompression_Disc



%FlagClasses: Output, Input


sfq -i ./In/1_L2_R1.fq -j ./In/1_L2_R2.fq -a c -t fastq -o ./Out/1_L2.sfq -s lossy
sfq -i ./In/2_L2_R1.fq -j ./In/2_L2_R2.fq -a c -t fastq -o ./Out/2_L2.sfq -s lossy
sfq -i ./In/3_L2_R1.fq -j ./In/3_L2_R2.fq -a c -t fastq -o ./Out/3_L2.sfq -s lossy
sfq -i ./In/4_L2_R1.fq -j ./In/4_L2_R2.fq -a c -t fastq -o ./Out/4_L2.sfq -s lossy
#
#
sfq -i ./Out/1_L2.sfq -a d -t fastq -f fq -m R -o ./Out/1_r-L2_R1R2.fq -s lossy
sfq -i ./Out/2_L2.sfq -a d -t fastq -f fq -m R -o ./Out/2_r-L2_R1R2.fq -s lossy
sfq -i ./Out/3_L2.sfq -a d -t fastq -f fq -m R -o ./Out/3_r-L2_R1R2.fq -s lossy
sfq -i ./Out/4_L2.sfq -a d -t fastq -f fq -m R -o ./Out/4_r-L2_R1R2.fq -s lossy
#
#
sfq -i ./Out/1_L2.sfq -a d -t fastq -f fq -m D -o ./Out/1_d-L2_R1R2.fq -s lossy
sfq -i ./Out/2_L2.sfq -a d -t fastq -f fq -m D -o ./Out/2_d-L2_R1R2.fq -s lossy
sfq -i ./Out/3_L2.sfq -a d -t fastq -f fq -m D -o ./Out/3_d-L2_R1R2.fq -s lossy
sfq -i ./Out/4_L2.sfq -a d -t fastq -f fq -m D -o ./Out/4_d-L2_R1R2.fq -s lossy
#
#
sfq -i ./In/1_H2_R1.fq -j ./In/1_H2_R2.fq -a c -t fastq -o ./Out/1_H2.sfq -s lossy
sfq -i ./In/2_H2_R1.fq -j ./In/2_H2_R2.fq -a c -t fastq -o ./Out/2_H2.sfq -s lossy
sfq -i ./In/3_H2_R1.fq -j ./In/3_H2_R2.fq -a c -t fastq -o ./Out/3_H2.sfq -s lossy
sfq -i ./In/4_H2_R1.fq -j ./In/4_H2_R2.fq -a c -t fastq -o ./Out/4_H2.sfq -s lossy
#
#
sfq -i ./Out/1_H2.sfq -a d -t fastq -f fq -m R -o ./Out/1_r-H2_R1R2.fq -s lossy
sfq -i ./Out/2_H2.sfq -a d -t fastq -f fq -m R -o ./Out/2_r-H2_R1R2.fq -s lossy
sfq -i ./Out/3_H2.sfq -a d -t fastq -f fq -m R -o ./Out/3_r-H2_R1R2.fq -s lossy
sfq -i ./Out/4_H2.sfq -a d -t fastq -f fq -m R -o ./Out/4_r-H2_R1R2.fq -s lossy
#
#
sfq -i ./Out/1_H2.sfq -a d -t fastq -f fq -m D -o ./Out/1_d-H2_R1R2.fq -s lossy
sfq -i ./Out/2_H2.sfq -a d -t fastq -f fq -m D -o ./Out/2_d-H2_R1R2.fq -s lossy
sfq -i ./Out/3_H2.sfq -a d -t fastq -f fq -m D -o ./Out/3_d-H2_R1R2.fq -s lossy
sfq -i ./Out/4_H2.sfq -a d -t fastq -f fq -m D -o ./Out/4_d-H2_R1R2.fq -s lossy
#





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
#
# Decompressing
#
spring -d -i ./Out/1_nova.spring -o ./Out/1_nova.fastq
spring -d -i ./Out/2_nova.spring -o ./Out/2_nova.fastq
spring -d -i ./Out/3_nova.spring -o ./Out/3_nova.fastq
spring -d -i ./Out/4_nova.spring -o ./Out/4_nova.fastq
#
#  [sfq] measuring runtime, memory and filesize
#
# Compressing
#
sfq -i ./In/1_nova_R1.fq -j ./In/1_nova_R2.fq -a c -t fastq -o ./Out/1_nova.sfq
sfq -i ./In/2_nova_R1.fq -j ./In/2_nova_R2.fq -a c -t fastq -o ./Out/2_nova.sfq
sfq -i ./In/3_nova_R1.fq -j ./In/3_nova_R2.fq -a c -t fastq -o ./Out/3_nova.sfq
sfq -i ./In/4_nova_R1.fq -j ./In/4_nova_R2.fq -a c -t fastq -o ./Out/4_nova.sfq
#
# Decompressing
#
sfq -i ./Out/1_nova.sfq -a d -t fastq -f fq -m R -o ./Out/1_r-nova_R1R2.fq
sfq -i ./Out/2_nova.sfq -a d -t fastq -f fq -m R -o ./Out/2_r-nova_R1R2.fq
sfq -i ./Out/3_nova.sfq -a d -t fastq -f fq -m R -o ./Out/3_r-nova_R1R2.fq
sfq -i ./Out/4_nova.sfq -a d -t fastq -f fq -m R -o ./Out/4_r-nova_R1R2.fq
#
#
sfq -i ./Out/1_nova.sfq -a d -t fastq -f fq -m D -o ./Out/1_d-nova_R1R2.fq
sfq -i ./Out/2_nova.sfq -a d -t fastq -f fq -m D -o ./Out/2_d-nova_R1R2.fq
sfq -i ./Out/3_nova.sfq -a d -t fastq -f fq -m D -o ./Out/3_d-nova_R1R2.fq
sfq -i ./Out/4_nova.sfq -a d -t fastq -f fq -m D -o ./Out/4_d-nova_R1R2.fq
#

```

Execution:

```
bench -f "-i,-o" -o Run_1_out -d 1 -i Run_1 -p sfq_spring.bench
bench -f "-i,-o" -o Run_2_out -d 1 -i Run_2 -p sfq_spring_2.bench
bench -f "-i,-o" -o Run_3_out -d 1 -i Run_3 -p sfq_spring_3.bench
```

Results:

```


```


### Runtime analysis


InputSize X Time

### Memory measurments  

InputSize X memory


### Disk usage

 InputSize X CompressedSize


## CLI Integration Testing

Testing was done by utilizing all possible combination of options (limited in -f [I figured if it works for bordering cases it should work for all cases as well]). At the moment there are 144 implemented tests and counting (this does not include in-library tests)



```
#Testing was done utilizing :
perl sfq_bash_test.pl

Testing: sfq -i  In/n.R1.fa -o  my_out.fa -a  c -M  1000 -t  fasta -m  D -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.fa -j  In/n.R2.fa -a  c -M  1000 -t  fasta -m  D -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.fa -a  c -M  1000 -t  fasta -m  R -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.fa -j  In/n.R2.fa -a  c -M  1000 -t  fasta -m  R -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.fq -a  c -M  1000 -t  fastq -m  D -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.fq -j  In/n.R2.fq -a  c -M  1000 -t  fastq -m  D -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.fq -a  c -M  1000 -t  fastq -m  R -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.fq -j  In/n.R2.fq -a  c -M  1000 -t  fastq -m  R -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.fa -a  c -M  10000 -t  fasta -m  D -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.fa -j  In/n.R2.fa -a  c -M  10000 -t  fasta -m  D -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.fa -a  c -M  10000 -t  fasta -m  R -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.fa -j  In/n.R2.fa -a  c -M  10000 -t  fasta -m  R -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.fq -a  c -M  10000 -t  fastq -m  D -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.fq -j  In/n.R2.fq -a  c -M  10000 -t  fastq -m  D -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.fq -a  c -M  10000 -t  fastq -m  R -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.fq -j  In/n.R2.fq -a  c -M  10000 -t  fastq -m  R -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.fa -a  c -M  Max -t  fasta -m  D -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.fa -j  In/n.R2.fa -a  c -M  Max -t  fasta -m  D -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.fa -a  c -M  Max -t  fasta -m  R -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.fa -j  In/n.R2.fa -a  c -M  Max -t  fasta -m  R -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.fq -a  c -M  Max -t  fastq -m  D -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.fq -j  In/n.R2.fq -a  c -M  Max -t  fastq -m  D -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.fq -a  c -M  Max -t  fastq -m  R -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.fq -j  In/n.R2.fq -a  c -M  Max -t  fastq -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "fa" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "fa" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "fa" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "fq" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "fq" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "fq" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "h" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "h" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "h" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "s" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "s" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "s" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "q" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "q" -m  D -l  "rand(7)" -s  complete  ... FAILED!!
 Error:

 get.rs: 179-FXME: 2
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "q" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "h+s" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "h+s" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "h+s" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "s+h" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "s+h" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "s+h" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "h+s+q" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "h+s+q" -m  D -l  "rand(7)" -s  complete  ... FAILED!!
 Error:

 get.rs: 179-FXME: 203
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "h+s+q" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "q+s+h" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "q+s+h" -m  D -l  "rand(7)" -s  complete  ... FAILED!!
 Error:

 get.rs: 179-FXME: 203
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "q+s+h" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "s+s" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "s+s" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "s+s" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "fa" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "fa" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "fa" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "fq" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "fq" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "fq" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "h" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "h" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "h" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "s" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "s" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "s" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "q" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "q" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "q" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "h+s" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "h+s" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "h+s" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "s+h" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "s+h" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "s+h" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "h+s+q" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "h+s+q" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "h+s+q" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "q+s+h" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "q+s+h" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "q+s+h" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "s+s" -m  D -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "s+s" -m  D -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "s+s" -m  D -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "fa" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "fa" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "fa" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "fq" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "fq" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "fq" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "h" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "h" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "h" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "s" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "s" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "s" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "q" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "q" -m  R -l  "rand(7)" -s  complete  ... FAILED!!
 Error:

 get.rs: 179-FXME: 2
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "q" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "h+s" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "h+s" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "h+s" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "s+h" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "s+h" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "s+h" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "h+s+q" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "h+s+q" -m  R -l  "rand(7)" -s  complete  ... FAILED!!
 Error:

 get.rs: 179-FXME: 203
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "h+s+q" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "q+s+h" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "q+s+h" -m  R -l  "rand(7)" -s  complete  ... FAILED!!
 Error:

 get.rs: 179-FXME: 203
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "q+s+h" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  d -t  fasta -f  "s+s" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "s+s" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fa -o  my_out.fa.interl -a  g -t  fasta -f  "s+s" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "fa" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "fa" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "fa" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "fq" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "fq" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "fq" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "h" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "h" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "h" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "s" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "s" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "s" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "q" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "q" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "q" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "h+s" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "h+s" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "h+s" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "s+h" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "s+h" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "s+h" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "h+s+q" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "h+s+q" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "h+s+q" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "q+s+h" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "q+s+h" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "q+s+h" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  d -t  fastq -f  "s+s" -m  R -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "s+s" -m  R -l  "rand(7)" -s  complete  ... ok!
Testing: sfq -i  my_out.fq -o  my_out.fq.interl -a  g -t  fastq -f  "s+s" -m  R -l  list.list -s  complete  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.lossy.fa -a  c -M  1000 -t  fasta -m  D -s  lossy  ... FAILED!!
 Error:

 Reading data ... get.rs: 179-FXME: 209685
 6.98ms
Compressing ... Bug in common l:140
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  In/n.R1.fa -o  my_out.lossy.fa -j  In/n.R2.fa -a  c -M  1000 -t  fasta -m  D -s  lossy  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.lossy.fa -a  c -M  1000 -t  fasta -m  R -s  lossy  ... FAILED!!
 Error:

 Reading data ... get.rs: 179-FXME: 209685
 2.26ms
Compressing ... Bug in common l:140
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  In/n.R1.fa -o  my_out.lossy.fa -j  In/n.R2.fa -a  c -M  1000 -t  fasta -m  R -s  lossy  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.lossy.fq -a  c -M  1000 -t  fastq -m  D -s  lossy  ... FAILED!!
 Error:

 Reading data ... get.rs: 179-FXME: 356684
 3.69ms
Compressing ... Bug in common l:140
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  In/n.R1.fq -o  my_out.lossy.fq -j  In/n.R2.fq -a  c -M  1000 -t  fastq -m  D -s  lossy  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.lossy.fq -a  c -M  1000 -t  fastq -m  R -s  lossy  ... FAILED!!
 Error:

 Reading data ... get.rs: 179-FXME: 356684
 3.50ms
Compressing ... Bug in common l:140
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  In/n.R1.fq -o  my_out.lossy.fq -j  In/n.R2.fq -a  c -M  1000 -t  fastq -m  R -s  lossy  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.lossy.fa -a  c -M  10000 -t  fasta -m  D -s  lossy  ... FAILED!!
 Error:

 Reading data ... get.rs: 179-FXME: 209685
 2.30ms
Compressing ... Bug in common l:140
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  In/n.R1.fa -o  my_out.lossy.fa -j  In/n.R2.fa -a  c -M  10000 -t  fasta -m  D -s  lossy  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.lossy.fa -a  c -M  10000 -t  fasta -m  R -s  lossy  ... FAILED!!
 Error:

 Reading data ... get.rs: 179-FXME: 209685
 2.28ms
Compressing ... Bug in common l:140
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  In/n.R1.fa -o  my_out.lossy.fa -j  In/n.R2.fa -a  c -M  10000 -t  fasta -m  R -s  lossy  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.lossy.fq -a  c -M  10000 -t  fastq -m  D -s  lossy  ... FAILED!!
 Error:

 Reading data ... get.rs: 179-FXME: 356684
 3.49ms
Compressing ... Bug in common l:140
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  In/n.R1.fq -o  my_out.lossy.fq -j  In/n.R2.fq -a  c -M  10000 -t  fastq -m  D -s  lossy  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.lossy.fq -a  c -M  10000 -t  fastq -m  R -s  lossy  ... FAILED!!
 Error:

 Reading data ... get.rs: 179-FXME: 356684
 3.51ms
Compressing ... Bug in common l:140
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  In/n.R1.fq -o  my_out.lossy.fq -j  In/n.R2.fq -a  c -M  10000 -t  fastq -m  R -s  lossy  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.lossy.fa -a  c -M  Max -t  fasta -m  D -s  lossy  ... FAILED!!
 Error:

 Reading data ... get.rs: 179-FXME: 209685
 2.30ms
Compressing ... Bug in common l:140
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  In/n.R1.fa -o  my_out.lossy.fa -j  In/n.R2.fa -a  c -M  Max -t  fasta -m  D -s  lossy  ... ok!
Testing: sfq -i  In/n.R1.fa -o  my_out.lossy.fa -a  c -M  Max -t  fasta -m  R -s  lossy  ... FAILED!!
 Error:

 Reading data ... get.rs: 179-FXME: 209685
 2.29ms
Compressing ... Bug in common l:140
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  In/n.R1.fa -o  my_out.lossy.fa -j  In/n.R2.fa -a  c -M  Max -t  fasta -m  R -s  lossy  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.lossy.fq -a  c -M  Max -t  fastq -m  D -s  lossy  ... FAILED!!
 Error:

 Reading data ... get.rs: 179-FXME: 356684
 3.52ms
Compressing ... Bug in common l:140
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  In/n.R1.fq -o  my_out.lossy.fq -j  In/n.R2.fq -a  c -M  Max -t  fastq -m  D -s  lossy  ... ok!
Testing: sfq -i  In/n.R1.fq -o  my_out.lossy.fq -a  c -M  Max -t  fastq -m  R -s  lossy  ... FAILED!!
 Error:

 Reading data ... get.rs: 179-FXME: 356684
 3.59ms
Compressing ... Bug in common l:140
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Testing: sfq -i  In/n.R1.fq -o  my_out.lossy.fq -j  In/n.R2.fq -a  c -M  Max -t  fastq -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "fa" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "fa" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "fa" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "fq" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "fq" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "fq" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "h" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "h" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "h" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "s" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "s" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "s" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "q" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "q" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "q" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "h+s" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "h+s" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "h+s" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "s+h" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "s+h" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "s+h" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "h+s+q" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "h+s+q" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "h+s+q" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "q+s+h" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "q+s+h" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "q+s+h" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "s+s" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "s+s" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "s+s" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "fa" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "fa" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "fa" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "fq" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "fq" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "fq" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "h" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "h" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "h" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "s" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "s" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "s" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "q" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "q" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "q" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "h+s" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "h+s" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "h+s" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "s+h" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "s+h" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "s+h" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "h+s+q" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "h+s+q" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "h+s+q" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "q+s+h" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "q+s+h" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "q+s+h" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "s+s" -m  D -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "s+s" -m  D -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "s+s" -m  D -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "fa" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "fa" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "fa" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "fq" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "fq" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "fq" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "h" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "h" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "h" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "s" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "s" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "s" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "q" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "q" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "q" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "h+s" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "h+s" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "h+s" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "s+h" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "s+h" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "s+h" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "h+s+q" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "h+s+q" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "h+s+q" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "q+s+h" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "q+s+h" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "q+s+h" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  d -t  fasta -f  "s+s" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "s+s" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fa -o  my_out.lossy.fa.interl -a  g -t  fasta -f  "s+s" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "fa" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "fa" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "fa" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "fq" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "fq" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "fq" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "h" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "h" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "h" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "s" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "s" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "s" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "q" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "q" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "q" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "h+s" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "h+s" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "h+s" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "s+h" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "s+h" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "s+h" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "h+s+q" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "h+s+q" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "h+s+q" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "q+s+h" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "q+s+h" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "q+s+h" -m  R -l  list.list -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  d -t  fastq -f  "s+s" -m  R -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "s+s" -m  R -l  "rand(7)" -s  lossy  ... ok!
Testing: sfq -i  my_out.lossy.fq -o  my_out.lossy.fq.interl -a  g -t  fastq -f  "s+s" -m  R -l  list.list -s  lossy  ... ok!
In summary:
PASS: 270
FAIL: 18
TOTAL: 288

```
