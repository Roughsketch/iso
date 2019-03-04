pub enum VolumeDescriptor {
    Boot(BootRecord),
    Primary(PrimaryVolume),
    Supplementary(SupplementaryVolume),
    Partition,
    Terminator,
}

struct DirectoryEntry {}

pub struct BootRecord {
    sys_ident: String,
    ident: String,
    data: [u8; 1977],
}

pub struct PrimaryVolume {
    sys_id: String,
    vol_id: String,
    vol_space_size: u32,
    vol_set_size: u32,
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
    vol_create_date: String,
    vol_modify_date: String,
    vol_expiration_date: String,
    vol_effective_date: String,
    fs_version: u8,
    app_data: [u8; 512],
}

pub struct SupplementaryVolume {

}