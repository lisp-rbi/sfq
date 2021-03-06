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

use crate::{Push, Fdb};


// FX me this is slow if unziping the entire file   prealocate memory !!!

impl Push for Fdb {

    fn  push_head(&mut self,v: Vec<u8>)-> &mut Self {
        if self.head.len() != 0 {
            self.head.extend(b"\n");
        }
        self.head.extend(v);
        self
    }

    fn push_seq(&mut self,v: Vec<u8>)-> &mut Self  {
        if self.seq.len() != 0 {
            self.seq.extend(b"\n");
        }
        self.seq.extend(v);
        self
    }

    fn push_qual(&mut self,v: Vec<u8>)-> &mut Self  {
        if self.qual.len() != 0 {
            self.qual.extend(b"\n");
        }
        self.qual.extend(v);
        self
    }

    fn push_cpcnt(&mut self,v: Vec<usize>)-> &mut Self  {
        self.cpcnt.extend(v);
        self
    }
}
