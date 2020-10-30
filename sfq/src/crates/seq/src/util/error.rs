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

use snafu::{ErrorCompat, ResultExt, Snafu};

use std::{
    path::{Path, PathBuf},
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not read {}: {}", filename.display(), source))]
    ReadErr {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Could not write {}: {}", filename.display(), source))]
    WriteErr {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Vector lengths do not match {} != {}", a, b))]
    LengthErr {
        a: usize,
        b: usize,
    },
    #[snafu(display("Please assert  {} has been set prior to {} call", a, b))]
    SetErr{
        a: String,
        b: String,
    },
    #[snafu(display("Please assert  {} has been set prior to {} call", a, b))]
    SusQueryErr{
        a: String,
        b: String,
    },
    #[snafu(display("Please assert  {} has been set prior to {} call", a, b))]
    SusCompErr{
        a: String,
        b: String,
    },


}
