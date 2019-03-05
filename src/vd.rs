use std::io::{Read, Seek};

pub mod boot;
pub mod primary;
pub mod supplementary;

use crate::vd::{boot::BootRecord, primary::PrimaryVolume, supplementary::SupplementaryVolume};

#[derive(Clone, Debug)]
pub enum VolumeDescriptor {
    Boot(BootRecord),
    Primary(PrimaryVolume),
    Supplementary(SupplementaryVolume),
    Partition,
    Terminator,
}

impl VolumeDescriptor {
    pub fn from_reader<R: Read + Seek>(mut reader: R) -> Result<VolumeDescriptor, std::io::Error> {
        let mut header = [0u8; 7];

        reader.read_exact(&mut header)?;

        if &header[1..6] != b"CD001" || header[6] != 1 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid header constant").into());
        }

        match header[0] {
            0 => Ok(BootRecord::from_reader(&mut reader)?),
            1 => Ok(PrimaryVolume::from_reader(&mut reader)?),
            2 => Ok(SupplementaryVolume::from_reader(&mut reader)?),
            3 => Ok(VolumeDescriptor::Partition),
            255 => Ok(VolumeDescriptor::Terminator),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid volume descriptor").into()),
        }
    }
}

#[derive(Clone, Debug)]
struct DirectoryEntry {}

pub fn read_str<R: Read>(mut reader: R, len: usize) -> Result<String, std::io::Error> {
    let mut buf = vec![0; len];
    reader.read_exact(&mut buf)?;

    match std::str::from_utf8(&buf) {
        Ok(s) => {
            println!("Read string: {}", s.to_owned());
            Ok(s.trim().into())
        },
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid boot record system identifier").into()),
    }
}