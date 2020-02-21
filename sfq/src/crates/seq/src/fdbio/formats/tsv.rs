



 use crate::{Fdb,Get};
 use crate::util::error::Error;
 use std::io::{self, prelude::*, stdout, Write, Read, BufReader, BufWriter};

 impl Fdb{

     pub fn tsw_dw<W: Write> (&mut self, mut writer:  W)   -> Result<bool,Error>  {

         writer.write_all(&self.get_tsv()).unwrap();

         Ok(true)

     }

 }
