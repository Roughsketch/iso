use std::fs::File;
use std::path::Path;

use crate::vd::VolumeDescriptor;

pub struct Iso {
    vds: Vec<VolumeDescriptor>
}

impl Iso {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        reader.skip(0x8000);
    }
}