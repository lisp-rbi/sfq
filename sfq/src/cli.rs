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
            Auth: Bakaric R. Korencic, D. & Ristov, S.";

    let  matches = App::new("sfq")
          .version("0.2.2")
          .author("Robert Bakaric <rbakaric@irb.hr>, Damir Korencic<dkorencic@irb.hr>")
          .about(head)

          .arg(Arg::with_name("input")
               .short("i")
               .long("input")
               .required(false)
               .default_value("stdin")
               .value_name("FILE")
               .help("Input file (fasta,fastq,lzt)")
               .takes_value(true))

          .arg(Arg::with_name("input-rev")
               .short("j")
               .long("input-rev")
               .required(false)
               .value_name("FILE")
               .help("Input file of a revers file (fastq)")
               .takes_value(true))

          .arg(Arg::with_name("output")
               .short("o")
               .long("output")
               .required(false)
               .value_name("FILE")
               .default_value("stdout")
               .help("Output file: interleved if paired fastq, dict.lzt if compressed")
               .takes_value(true))

          .arg(Arg::with_name("output-dir")
               .short("d")
               .long("output-dir")
               .required(false)
               .value_name("DIRECTORY")
               .help("Output directory")
               .takes_value(true))

          .arg(Arg::with_name("action")
               .short("a")
               .long("action")
               .default_value("c")
               .required(true)
               .value_name("c|d|q")
               .help("Action: (c) compress, (d) decompress, (g) get <requires --list > ")
               .takes_value(true))

           .arg(Arg::with_name("cmode")
               .short("s")
               .long("compression-mode")
               .default_value("complete")
               .required(true)
               .value_name("complete|lossy")
               .help("Compression mode")
               .takes_value(true))

           .arg(Arg::with_name("outfmt")
               .short("f")
               .long("outfmt")
               .default_value("fq")
               .required(true)
               .value_name("fq|fa|s|q|h|...")
               .help("Output format: \n \n\tfq   \t:fastq, \n\tfa  \t:fasta, \n\ts  \t:sequence, \n\tq  \t:quality, \n\th  \t:head, \n\ts+q  \t:sequence quality, \n\th+q  \t:head quality, \n\th+s  \t:head sequence, \n\th+s+q  \t:head sequence quality, \n\ts+h+q  \t:sequence head quality, \n\t...\n")
               .takes_value(true))

          .arg(Arg::with_name("infmt")
                .short("t")
                .long("infmt")
                .default_value("fastq")
                .required(true)
                .value_name("fastq|fasta|raw")
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
               .required(true)
               .value_name("D|R")
               .help("Memory mode: defines memory type  (D - disc, R - RAM)")
               .takes_value(true))

          .arg(Arg::with_name("fragment-size")
               .short("F")
               .long("fragment-size")
               .default_value("Max")
               .required(true)
               .value_name("Max|3600,5000")
               .help("Number of lines to be processed at a time (Max - use all available)")
               .takes_value(true))

          .arg(Arg::with_name("list")
               .short("l")
               .long("list")
               .required(true)
               .default_value("rand(10)")
               .value_name("file.csv|rand(10)")
               .help("Please provide a list of prefixes, records of which are to be extracted (works only with -a g)")
               .takes_value(true))

        .get_matches();

    matches
}
