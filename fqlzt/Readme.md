# fqltz
Fast(Q/A) compression and random access library

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/RobertBakaric/susq-rust/blob/master/LICENSE)

Short description

## Installation

To install fqlzt, first install Rust (> v1.38) and g++ (> v4.9.1). fqlzt is currently tested on Rust 1.38.0, but it is likely to work on other subsequent versions as well.

1. Download the library:

```
git clone path...
```
2. Install:

To install the library please go to:
```
cd ./fqlzt/
```
and run:

```
cargo install fqlzt
```

This will install fqlzt library which than ca be used in your personal projects.


## Compile binary

To create new binaries please go to your local `fqlzt/` folder and  run:

```
cargo build --release

```

After build completes, binaries will be located in `./target/release/`
and can be executed by running:

```
./target/release/fqlzt
```

## Usage

Once the binaries have been compiled, accessing quick
help menu can be achieved by executing:


```

./target/release/fqlzt -h

fqlzt 0.01
Robert Bakaric <rbakaric@irb.hr>, Damir Korencic<dkorencic@irb.hr>

 ________ ________  ___       ________  _________    	
|\  _____\\   __  \|\  \     |\_____  \|\___   ___\    	
\ \  \__/\ \  \|\  \ \  \     \|___/  /\|___ \  \_|    	
 \ \   __\\ \  \\\  \ \  \        /  / /    \ \  \     	
  \ \  \_| \ \  \\\  \ \  \____  /  /_/__    \ \  \    	
   \ \__\   \ \_____  \ \_______\\________\   \ \__\   	
    \|__|    \|___| \__\|_______|\|_______|    \|__|   	
                   \|__|                               	

            Auth: Bakaric R. Korencic, D. & Ristov, S.

USAGE:
    fqlzt [OPTIONS] --action <c|d|e>
                    --direction <bi|fwd|rew>
                    --format <H+F+R+Fq+Rq|...>
                    --ftype <fastq|fasta|raw>
                    --list <file.csv|rand(10)>
                    --memory-mode <0|1|2|..|5>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --action <c|d|e>              Action: (c) compress, (d) decompress, (e) extract <requires --list >  [default: c]
    -d, --direction <bi|fwd|rew>      Condition to be met when extraction (--action e ) is invoked! [default: bi]
    -f, --format <H+F+R+Fq+Rq|...>    Types of reformatting supported [default: H(F,R,Fq,Rq)]
    -t, --ftype <fastq|fasta|raw>     File types supported [default: fastq]
    -i, --input <FILE>                Input file [fasta,fastq,lzt] [default: stdin]
    -l, --list <file.csv|rand(10)>    Please provide a list of prefixes, records of which are to be extracted (works
                                      only with -a e) [default: rand(10)]
    -m, --memory-mode <0|1|2|..|5>    Memory mode: defines memory sparsity level [0- low,5- high] [default: 0]
    -o, --output <FILE>               Output file [default: dict.lzt]

```
As a result a short list of options is printed out. To see the complete documentation  
manual please compile the doc by executing:

```
cargo doc
```
This will generate full list of documentation manuals for each library used in the
project including documenataion manula for `fqlzt` which is located at:

```
./target/doc/fqlzt/all.html
```

In order to compress a simple fastq file using a default data model, the following
needs to be executed:

```
./target/release/fqlzt -a c -i ./example/data/fq.tmp -o compressed

# Result:

compressed.lzt
```
Options used in this example are:
 -a: action(c=compress)  
 -i: ./relative/or/abs/input/file.path
 -o: output file (program will split the output according to the model provided
     def. model = "H(R,F,Rq,Fq)")

### To be implemented !

# de(compress)
  fqlzt -i file.fq.lzt -a d -o file.fq

# extract (random access)
  fqlzt -i file.fq.lzt -a e -o file.fq -l list.csv -d bi
  fqlzt -i file.fq.lzt -a e -o file.fq -l "rand(15)" -d fwd


```


## License

The software is licensed under the  [MIT license](http://opensource.org/licenses/MIT).
