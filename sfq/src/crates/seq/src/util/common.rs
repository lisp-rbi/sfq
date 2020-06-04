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

use crate::Fdb;


impl Fdb{

    pub fn clear(&mut self) -> &mut Self {
        self.head.resize(0,0x00);
        self.seq.resize(0,0x00);
        self.qual.resize(0,0x00);
        self.numrec = 0;

        self
    }

    pub fn clear_head(&mut self) -> &mut Self {
        self.head.resize(0,0x00);
        self
    }
    pub fn clear_seq(&mut self) -> &mut Self {
        self.seq.resize(0,0x00);
        self
    }
    pub fn clear_qual(&mut self) -> &mut Self {
        self.qual.resize(0,0x00);
        self
    }

    pub fn revcomp(&self, s: String) -> String {

        s.chars()
        .map(|x| match x {
            'a' | 'A' => 'T',
            't' | 'T' => 'A',
            'g' | 'G' => 'C',
            'c' | 'C' => 'G',
            _         => x
        }).rev().collect()

    }

    pub fn  encode (
        self,
        a: usize,
        b: usize,
        c: &Vec<u8>) -> Vec<u8>{

        let (mut v, mut res) = (vec![0u8;b],a);

        for i in 0..b {
            let rmd = res%4;
            res = res/4;
            v[b-(i+1)] = c[rmd];
        }
        v
    }

    pub(crate) fn compare_vslice(&self, va: &[u8], vb: &[u8]) -> bool {

        (va.len() == vb.len()) && va.iter()
            .zip(vb)
            .all(|(a,b)| *a == *b)

    }

}
