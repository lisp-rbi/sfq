# SFQ

[![License](https://img.shields.io/badge/license-MIT-blue.svg)]( )

Short description

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
                                               	h+s  	  :head sequence,
                                               	h+s+q  	:head sequence quality,
                                               	s+h+q  	:sequence head quality,
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

Example No.2 - Compress pairend fastq file

```
./target/release/sfq -i ./data/fwd.fq -j ./data/rev.fq -a c -t fastq -o FwdRevIdx
```

Example No.3 - Decompress single stranded fastq files by printing full records

```
./target/release/sfq -i FwdIdx -a d -f fq -t fastq -o fw.fq
```

Example No.4 - Decompress paird-end fastq files by printing full records

```
./target/release/sfq -i FwdRevIdx -a d -f fq -t fastq -o fw_rv.fq
```

Example No.5 - Decompress single stranded fastq files by printing fasta records

```
./target/release/sfq -i FwdIdx -a d -f fa -t fastq -o fw.fa
```

Example No.6 - Decompress pairend fastq files by printing fasta records

```
./target/release/sfq -i FwdRevIdx -a d -f fa -t fastq -o fw-re.fa
```

Example No.7 - Decompress single stranded fastq files by printing tsv formatted: head \\tab seq

```
./target/release/sfq -i FwdIdx -a d -f "h+s" -t fastq -o fw_hs.tsv
```

Example No.8 - Decompress pairend fastq files by printing tsv formatted: quality \\tab head

```
./target/release/sfq -i FwdRevIdx -a d -f "q+h" -t fastq -o fw_qh.tsv
```

Example No.9 - Extract a particular set of records listed in list.file as sequence only format

```
./target/release/sfq -i FwdRevIdx -a g -f "s" -t fastq -o s.out -l list.file
```

Example No.10 - Compress pairend fasta file by limiting memory to 8GB (experimental)

```
./target/release/sfq -i ./data.in/fa.fa -a c -t fasta -o fa.out -M 8000
```


## Benchmarks

Tools:
SPRING
sfq

Input files (./data/):
nova_R1.fq (1.7GB)
nova_R2.fq (1.7GB)


Prepare data:

```
mkdir In
mkdir Out
perl -lnw '$x = 400; $in_1="./data/nova_R1.fq"; $in_2="./data/nova_R2.fq";for(1..5){ $x = $x*10; system("head -n $x $in_1 > In/$_\_$in_1; head -n $x $in_2 > In/$_\_$in_2;")}'
perl -lnw '$x = 400; $in_1="./data/HiSeq_R1.fq"; $in_2="./data/HiSeq_R2.fq";for(1..5){ $x = $x*10; system("head -n $x $in_1 > In/$_\_$in_1; head -n $x $in_2 > In/$_\_$in_2;")}'
```
Benchmark:

```
# [sfq] measuring runtime, memory and filesize
./target/release/sfq -i ./In/1_nova_R1.fq -j ./In/1_nova_R2.fq -a c -t fastq -o Out/1_Nova -M 100000
./target/release/sfq -i ./In/2_nova_R1.fq -j ./In/2_nova_R2.fq -a c -t fastq -o Out/2_Nova -M 100000
./target/release/sfq -i ./In/3_nova_R1.fq -j ./In/3_nova_R2.fq -a c -t fastq -o Out/3_Nova -M 100000
./target/release/sfq -i ./In/4_nova_R1.fq -j ./In/4_nova_R2.fq -a c -t fastq -o Out/4_Nova -M 100000
./target/release/sfq -i ./In/5_nova_R1.fq -j ./In/5_nova_R2.fq -a c -t fastq -o Out/5_Nova -M 100000
# [SPRING] measuring runtime, memory and filesize
spring ....
```




### Runtime analysis


InputSize X Time

### Memory measurments  

InputSize X memory


### Disk usage

 InputSize X CompressedSize
