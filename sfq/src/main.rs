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
