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



use clap::*;
use std::str;

pub(crate) fn parse_cli ()->  clap::ArgMatches<'static> {

    let head : &str =" \t
    ______     ______   ______    \t
   /\\  ___\\   /\\  ___\\ /\\  __ \\   \t
   \\ \\___  \\  \\ \\  __\\ \\ \\ \\/\\_\\  \t
    \\/\\_____\\  \\ \\_\\    \\ \\___\\_\\ \t
     \\/_____/   \\/_/     \\/___/_/ \t
                                  \t
                                                      \t
            Authors: Bakaric R., Hrsak D., Korencic, D. & Ristov, S.";

    let matches = App::new("sfq")
        .version("0.3.0")
        .author("Robert Bakaric <rbakaric@irb.hr>, Dalibor Hrsak <dalibor.hrsak@irb.hr>, Damir Korencic <dkorencic@irb.hr>")
        .about(head)
        //.setting(AppSettings::ArgRequiredElseHelp)
        //.setting(AppSettings::AllowMissingPositional)

        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .required(false)
            .value_name("FILE")
            .help("Input file (fasta,fastq,sfastq)")
            .takes_value(true))

        .arg(Arg::with_name("input-rev")
            .short("j")
            .long("input-rev")
            .required(false)
            .value_name("FILE")
            .help("Filename of a reverse file (fastq, fasta)")
            .takes_value(true))

        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .required(false)
            .value_name("FILE")
            //.default_value("stdout")
            .help("Output file; interleaved if input is two paired end fastq files")
            .takes_value(true))

        .arg(Arg::with_name("action")
            .short("a")
            .long("action")
            .default_value("c")
            .required(false)
            .value_name("c|d|g")
            .help("Action: (c) compress, (d) decompress, (g) grep <requires --list > ")
            .takes_value(true))

        /*.arg(Arg::with_name("cmode")
            .short("s")
            .long("compression-mode")
            .default_value("complete")
            .required(false)
            .value_name("complete|lossy")
            .help("Compression mode")
            .takes_value(true))
*/
        .arg(Arg::with_name("cmode")
            .short("s")
            .long("compression-mode")
            .default_value("0")
            .required(false)
            .value_name("0-4")
            .help("Compression mode. Lossless: 0, lossy: 1 to 4.")
            .takes_value(true))

        .arg(Arg::with_name("outfmt")
            .short("f")
            .long("outfmt")
            .default_value("fq")
            .required(false)
            .value_name("fq|fa|s|q|h|...")
            .help("Output format: \n \n\tfq   \t:fastq, \n\tfa  \t:fasta, \n\ts  \t:sequence, \n\tq  \t:quality, \n\th  \t:head, \n\ts+q  \t:sequence quality, \n\th+q  \t:head quality, \n\th+s  \t:head sequence, \n\th+s+q  \t:head sequence quality, \n\ts+h+q  \t:sequence head quality, \n\t...\n")
            .takes_value(true))

        .arg(Arg::with_name("infmt")
            .short("t")
            .long("infmt")
            .default_value("fastq")
            .required(false)
            .value_name("fastq|fasta")
            .help("File types supported")
            .takes_value(true))
/*
        .arg(Arg::with_name("make-index")
            .short("y")
            .long("make-index")
            .default_value("7")
            .required(true)
            .value_name("hd|>3")
            .help("Make index (4,5,6,... - kmer size, hd - high dimensional kmer index)")
            .takes_value(true))
*/
        .arg(Arg::with_name("mem-mod")
            .short("m")
            .long("memory-mode")
            .default_value("D")
            .required(false)
            .possible_values(&["D","R"])
            .value_name("D|R")
            .help("Memory mode: defines memory type  (D - disc, R - RAM)")
            .takes_value(true))

        .arg(Arg::with_name("fragment-size")
            .short("F")
            .long("fragment-size")
            .default_value("Max")
            .value_name("Max|<integer>")
            .hidden(false)
            .help("Amount of RAM in MB allocated for the compression. Max = use all available RAM.")
            .required(false)
            .takes_value(true))

        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .required(false)
            .default_value("rand(10)")
            .value_name("filename|rand(10)")
            .help("Please provide a list of prefixes (numbers or ranges), in separate lines. SFQ returns records associated with the input prefixes. Works only with -a g.")
            .takes_value(true))

        .arg(Arg::with_name("decompress-exponent")
            .short("e")
            .long("decompress-exponent")
            .required(false)
            .default_value("6")
            .value_name("integer 1 to ~10")
            .hidden(true)
            .help("Exponent of the alphabet length, that determines the size of the batch for simultaneous decompression. Works only with -a d.")
            .takes_value(true))

        .arg(Arg::with_name("restart")
            .short("r")
            .long("restart")
            .required(false)
            .default_value("no")
            .value_name("no|yes")
            .hidden(false)
            .help("Restart compression from temporary files. Works only with -a c. NOTE: Temporary files must be complete and correct!")
            .takes_value(true))

        .get_matches();

    matches
}
