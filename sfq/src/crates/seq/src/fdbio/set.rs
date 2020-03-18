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

use crate::{Set, Fdb};

impl Set for Fdb {

    fn  set_head(&mut self,v: Vec<u8>)-> &mut Self {
        self.head = v;
        self
    }

    fn set_seq(&mut self,v: Vec<u8>)-> &mut Self  {
        self.seq = v;
        self
    }

    fn set_qual(&mut self,v: Vec<u8>)-> &mut Self  {
        self.qual = v;
        self
    }

    fn set_cpcnt(&mut self,v: Vec<usize>)-> &mut Self  {
        self.cpcnt = v;
        self
    }

    fn set_numrec(&mut self,n:usize)-> &mut Self  {
        self.numrec = n;
        self
    }

    fn set_model(&mut self, paired: bool ) -> &mut Self {
        self.paired = paired;
        self
    }


}
