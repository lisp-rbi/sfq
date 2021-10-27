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
       (cli.occurrences_of("outfmt") == 0) && (cli.occurrences_of("output") == 0) &&
       (cli.occurrences_of("decompress-exponent") == 0) && (cli.occurrences_of("restart") == 0) {
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
    println!("sfq 0.3.0");
    println!("Robert Bakaric <rbakaric@irb.hr>, Dalibor Hrsak <dalibor.hrsak@irb.hr>, Damir Korencic <dkorencic@irb.hr>\n");
    println!("    ______     ______   ______    	");
    println!("   /\\  ___\\   /\\  ___\\ /\\  __ \\   	");
    println!("   \\ \\___  \\  \\ \\  __\\ \\ \\ \\/\\_\\  	");
    println!("    \\/\\_____\\  \\ \\_\\    \\ \\___\\_\\ 	");
    println!("     \\/_____/   \\/_/     \\/___/_/ \n\n");	
    println!("            Authors: Bakaric R., Hrsak D., Korencic, D. & Ristov, S.\n");
    println!("USAGE:");
    println!("    sfq [OPTIONS]\n");
    println!("FLAGS:");
    println!("    -h, --help       Prints help information");
    println!("    -V, --version    Prints version information\n");
    println!("OPTIONS:");
    println!("    -a, --action <c|d|g>                   Action: (c) compress, (d) decompress, (g) grep <requires --list >  [default:");
    println!("                                           c]");
    println!("    -s, --compression-mode <0-4>           Compression mode. Lossless: 0, lossy: 1 to 4. [default: 0]");
    println!("    -F, --fragment-size <Max|<integer>>    Amount of RAM in MB allocated for the compression. Max = use all available");
    println!("                                           RAM. [default: Max]");
    println!("    -t, --infmt <fastq|fasta>              File types supported [default: fastq]");
    println!("    -i, --input <FILE>                     Input file (fasta,fastq,sfastq)");
    println!("    -j, --input-rev <FILE>                 Filename of a reverse file (fastq, fasta)");
    println!("    -l, --list <filename|\"rand(N)\">        Please provide a file with a list of prefixes (numbers or ranges) or choose N");
    println!("                                           random prefixes with \"rand(N)\". SFQ returns records associated with the input");
    println!("                                           prefixes. Works only with -a g. [default: rand(10)]");
    println!("    -m, --memory-mode <D|R>                Memory mode: defines memory type  (D - disk, R - RAM) [default: D]  [possible");
    println!("                                           values: D, R]");
    println!("    -f, --outfmt <fq|fa|s|q|h|...>         Output format when decompressing or grepping.");
    println!("                                           Options s+q, h+q, etc. produce TSV format.\n");
    println!("                                           	fq   	:fastq, ");
    println!("                                           	fa  	:fasta, ");
    println!("                                           	s  	:sequence, ");
    println!("                                           	q  	:quality, ");
    println!("                                           	h  	:head, ");
    println!("                                           	s+q  	:sequence quality, ");
    println!("                                           	h+q  	:head quality, ");
    println!("                                           	h+s  	:head sequence, ");
    println!("                                           	h+s+q  	:head sequence quality, ");
    println!("                                           	s+h+q  	:sequence head quality, ");
    println!("                                           	...");
    println!("                                            [default: fq]");
    println!("    -o, --output <FILE>                    Output file; interleaved if input is two paired end fastq files");
    println!("    -r, --restart <no|yes>                 Restart compression from temporary files. Works only with -a c. NOTE:");
    println!("                                           Temporary files must be complete and correct! [default: no]");
}

