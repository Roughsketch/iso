use std::io::Read;

use crate::vd::VolumeDescriptor;

#[derive(Clone, Debug)]
pub struct SupplementaryVolume {

}

impl SupplementaryVolume {
    pub fn from_reader<R: Read>(mut reader: R) -> Result<VolumeDescriptor, std::io::Error> {
        Ok(VolumeDescriptor::Supplementary(SupplementaryVolume{}))
    }
}