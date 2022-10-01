use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::script;

script!(CreatePartitionsScript {
    file = "create-partitions"
    args = PartitionsConfig
});

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartitionsConfig {
    pub device: PathBuf,
    pub efi_partition: bool,
    pub partitions: Partitions,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Partitions {
    Auto,
    Manual(Vec<Partition>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Partition {
    pub mountpoint: PathBuf,
    pub blockdevice: PathBuf,
    pub filesystem: Option<FileSystem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
