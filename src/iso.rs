use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::vd::VolumeDescriptor;

pub struct Iso {
    pub sys_area: [u8; 0x8000],
    vds: Vec<VolumeDescriptor>
}

impl Default for Iso {
    fn default() -> Iso {
        Self {
            sys_area: [0u8; 0x8000],
            vds: Vec::new(),
        }
    }
}

impl std::fmt::Debug for Iso {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{")?;
        write!(f, "sys_area: {:?}, ", self.sys_area.to_vec())?;
        write!(f, "vds: {:?}, ", self.vds)?;
        write!(f, "}}")
    }
}

impl Iso {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut iso: Iso = Default::default();
        reader.read_exact(&mut iso.sys_area);

        // while let Ok(vd) = VolumeDescriptor::from_reader(&mut reader) {
        //     iso.vds.push(vd);
        // }

        loop {
            match VolumeDescriptor::from_reader(&mut reader) {
                Ok(vd) => iso.vds.push(vd),
                Err(why) => {
                    println!("{:#?}", why);
                    break;
                }
            }
        }
        Ok(iso)
    }
}