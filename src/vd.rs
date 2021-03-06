use std::io::{Read, Seek};
use byteorder::ReadBytesExt;
use chrono::prelude::*;

mod boot;
mod primary;
mod supplementary;

pub use crate::vd::{
    boot::BootRecord,
    primary::PrimaryVolume,
    supplementary::SupplementaryVolume
};

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
struct DirectoryEntry {
    len: u8,
    ext_attrib_len: u8,
    location: u32,
    data_len: u32,
    rec_date: DateTime<FixedOffset>,
    file_flags: u8,
    file_unit_size: u8,
    interleave_gap_size: u8,
    vol_seq_num: u16,
    file_id_len: u8,
}

impl Default for DirectoryEntry {
    fn default() -> DirectoryEntry {
        DirectoryEntry {
            len: 0,
            ext_attrib_len: 0,
            location: 0,
            data_len: 0,
            rec_date: DateTime::<FixedOffset>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc.fix()),
            file_flags: 0,
            file_unit_size: 0,
            interleave_gap_size: 0,
            vol_seq_num: 0,
            file_id_len: 0,
        }
    }
}

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

pub fn read_date<R: Read>(mut reader: R) -> Result<DateTime<FixedOffset>, std::io::Error> {
    let mut data = [0; 7];
    reader.read_exact(&mut data)?;

    if  data[1] < 1 || data[1] > 12 ||  //  Month   (1 - 12)
        data[2] < 1 || data[2] > 31 ||  //  Day     (1 - 31)
        data[3] > 23 ||                 //  Hour    (0 - 23)
        data[4] > 59 ||                 //  Minute  (0 - 59)
        data[5] > 59                    //  Second  (0 - 59)
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid date {:?}", data)).into());
    }

    let date = format!("{}{}{}{}{}{}{}",
        data[0] as u16 + 1900,
        data[1],
        data[2],
        data[3],
        data[4],
        data[5],
        gmt_to_utc_str(data[6] as i8)?
    );

    match DateTime::parse_from_str(&date, "%Y%m%d%H%M%S%z") {
        Ok(date) => Ok(date),
        Err(why) => Err(
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("{:?}", why)).into()
            ),
    }
}

pub fn read_date_str<R: Read>(mut reader: R) -> Result<DateTime<FixedOffset>, std::io::Error> {
    let mut date = read_str(&mut reader, 16)?;
    let gmt = reader.read_i8()?;

    //  If date is all zero, then return the epoch
    if date == "0000000000000000" {
        return Ok(
            DateTime::<FixedOffset>::from_utc(
                NaiveDateTime::from_timestamp(0, 0),
                Utc.fix())
            );
    }

    //  ISO 9660 gives us 2 digits for fractional seconds,
    //  but chrono expects a minimum of 3 digits.
    date.push_str("0");
    date.push_str(&gmt_to_utc_str(gmt)?);

    match DateTime::parse_from_str(&date, "%Y%m%d%H%M%S%3f%z") {
        Ok(date) => Ok(date),
        Err(why) => Err(
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("{:?}", why)).into()
            ),
    }
}

fn gmt_to_utc_str(mut gmt: i8) -> Result<String, std::io::Error> {
    //  GMT byte ranges from 0 - 100, but is mapped to -48 to 52
    if gmt < 0 || gmt > 100 {
        //  If not in expected range, return an error
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid GMT offset: {}", gmt)).into());
    } else {
        gmt -= 48;
    }

    //  Convert the given -48 to +52 to UTC format for chrono
    let utc: i16 = (gmt as i16 / 4) * 100 + (gmt as i16 % 4) * 15;

    if utc > 0 {
        Ok(format!("+{:04}", utc))
    } else {
        Ok(format!("-{:04}", utc.abs()))
    }
}