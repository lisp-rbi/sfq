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

use crate::{Save, Fdb};


impl Save for Fdb {

    fn save(&mut self, path: &str, model: &str) -> bool{

        let writer = self.make_writer(path);

        match model {
            "fq" => {

                if let Ok(false) = self.fastq_dw(writer) {
                    panic!("Model {} was not saved !", model);
                };


            },
            "fa" => {

                if let Ok(false) = self.fasta_dw(writer) {
                    panic!("Model {} was not saved !", model);
                };

            },
            _       => {
                if let Ok(false) = self.tsv_dw(writer, model) {
                    panic!("Model {} was not saved !", model);
                };
            }
        }
        true
    }

    fn save_append(&mut self, path: &str,  model: &str) -> bool {

        let writer = self.make_append_writer(path);

        match model {
            "fq" => {

                if let Ok(false) = self.fastq_dw(writer) {
                    panic!("Model {} was not saved !", model);
                };


            },
            "fa" => {

                if let Ok(false) = self.fasta_dw(writer) {
                    panic!("Model {} was not saved !", model);
                };

            },
            _       => {
                if let Ok(false) = self.tsv_dw(writer, model) {
                    panic!("Model {} was not saved !", model);
                };
            }
        }


        true
    }

}
