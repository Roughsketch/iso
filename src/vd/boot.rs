use std::io::Read;

use crate::vd::{read_str, VolumeDescriptor};

#[derive(Clone)]
pub struct BootRecord {
    sys_ident: String,
    ident: String,
    data: [u8; 1977],
}

impl Default for BootRecord {
    fn default() -> BootRecord {
        BootRecord {
            sys_ident: "".into(),
            ident: "".into(),
            data: [0u8; 1977],
        }
    }
}

impl std::fmt::Debug for BootRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{")?;
        write!(f, "sys_ident: {}, ", self.sys_ident)?;
        write!(f, "ident: {}, ", self.ident)?;
        write!(f, "data: {:?}", self.data.to_vec())?;
        write!(f, "}}")
    }
}

impl BootRecord {
    pub fn from_reader<R: Read>(mut reader: R) -> Result<VolumeDescriptor, std::io::Error> {
        let mut boot_rec: BootRecord = Default::default();

        boot_rec.sys_ident = read_str(&mut reader, 32)?;
        boot_rec.ident = read_str(&mut reader, 32)?;
        reader.read_exact(&mut boot_rec.data)?;

        Ok(VolumeDescriptor::Boot(boot_rec))
    }
}