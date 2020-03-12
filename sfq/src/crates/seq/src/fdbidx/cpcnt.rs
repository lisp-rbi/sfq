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

use crate::{Fdb,Counter,Get};
use crate::util::error::Error;
use fxhash::FxHashMap;

// utility of this function is to sort sequences by their dna strings and and compute
// hom many copies of a same string exists  -> this information is returned to the main program
// and once encountered -> the first is written down and the number of copies is added using the
// same codec while oll the rest are skippedd  // qualities are not touched
//
// this makes no sence :

impl Counter for Fdb {

    fn cpcnt (&self) -> bool { //FxHashMap<String, (usize,usize)> {

        let mut pairs : FxHashMap<String,String> = FxHashMap::default();
        let mut tab : FxHashMap<usize,usize> = FxHashMap::default();

        {
            let head = self.get_head();
            let head2d: Vec<_> = head.split(|i| *i == 10u8).collect();
            for i in (0..head2d.len()).step_by(2) {
                let  a : String = String::from_utf8(head2d[i].to_vec()).unwrap();
                let  b : String = String::from_utf8(head2d[i+1].to_vec()).unwrap();;
                pairs.insert(a,b);

            }
        }
        {
            let sq = self.get_tsv("s+q");
            let h = self.get_head();
            let h2d : Vec<_> = h.split(|i| *i == 10u8).collect();
            let sq2d: Vec<_> = sq.split(|i| *i == 10u8).collect();
            let mut cp=1;

            for i in 1..sq2d.len() {
                if sq2d[i] == sq2d[i-1] {
                    cp+=1;
                }else{
                    tab.insert(String::from_utf8(h2d[i].to_vec()).unwrap(),cp);
                    cp=1;
                }
                println!("{}:{:?}-{:?}\n{:?}-{:?}\n{:?}-{:?}\n{:?}-{:?}", cp,
                String::from_utf8(h2d[i-3].to_vec()).unwrap(),String::from_utf8(sq2d[i-1].to_vec()).unwrap(),
                String::from_utf8(h2d[i-2].to_vec()).unwrap(),String::from_utf8(sq2d[i-2].to_vec()).unwrap(),
                String::from_utf8(h2d[i-1].to_vec()).unwrap(), String::from_utf8(sq2d[i-3].to_vec()).unwrap(),
                String::from_utf8(h2d[i].to_vec()).unwrap(),String::from_utf8(sq2d[i].to_vec()).unwrap() );
            }
        }
        println!("{:#?}", pairs);
        println!("{:#?}", tab);

        true

    }

}
