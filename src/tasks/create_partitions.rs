use std::path::PathBuf;

use embed_nu::rusty_value::RustyValue;
use serde::Deserialize;

use crate::script;

script!(CreatePartitionsScript {
    file = "create-partitions"
    args = PartitionsConfig
});

#[derive(Clone, Debug, Deserialize, RustyValue)]
pub struct PartitionsConfig {
    pub device: PathBuf,
    pub efi_partition: bool,
    pub partitions: Partitions,
}

#[derive(Clone, Debug, Deserialize, RustyValue)]
pub enum Partitions {
    Auto,
    Manual(Vec<Partition>),
}

#[derive(Clone, Debug, Deserialize, RustyValue)]
pub struct Partition {
    pub mountpoint: PathBuf,
    pub blockdevice: PathBuf,
    pub filesystem: Option<FileSystem>,
}

#[derive(Clone, Debug, Deserialize, RustyValue)]
pub enum FileSystem {
    VFAT,
    BFS,
    CramFS,
    Ext2,
    Ext3,
    Ext4,
    FAT,
    MSDOS,
    XFS,
    BTRFS,
    Minix,
    F2FS,
}
