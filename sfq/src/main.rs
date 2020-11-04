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
use std::alloc::System;
use std::process;

#[global_allocator]
static A: System = System;


//#[cfg(test)]
//mod tests;

// modules
mod cli;
mod util;
mod cmd;


// calls
use cli::parse_cli;
use cmd::compress::{
    compress
};
use cmd::decompress::{
    extract
};
use cmd::grep::{
    export
};

fn main() {

    let cli = parse_cli();
    if (cli.occurrences_of("input") == 0) && (cli.occurrences_of("input-rev") == 0) && 
       (cli.occurrences_of("action") == 0) && (cli.occurrences_of("compression-mode") == 0) && 
       (cli.occurrences_of("fragment-size") == 0) && (cli.occurrences_of("infmt") == 0) && 
       (cli.occurrences_of("list") == 0) && (cli.occurrences_of("memory-mode") == 0) && 
       (cli.occurrences_of("outfmt") == 0) && (cli.occurrences_of("output") == 0) {
        print_help();
        process::exit(0);
    }

    match cli.value_of("action") {

        Some(x) => {

            match x {
                "c" => {

                    if compress(cli) == false {
                        panic!("Compression failed!!");
                    };

                },
                "d" => {

                    if extract(cli) == false {
                        panic!("Decompression failed!!");
                    };

                },
                "g" => {
                    if export(cli) == false {
                        panic!("Record Id not recognized!");
                    };

                },
                _   => {
                    panic!("Unrecognized action: -a {}", cli.value_of("action").unwrap());
                }
            }
        },
        None => {
            println!("Please specify the action: -a option!");
        }

    }


}


fn print_help(){
    println!("sfq 0.2.2");
    println!("Robert Bakaric <rbakaric@irb.hr>, Dalibor Hrsak <dalibor.hrsak@irb.hr>, Damir Korencic<dkorencic@irb.hr>\n");
    println!("       ______     ______   ______    	");
    println!("      /\\  ___\\   /\\  ___\\ /\\  __ \\   	");
    println!("      \\ \\___  \\  \\ \\  __\\ \\ \\ \\/\\_\\  	");
    println!("       \\/\\_____\\  \\ \\_\\    \\ \\___\\_\\ 	");
    println!("        \\/_____/   \\/_/     \\/___/_/ \n\n");	
    println!("   Authors: Bakaric R., Hrsak D., Korencic, D. & Ristov, S.\n");
    println!("USAGE:");
    println!("    sfq [OPTIONS] --action <c|d|q> --compression-mode <complete|lossy> --fragment-size <Max|3600,5000> --infmt <fastq|fasta> --input <FILE> --list <file.csv|rand(10)> --memory-mode <D|R> --outfmt <fq|fa|s|q|h|...>");
    println!("FLAGS:");
    println!("    -h, --help       Prints help information");
    println!("    -V, --version    Prints version information\n");
    println!("OPTIONS:");
    println!("    -a, --action <c|d|q>                       Action: (c) compress, (d) decompress, (q) get <requires --list >");
    println!("                                               [default: c]");
    println!("    -s, --compression-mode <complete|lossy>    Compression mode [default: complete]");
    println!("    -F, --fragment-size <Max|3600,5000>        Number of lines to be processed at a time (Max - use all available)");
    println!("                                               [default: Max]");
    println!("    -t, --infmt <fastq|fasta>                  File types supported [default: fastq]");
    println!("    -i, --input <FILE>                         Input file (fasta,fastq,lzt) [default: stdin]");
    println!("    -j, --input-rev <FILE>                     Input file of a revers file (fastq)");
    println!("    -l, --list <file.csv|rand(10)>             Please provide a list of prefixes, records of which are to be extracted");
    println!("                                               (works only with -a g) [default: rand(10)]");
    println!("    -m, --memory-mode <D|R>                    Memory mode: defines memory type  (D - disc, R - RAM) [default: D]");
    println!("    -f, --outfmt <fq|fa|s|q|h|...>             Output format: \n");
    println!("                                               	fq   	:fastq, ");
    println!("                                               	fa  	:fasta, ");
    println!("                                               	s  	:sequence, ");
    println!("                                               	q  	:quality, ");
    println!("                                               	h  	:head, ");
    println!("                                               	s+q  	:sequence quality, ");
    println!("                                               	h+q  	:head quality, ");
    println!("                                               	h+s  	:head sequence, ");
    println!("                                               	h+s+q  	:head sequence quality, ");
    println!("                                               	s+h+q  	:sequence head quality, ");
    println!("                                               	...");
    println!("                                                [default: fq]");
    println!("    -o, --output <FILE>                        Output file: interleaved if paired fastq, dict.lzt if compressed");

}

