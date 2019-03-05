use std::io::{Read, Seek, SeekFrom};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use chrono::prelude::*;

use crate::vd::{read_date, read_str, DirectoryEntry, VolumeDescriptor};

#[derive(Clone)]
pub struct PrimaryVolume {
    sys_id: String,
    vol_id: String,
    vol_space_size: u32,
    vol_set_size: u16,
    vol_seq_num: u16,
    logical_block_size: u16,
    path_table_size: u32,
    le_path_table: u32,
    le_opt_path_table: u32,
    be_path_table: u32,
    be_opt_path_table: u32,
    root_dir: DirectoryEntry,
    vol_set_id: String,
    publisher_id: String,
    data_preparer_id: String,
    app_id: String,
    copyright_id: String,
    abstract_file_id: String,
    bibliographic_file_id: String,
    vol_create_date: DateTime<FixedOffset>,
    vol_modify_date: DateTime<FixedOffset>,
    vol_expiration_date: DateTime<FixedOffset>,
    vol_effective_date: DateTime<FixedOffset>,
    fs_version: u8,
    app_data: [u8; 512],
}

impl Default for PrimaryVolume {
    fn default() -> PrimaryVolume {
        PrimaryVolume {
            sys_id: "".into(),
            vol_id: "".into(),
            vol_space_size: 0,
            vol_set_size: 0,
            vol_seq_num: 0,
            logical_block_size: 0,
            path_table_size: 0,
            le_path_table: 0,
            le_opt_path_table: 0,
            be_path_table: 0,
            be_opt_path_table: 0,
            root_dir: Default::default(),
            vol_set_id: "".into(),
            publisher_id: "".into(),
            data_preparer_id: "".into(),
            app_id: "".into(),
            copyright_id: "".into(),
            abstract_file_id: "".into(),
            bibliographic_file_id: "".into(),
            vol_create_date: DateTime::<FixedOffset>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc.fix()),
            vol_modify_date: DateTime::<FixedOffset>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc.fix()),
            vol_expiration_date: DateTime::<FixedOffset>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc.fix()),
            vol_effective_date: DateTime::<FixedOffset>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc.fix()),
            fs_version: 0,
            app_data: [0u8; 512],
        }
    }
}

impl std::fmt::Debug for PrimaryVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{")?;
        write!(f, "sys_id: {:?}, ", self.sys_id)?;
        write!(f, "vol_id: {:?}, ", self.vol_id)?;
        write!(f, "vol_space_size: {}, ", self.vol_space_size)?;
        write!(f, "vol_set_size: {}, ", self.vol_set_size)?;
        write!(f, "vol_seq_num: {}, ", self.vol_seq_num)?;
        write!(f, "logical_block_size: {}, ", self.logical_block_size)?;
        write!(f, "path_table_size: {}, ", self.path_table_size)?;
        write!(f, "le_path_table: {}, ", self.le_path_table)?;
        write!(f, "le_opt_path_table: {}, ", self.le_opt_path_table)?;
        write!(f, "be_path_table: {}, ", self.be_path_table)?;
        write!(f, "be_opt_path_table: {}, ", self.be_opt_path_table)?;
        write!(f, "root_dir: {:?}, ", self.root_dir)?;
        write!(f, "vol_set_id: {:?}, ", self.vol_set_id)?;
        write!(f, "publisher_id: {:?}, ", self.publisher_id)?;
        write!(f, "data_preparer_id: {:?}, ", self.data_preparer_id)?;
        write!(f, "app_id: {:?}, ", self.app_id)?;
        write!(f, "copyright_id: {:?}, ", self.copyright_id)?;
        write!(f, "abstract_file_id: {:?}, ", self.abstract_file_id)?;
        write!(f, "bibliographic_file_id: {:?}, ", self.bibliographic_file_id)?;
        write!(f, "vol_create_date: {:?}, ", self.vol_create_date)?;
        write!(f, "vol_modify_date: {:?}, ", self.vol_modify_date)?;
        write!(f, "vol_expiration_date: {:?}, ", self.vol_expiration_date)?;
        write!(f, "vol_effective_date: {:?}, ", self.vol_effective_date)?;
        write!(f, "fs_version: {}, ", self.fs_version)?;
        write!(f, "app_data: {:?}", self.app_data.to_vec())?;
        write!(f, "}}")
    }
}

impl PrimaryVolume {
    pub fn from_reader<R: Read + Seek>(mut reader: R) -> Result<VolumeDescriptor, std::io::Error> {
        let mut primary: PrimaryVolume = Default::default();

        //  Unused
        reader.seek(SeekFrom::Current(1))?;

        primary.sys_id = read_str(&mut reader, 32)?; //"".into(),
        primary.vol_id = read_str(&mut reader, 32)?; //"".into(),
        //  Unused Field (All Zeroes)
        reader.seek(SeekFrom::Current(8))?;

        primary.vol_space_size = reader.read_u32::<LittleEndian>()?;
        reader.seek(SeekFrom::Current(4))?;

        //  Unused Field (All Zeroes)
        reader.seek(SeekFrom::Current(32))?;

        primary.vol_set_size = reader.read_u16::<LittleEndian>()?;
        reader.seek(SeekFrom::Current(2))?;

        primary.vol_seq_num = reader.read_u16::<LittleEndian>()?;
        reader.seek(SeekFrom::Current(2))?;

        primary.logical_block_size = reader.read_u16::<LittleEndian>()?;
        reader.seek(SeekFrom::Current(2))?;

        primary.path_table_size = reader.read_u32::<LittleEndian>()?;
        reader.seek(SeekFrom::Current(4))?;

        primary.le_path_table = reader.read_u32::<LittleEndian>()?;
        primary.le_opt_path_table = reader.read_u32::<LittleEndian>()?;
        primary.be_path_table = reader.read_u32::<BigEndian>()?;
        primary.be_opt_path_table = reader.read_u32::<BigEndian>()?;

        //primary.root_dir = ?;   //DirectoryEntry {},
        reader.seek(SeekFrom::Current(34))?;

        primary.vol_set_id = read_str(&mut reader, 128)?;
        primary.publisher_id = read_str(&mut reader, 128)?;
        primary.data_preparer_id = read_str(&mut reader, 128)?;
        primary.app_id = read_str(&mut reader, 128)?;
        primary.copyright_id = read_str(&mut reader, 38)?;
        primary.abstract_file_id = read_str(&mut reader, 36)?;
        primary.bibliographic_file_id = read_str(&mut reader, 37)?;

        primary.vol_create_date = read_date(&mut reader)?;
        primary.vol_modify_date = read_date(&mut reader)?;
        primary.vol_expiration_date = read_date(&mut reader)?;
        primary.vol_effective_date = read_date(&mut reader)?;

        primary.fs_version = reader.read_u8()?;
        reader.seek(SeekFrom::Current(1))?;
        reader.read_exact(&mut primary.app_data)?;
        //  Reserved
        reader.seek(SeekFrom::Current(653))?;

        Ok(VolumeDescriptor::Primary(primary))
    }
}