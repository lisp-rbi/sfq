# SFQ

[![License](https://img.shields.io/badge/license-MIT-blue.svg)]( )



With the advent of high throughput sequencing (HTS), challenges associate with storage, transmission, and analysis of generated HTS data has became a major stepping stone for fast pace processing as that required in clinical diagnostics. Terabytes of uncompressed data per individual/experiment have deprecated transmission protocols down to transactions involving cheap HDD’s and classic post-office delivery. This holds true even for compressed records!

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
cd ./sfq/src/

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
%PlotDisc:        sfq   ,   NovaSeq    ,  Decompress ,    -    ,       -      ,sf35/sf350/sf3500: SFQ_Compretion_Ratio
%PlotDisc:       spring ,   NovaSeq    ,  Decompress ,    -    ,       -      ,sp35/sp350/sp3500: SPRING_Compretion_Ratio
#------------------------------------------------------------------------------#
%PlotMemory:  sfq/spring, HiSeq/NovaSeq,   Compress  ,    -    ,    NoLimit   ,     -     : Compression_NoMemLimit
%PlotMemory:  sfq/spring, HiSeq/NovaSeq,  Decompress ,   Ram   ,    NoLimit   ,     -     : Decompression_RAM
%PlotMemory:  sfq/spring, HiSeq/NovaSeq,  Decompress ,   Disc  ,    NoLimit   ,     -     : Decompression_Disc
%PlotMemory:  sfq/spring, HiSeq/NovaSeq,   Compress  ,    -    ,     20GB     ,     -     : Compression_NoMemLimit
%PlotMemory:  sfq/spring, HiSeq/NovaSeq,  Decompress ,   Ram   ,     20GB     ,     -     : Decompression_Ram_20GBMemLimit
%PlotMemory:  sfq/spring, HiSeq/NovaSeq,  Decompress ,   Disc  ,     20GB     ,     -     : Decompression_Disc_20GBMemLimit


%FlagClasses: Input, Output
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
%Flags: -i, -o
spring -c -i ./In/1_L2_R1.fq ./In/1_L2_R2.fq -o ./Out/1_L2.spring -t 1
%Tags:    spring, NovaSeq, Compress , -, NoLimit,  sp35
%Flags: -i, -o
spring -c -i ./In/2_L2_R1.fq ./In/2_L2_R2.fq -o ./Out/2_L2.spring -t 1
%Tags:    spring, NovaSeq, Compress , -, NoLimit,  sp350
%Flags: -i, -o
spring -c -i ./In/3_L2_R1.fq ./In/3_L2_R2.fq -o ./Out/3_L2.spring -t 1
%Tags:    spring, NovaSeq, Compress , -, NoLimit,  sp3500
%Flags: -i, -o
spring -c -i ./In/4_L2_R1.fq ./In/4_L2_R2.fq -o ./Out/4_L2.spring -t 1
#
# Decompressing
#
%Tags:    spring, NovaSeq, Decompress , -, NoLimit,  -
%Flags: -i, -o
spring -d -i ./Out/1_L2.spring -o ./Out/1_L2.fastq
%Tags:    spring, NovaSeq, Decompress , -, NoLimit,  -
%Flags:   -i,          -o
spring -d -i ./Out/2_L2.spring -o ./Out/2_L2.fastq
%Tags:    spring, NovaSeq, Decompress , -, NoLimit,  -
%Flags:   -i,          -o
spring -d -i ./Out/3_L2.spring -o ./Out/3_L2.fastq
%Tags:    spring, NovaSeq, Decompress , -, NoLimit,  -
%Flags:   -i,          -o
spring -d -i ./Out/4_L2.spring -o ./Out/4_L2.fastq


#----------------------------------------------#
#  [sfq] measuring runtime, memory and filesize
#----------------------------------------------#
#
# Compressing
#
%Tags:    sfq, NovaSeq, Compress , -, NoLimit,  -
%Flags: -i, -o
sfq -i ./In/1_L2_R1.fq -j ./In/1_L2_R2.fq -a c -t fastq -o ./Out/1_L2.sfq
%Tags:    sfq, NovaSeq, Compress , -, NoLimit,  sf35
%Flags: -i, -o
sfq -i ./In/2_L2_R1.fq -j ./In/2_L2_R2.fq -a c -t fastq -o ./Out/2_L2.sfq
%Tags:    sfq, NovaSeq, Compress , -, NoLimit,  sf350
%Flags: -i, -o
sfq -i ./In/3_L2_R1.fq -j ./In/3_L2_R2.fq -a c -t fastq -o ./Out/3_L2.sfq
%Tags:    sfq, NovaSeq, Compress , -, NoLimit,  sf3500
%Flags: -i, -o
sfq -i ./In/4_L2_R1.fq -j ./In/4_L2_R2.fq -a c -t fastq -o ./Out/4_L2.sfq
#
# Decompressing
#
%Tags:    sfq, NovaSeq, Decompress , Ram, NoLimit,  -
%Flags: -i, -o
sfq -i ./Out/1_L2.sfq -a d -t fastq -f fq -m R -o ./Out/1_r-L2_R1R2.fq
%Tags:    sfq, NovaSeq, Decompress , Ram, NoLimit,  -
%Flags:   -i,          -o
sfq -i ./Out/2_L2.sfq -a d -t fastq -f fq -m R -o ./Out/2_r-L2_R1R2.fq
%Tags:    sfq, NovaSeq, Decompress , Ram, NoLimit,  -
%Flags:   -i,          -o
sfq -i ./Out/3_L2.sfq -a d -t fastq -f fq -m R -o ./Out/3_r-L2_R1R2.fq
%Tags:    sfq, NovaSeq, Decompress , Ram, NoLimit,  -
%Flags:   -i,          -o
sfq -i ./Out/4_L2.sfq -a d -t fastq -f fq -m R -o ./Out/4_r-L2_R1R2.fq
#
#
%Tags:    sfq, NovaSeq, Decompress , Disc, NoLimit,  -
%Flags: -i, -o
sfq -i ./Out/1_L2.sfq -a d -t fastq -f fq -m D -o ./Out/1_d-L2_R1R2.fq
%Tags:    sfq, NovaSeq, Decompress , Disc, NoLimit,  -
%Flags: -i, -o
sfq -i ./Out/2_L2.sfq -a d -t fastq -f fq -m D -o ./Out/2_d-L2_R1R2.fq
%Tags:    sfq, NovaSeq, Decompress , Disc, NoLimit,  -
%Flags: -i, -o
sfq -i ./Out/3_L2.sfq -a d -t fastq -f fq -m D -o ./Out/3_d-L2_R1R2.fq
%Tags:    sfq, NovaSeq, Decompress , Disc, NoLimit,  -
%Flags: -i, -o
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
%Flags: -i, -o
spring -c -i ./In/1_H2_R1.fq ./In/1_H2_R2.fq -o ./Out/1_H2.spring -t 1
%Tags:    spring, HiSeq, Compress , -, NoLimit,  -
%Flags: -i, -o
spring -c -i ./In/2_H2_R1.fq ./In/2_H2_R2.fq -o ./Out/2_H2.spring -t 1
%Tags:    spring, HiSeq, Compress , -, NoLimit,  -
%Flags: -i, -o
spring -c -i ./In/3_H2_R1.fq ./In/3_H2_R2.fq -o ./Out/3_H2.spring -t 1
%Tags:    spring, HiSeq, Compress , -, NoLimit,  -
%Flags: -i, -o
spring -c -i ./In/4_H2_R1.fq ./In/4_H2_R2.fq -o ./Out/4_H2.spring -t 1
#
# Decompressing
#
%Tags:    spring, HiSeq, Decompress , -, NoLimit,  -
%Flags: -i, -o
spring -d -i ./Out/1_H2.spring -o ./Out/1_H2.fastq
%Tags:    spring, HiSeq, Decompress , -, NoLimit,  -
%Flags:   -i,          -o
spring -d -i ./Out/2_H2.spring -o ./Out/2_H2.fastq
%Tags:    spring, HiSeq, Decompress , -, NoLimit,  -
%Flags:   -i,          -o
spring -d -i ./Out/3_H2.spring -o ./Out/3_H2.fastq
%Tags:    spring, HiSeq, Decompress , -, NoLimit,  -
%Flags:   -i,          -o
spring -d -i ./Out/4_H2.spring -o ./Out/4_H2.fastq



#----------------------------------------------#
#  [sfq] measuring runtime, memory and filesize
#----------------------------------------------#
#
# Compressing
#
%Tags:    sfq, HiSeq, Compress , -, NoLimit,  -
%Flags: -i, -o
sfq -i ./In/1_H2_R1.fq -j ./In/1_H2_R2.fq -a c -t fastq -o ./Out/1_H2.sfq
%Tags:    sfq, HiSeq, Compress , -, NoLimit,  -
%Flags: -i, -o
sfq -i ./In/2_H2_R1.fq -j ./In/2_H2_R2.fq -a c -t fastq -o ./Out/2_H2.sfq
%Tags:    sfq, HiSeq, Compress , -, NoLimit,  -
%Flags: -i, -o
sfq -i ./In/3_H2_R1.fq -j ./In/3_H2_R2.fq -a c -t fastq -o ./Out/3_H2.sfq
%Tags:    sfq, HiSeq, Compress , -, NoLimit,  -
%Flags: -i, -o
sfq -i ./In/4_H2_R1.fq -j ./In/4_H2_R2.fq -a c -t fastq -o ./Out/4_H2.sfq
#
# Decompressing
#
%Tags:    sfq, HiSeq, Decompress , Ram, NoLimit,  -
%Flags: -i, -o
sfq -i ./Out/1_H2.sfq -a d -t fastq -f fq -m R -o ./Out/1_r-H2_R1R2.fq
%Tags:    sfq, HiSeq, Decompress , Ram, NoLimit,  sf35
%Flags:   -i,          -o
sfq -i ./Out/2_H2.sfq -a d -t fastq -f fq -m R -o ./Out/2_r-H2_R1R2.fq
%Tags:    sfq, HiSeq, Decompress , Ram, NoLimit,  sf350
%Flags:   -i,          -o
sfq -i ./Out/3_H2.sfq -a d -t fastq -f fq -m R -o ./Out/3_r-H2_R1R2.fq
%Tags:    sfq, HiSeq, Decompress , Ram, NoLimit,  sf3500
%Flags:   -i,          -o
sfq -i ./Out/4_H2.sfq -a d -t fastq -f fq -m R -o ./Out/4_r-H2_R1R2.fq
#
#
%Tags:    sfq, HiSeq, Decompress , Disc, NoLimit,  -
%Flags: -i, -o
sfq -i ./Out/1_H2.sfq -a d -t fastq -f fq -m D -o ./Out/1_d-H2_R1R2.fq
%Tags:    sfq, HiSeq, Decompress , Disc, NoLimit,  -
%Flags: -i, -o
sfq -i ./Out/2_H2.sfq -a d -t fastq -f fq -m D -o ./Out/2_d-H2_R1R2.fq
%Tags:    sfq, HiSeq, Decompress , Disc, NoLimit,  -
%Flags: -i, -o
sfq -i ./Out/3_H2.sfq -a d -t fastq -f fq -m D -o ./Out/3_d-H2_R1R2.fq
%Tags:    sfq, HiSeq, Decompress , Disc, NoLimit,  -
%Flags: -i, -o
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

![Results](https://bitbucket.org/mirda_root/fastqlzt/downloads/Complete.results.png)


 * Compression ration was calculated using complete dataset including Headers
```

# Runtime

#Tool	Data	Process	Mode	Size	IO	TotTime
spring	NovaSeq	Compress	-	NoLimit	-	1.314435
spring	NovaSeq	Compress	-	NoLimit	sp35	12.408721
spring	NovaSeq	Compress	-	NoLimit	sp350	114.348113
spring	NovaSeq	Compress	-	NoLimit	sp3500	824.884892
spring	NovaSeq	Decompress	-	NoLimit	-	0.311604
spring	NovaSeq	Decompress	-	NoLimit	-	3.248917
spring	NovaSeq	Decompress	-	NoLimit	-	13.010098
spring	NovaSeq	Decompress	-	NoLimit	-	62.756803
sfq	NovaSeq	Compress	-	NoLimit	-	5.35513
sfq	NovaSeq	Compress	-	NoLimit	sf35	98.56705
sfq	NovaSeq	Compress	-	NoLimit	sf350	1640.892372
sfq	NovaSeq	Compress	-	NoLimit	sf3500	25592.704953
sfq	NovaSeq	Decompress	Ram	NoLimit	-	2.263316
sfq	NovaSeq	Decompress	Ram	NoLimit	-	33.171961
sfq	NovaSeq	Decompress	Ram	NoLimit	-	373.64435
sfq	NovaSeq	Decompress	Ram	NoLimit	-	4184.688771
sfq	NovaSeq	Decompress	Disc	NoLimit	-	3.550235
sfq	NovaSeq	Decompress	Disc	NoLimit	-	54.516524
sfq	NovaSeq	Decompress	Disc	NoLimit	-	675.640949
sfq	NovaSeq	Decompress	Disc	NoLimit	-	8071.721965
spring	HiSeq	Compress	-	NoLimit	-	0.65684
spring	HiSeq	Compress	-	NoLimit	-	5.197979
spring	HiSeq	Compress	-	NoLimit	-	67.285613
spring	HiSeq	Compress	-	NoLimit	-	939.648139
spring	HiSeq	Decompress	-	NoLimit	-	0.238969
spring	HiSeq	Decompress	-	NoLimit	-	2.437223
spring	HiSeq	Decompress	-	NoLimit	-	9.176422
spring	HiSeq	Decompress	-	NoLimit	-	68.923665
sfq	HiSeq	Compress	-	NoLimit	-	4.175356
sfq	HiSeq	Compress	-	NoLimit	-	68.826063
sfq	HiSeq	Compress	-	NoLimit	-	1053.707942
sfq	HiSeq	Compress	-	NoLimit	-	15288.47841
sfq	HiSeq	Decompress	Ram	NoLimit	-	2.180624
sfq	HiSeq	Decompress	Ram	NoLimit	sf35	30.193404
sfq	HiSeq	Decompress	Ram	NoLimit	sf350	337.530509
sfq	HiSeq	Decompress	Ram	NoLimit	sf3500	3750.55229
sfq	HiSeq	Decompress	Disc	NoLimit	-	3.149178
sfq	HiSeq	Decompress	Disc	NoLimit	-	47.046225
sfq	HiSeq	Decompress	Disc	NoLimit	-	582.450215
sfq	HiSeq	Decompress	Disc	NoLimit	-	6894.860908


#Memory

#Tool	Data	Process	Mode	Size	IO	MemMax(MB)
spring	NovaSeq	Compress	-	NoLimit	-	1
spring	NovaSeq	Compress	-	NoLimit	sp35	360697856.00
spring	NovaSeq	Compress	-	NoLimit	sp350	520544256.00
spring	NovaSeq	Compress	-	NoLimit	sp3500	1562038272.00
spring	NovaSeq	Decompress	-	NoLimit	-	1
spring	NovaSeq	Decompress	-	NoLimit	-	325021696.00
spring	NovaSeq	Decompress	-	NoLimit	-	1868050432.00
spring	NovaSeq	Decompress	-	NoLimit	-	3702935552.00
sfq	NovaSeq	Compress	-	NoLimit	-	274587648.00
sfq	NovaSeq	Compress	-	NoLimit	sf35	2410684416.00
sfq	NovaSeq	Compress	-	NoLimit	sf350	22849122304.00
sfq	NovaSeq	Compress	-	NoLimit	sf3500	247594078208.00
sfq	NovaSeq	Decompress	Ram	NoLimit	-	65159168.00
sfq	NovaSeq	Decompress	Ram	NoLimit	-	115138560.00
sfq	NovaSeq	Decompress	Ram	NoLimit	-	246566912.00
sfq	NovaSeq	Decompress	Ram	NoLimit	-	1333821440.00
sfq	NovaSeq	Decompress	Disc	NoLimit	-	63426560.00
sfq	NovaSeq	Decompress	Disc	NoLimit	-	97697792.00
sfq	NovaSeq	Decompress	Disc	NoLimit	-	109510656.00
sfq	NovaSeq	Decompress	Disc	NoLimit	-	111230976.00
spring	HiSeq	Compress	-	NoLimit	-	1
spring	HiSeq	Compress	-	NoLimit	-	353136640.00
spring	HiSeq	Compress	-	NoLimit	-	545492992.00
spring	HiSeq	Compress	-	NoLimit	-	1582522368.00
spring	HiSeq	Decompress	-	NoLimit	-	1
spring	HiSeq	Decompress	-	NoLimit	-	311009280.00
spring	HiSeq	Decompress	-	NoLimit	-	1791750144.00
spring	HiSeq	Decompress	-	NoLimit	-	3770601472.00
sfq	HiSeq	Compress	-	NoLimit	-	249999360.00
sfq	HiSeq	Compress	-	NoLimit	-	2237997056.00
sfq	HiSeq	Compress	-	NoLimit	-	21727391744.00
sfq	HiSeq	Compress	-	NoLimit	-	234954555392.00
sfq	HiSeq	Decompress	Ram	NoLimit	-	69664768.00
sfq	HiSeq	Decompress	Ram	NoLimit	sf35	110338048.00
sfq	HiSeq	Decompress	Ram	NoLimit	sf350	255045632.00
sfq	HiSeq	Decompress	Ram	NoLimit	sf3500	1666015232.00
sfq	HiSeq	Decompress	Disc	NoLimit	-	68227072.00
sfq	HiSeq	Decompress	Disc	NoLimit	-	98926592.00
sfq	HiSeq	Decompress	Disc	NoLimit	-	108351488.00
sfq	HiSeq	Decompress	Disc	NoLimit	-	109764608.00


* Value of 1 indicates short lived process memory usage of which wan not recorded

# Disc

#Tool	Data	Process	Mode	Size	IO	DiscUsageFlags:Input	Output
spring	NovaSeq	Compress	-	NoLimit	-	7252960	839680
spring	NovaSeq	Compress	-	NoLimit	sp35	72499216	7628800
spring	NovaSeq	Compress	-	NoLimit	sp350	724934628	58112000
spring	NovaSeq	Compress	-	NoLimit	sp3500	7249746500	399431680
spring	NovaSeq	Decompress	DR	NoLimit	-	839680	7252960
spring	NovaSeq	Decompress	DR	NoLimit	-	7628800	72499216
spring	NovaSeq	Decompress	DR	NoLimit	-	58112000	724934628
spring	NovaSeq	Decompress	DR	NoLimit	-	399431680	7249746500
sfq	NovaSeq	Compress	-	NoLimit	-	7252960	1661026
sfq	NovaSeq	Compress	-	NoLimit	sf35	72499216	15347138
sfq	NovaSeq	Compress	-	NoLimit	sf350	724934628	139345172
sfq	NovaSeq	Compress	-	NoLimit	sf3500	7249746500	1224129828
sfq	NovaSeq	Decompress	Ram	NoLimit	-	1661026	7272960
sfq	NovaSeq	Decompress	Ram	NoLimit	-	15347138	72699216
sfq	NovaSeq	Decompress	Ram	NoLimit	-	139345172	726934628
sfq	NovaSeq	Decompress	Ram	NoLimit	-	1224129828	7269746500
sfq	NovaSeq	Decompress	Disc	NoLimit	-	1661026	7272960
sfq	NovaSeq	Decompress	Disc	NoLimit	-	15347138	72699216
sfq	NovaSeq	Decompress	Disc	NoLimit	-	139345172	726934628
sfq	NovaSeq	Decompress	Disc	NoLimit	-	1224129828	7269746500
spring	HiSeq	Compress	-	NoLimit	-	6411604	593920
spring	HiSeq	Compress	-	NoLimit	-	64411192	5703680
spring	HiSeq	Compress	-	NoLimit	-	649221192	57528320
spring	HiSeq	Compress	-	NoLimit	-	6533764532	593039360
spring	HiSeq	Decompress	DR	NoLimit	-	593920	6396038
spring	HiSeq	Decompress	DR	NoLimit	-	5703680	64235422
spring	HiSeq	Decompress	DR	NoLimit	-	57528320	647837634
spring	HiSeq	Decompress	DR	NoLimit	-	593039360	6519102018
sfq	HiSeq	Compress	-	NoLimit	-	6411604	1497349
sfq	HiSeq	Compress	-	NoLimit	-	64411192	14268545
sfq	HiSeq	Compress	-	NoLimit	-	649221192	149079430
sfq	HiSeq	Compress	-	NoLimit	-	6533764532	1556927028
sfq	HiSeq	Decompress	Ram	NoLimit	-	1497349	6416038
sfq	HiSeq	Decompress	Ram	NoLimit	sf35	14268545	64435422
sfq	HiSeq	Decompress	Ram	NoLimit	sf350	149079430	649837634
sfq	HiSeq	Decompress	Ram	NoLimit	sf3500	1556927028	6539102018
sfq	HiSeq	Decompress	Disc	NoLimit	-	1497349	6416038
sfq	HiSeq	Decompress	Disc	NoLimit	-	14268545	64435422
sfq	HiSeq	Decompress	Disc	NoLimit	-	149079430	649837634
sfq	HiSeq	Decompress	Disc	NoLimit	-	1556927028	6539102018

```

Random Access VS Sequential Access   


Plot illustrated Runtime preformance of random access to 1000 records as a function of number of compressed records. Illustration was repeated 5 times with averages being only plotted



![Results](https://bitbucket.org/mirda_root/fastqlzt/downloads/RandomAccVSSeqAcc.png)


SSD Access VS HDD Access   


Plot illustrated Runtime performance associated to access to records stored on SSD  vs those stored on HDD. No difference was observed



![Results](https://bitbucket.org/mirda_root/fastqlzt/downloads/Runtime_SSD_vs_HDD.svg.png)







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
