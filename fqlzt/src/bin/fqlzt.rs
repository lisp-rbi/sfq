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

mod cli;

use cli::fqlzt::parse_cli;
use fqlzt::{FFI, Drop, Fdb, IO, GetSet};
use std::time::Instant;


fn main (){


    let cli = parse_cli();

    //println!("{:#?}", cli);

    // read strings from file  into Vec<String> || read index

    match cli.value_of("action").unwrap() {
        "e" => {
            println!("Extracting...");
/*
            let mut strings: Vec<u8> = FFI::open(
                        cli.value_of("input").unwrap(),
                    )
                    .query(

                    )
                    .get();

            let mut fdb = Fdb::new(
                        cli.value_of("ftype").unwrap(),
                    )
                    .make(
                        strings
                    )
                    .save_to(
                        cli.value_of("output").unwrap()
                    );
*/

        },
        "c" => {
            let mut fdb = Fdb::new(
                        cli.value_of("ftype").unwrap()
                    );
            fdb.open(
                        cli.value_of("input").unwrap(),
                    );
            match cli.value_of("format").unwrap() {

                "H(F,R)" | "H(R,F)" => {
                    let before_l = Instant::now();
                    let mut strings: Vec<u8> = fdb.get(
                                "H(F,R)"
                            );
                    println!("Elapsed time: {:.2?}", before_l.elapsed());
                    println!("Compressing sequences...");
                    let out = format!("{}.{}",cli.value_of("output").unwrap(),"seq.lzt");

                    let before = Instant::now();
                    let mut lzt = FFI::new(
                                &out,
                                &mut strings
                            );
                    println!("Elapsed time: {:.2?}", before.elapsed());
                    lzt.drop();
                },
                "H(Fq,Rq)" | "H(Rq,Fq)" => {
                    let before_l = Instant::now();
                    let mut qualities: Vec<u8> = fdb.get(
                                "H(Fq,Rq)"
                            );
                    println!("Elapsed time: {:.2?}", before_l.elapsed());
                    println!("Compressing qualities...");
                    let out = format!("{}.{}",cli.value_of("output").unwrap(),"qual.lzt");
                    let before = Instant::now();
                    let mut lzt = FFI::new(
                                &out,
                                &mut qualities
                            );
                    println!("Elapsed time: {:.2?}", before.elapsed());
                    lzt.drop();

                },
                "H(F,R,Fq,Rq)" | "H(R,F,Rq,Fq)" |
                "H(R,F,Fq,Rq)" | "H(F,R,Rq,Fq)" => {
                    let before_l = Instant::now();
                    let mut strings: Vec<u8> = fdb.get(
                                "H(F,R)"
                            );
                    let mut qualities: Vec<u8> = fdb.get(
                                "H(Fq,Rq)"
                            );
                    println!("Elapsed time: {:.2?}", before_l.elapsed());
                    strings.push('\n' as u8);
                    strings.append(&mut qualities);
                    println!("Compressing ...");
                    let out = format!("{}.{}",cli.value_of("output").unwrap(),"lzt");
                    let before = Instant::now();
                    let mut lzt = FFI::new(
                                &out,
                                &mut strings
                            );
                    println!("Elapsed time: {:.2?}", before.elapsed());
                    lzt.drop();

                },
                _ => {panic!("Format {} not supported!", cli.value_of("format").unwrap())}
            }

        },
        "d" => {
            println!("Decompressing...");
/*
            let mut strings: Vec<u8> = FFI::open(
                        cli.value_of("input").unwrap(),
                    )
                    .query(
                        "*"
                    )
                    .get();

            let mut fdb = Fdb::new(
                        cli.value_of("ftype").unwrap(),
                    )
                    .make(
                        strings
                    )
                    .save(
                        cli.value_of("output").unwrap()
                    );
*/
        },
        _   => {panic!("Unknown action: --action \"{}\" ", cli.value_of("action").unwrap() )}
    }

    // query lzt given:
    //  a) a pattern (prefix)
    //  b) random number of strings
    //  c)
    //  ...

    // destruct the object



/*
    let dict : Vec<String> = vec!["ayxs9".to_string(),"dsdgf".to_string(),"dfnjv".to_string(),"dfnjv".to_string()];

    let mut my_tmp_obj = FFI::new(cli.value_of("output").unwrap().to_string(),dict);
    let pattern : Vec<String> = vec!["ayxs9".to_string(),"dsdgf".to_string()];
    let res = my_tmp_obj.query("ayx".to_string());
    let res_2 = my_tmp_obj.query("d".to_string());
    println!("{:?}  {:?}\nDone ... ok ", res, res_2);
*/
    //my_tmp_obj.drop();


}
