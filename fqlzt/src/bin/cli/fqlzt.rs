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
 ________ ________  ___       ________  _________    \t
|\\  _____\\\\   __  \\|\\  \\     |\\_____  \\|\\___   ___\\    \t
\\ \\  \\__/\\ \\  \\|\\  \\ \\  \\     \\|___/  /\\|___ \\  \\_|    \t
 \\ \\   __\\\\ \\  \\\\\\  \\ \\  \\        /  / /    \\ \\  \\     \t
  \\ \\  \\_| \\ \\  \\\\\\  \\ \\  \\____  /  /_/__    \\ \\  \\    \t
   \\ \\__\\   \\ \\_____  \\ \\_______\\\\________\\   \\ \\__\\   \t
    \\|__|    \\|___| \\__\\|_______|\\|_______|    \\|__|   \t
                   \\|__|                               \t
                                                      \t
            Auth: Bakaric R. Korencic, D. & Ristov, S.";

    let  matches = App::new("fqlzt")
          .version("0.01")
          .author("Robert Bakaric <rbakaric@irb.hr>, Damir Korencic<dkorencic@irb.hr>")
          .about(head)

          .arg(Arg::with_name("input")
               .short("i")
               .long("input")
               .required(false)
               .default_value("stdin")
               .value_name("FILE")
               .help("Input file [fasta,fastq,lzt]")
               .takes_value(true))

          .arg(Arg::with_name("output")
               .short("o")
               .long("output")
               .required(false)
               .value_name("FILE")
               .default_value("dict.lzt")
               .help("Output file")
               .takes_value(true))

          .arg(Arg::with_name("action")
               .short("a")
               .long("action")
               .default_value("c")
               .required(true)
               .value_name("c|d|e")
               .help("Action: (c) compress, (d) decompress, (e) extract <requires --list > ")
               .takes_value(true))

          .arg(Arg::with_name("ftype")
               .short("t")
               .long("ftype")
               .default_value("fastq")
               .required(true)
               .value_name("fastq|fasta|raw")
               .help("File types supported")
               .takes_value(true))

          .arg(Arg::with_name("format")
               .short("f")
               .long("format")
               .default_value("H(F,R,Fq,Rq)")
               .required(true)
               .value_name("H+F+R+Fq+Rq|...")
               .help("Types of reformating supported")
               .takes_value(true))

          .arg(Arg::with_name("mem-mod")
               .short("m")
               .long("memory-mode")
               .default_value("0")
               .required(true)
               .value_name("0|1|2|..|5")
               .help("Moemory mode: defines memory sparsity level [0- low,5- high]")
               .takes_value(true))

          .arg(Arg::with_name("max-mem")
               .short("M")
               .long("max-memory-used")
               .default_value("3600")
               .required(true)
               .value_name("3600")
               .help("Max Memory to be used [in MB]")
               .takes_value(true))

          .arg(Arg::with_name("list")
               .short("l")
               .long("list")
               .required(true)
               .default_value("rand(10)")
               .value_name("file.csv|rand(10)")
               .help("Please provide a list of prefixes, records of which are to be extracted (works only with -a e)")
               .takes_value(true))

          .arg(Arg::with_name("direction")
               .short("d")
               .long("direction")
               .required(true)
               .default_value("bi")
               .value_name("bi|fwd|rew")
               .help("Condition to be met when extraction (--action e ) is invoked!")
               .takes_value(true))

        .get_matches();

    matches
}
