use std::path::PathBuf;

use serde::Serialize;

use crate::script;

script!(CreatePartitionsScript {
    file = "create-partitions.nu"
    args = PartitionsConfig
});

#[derive(Clone, Debug, Serialize)]
pub struct PartitionsConfig {
    pub device: PathBuf,
    pub efi_partition: bool,
    pub partitions: Partitions,
}

#[derive(Clone, Debug, Serialize)]
pub enum Partitions {
    Auto,
    Manual(Vec<Partition>),
}

#[derive(Clone, Debug, Serialize)]
pub struct Partition {
    pub mountpoint: PathBuf,
    pub blockdevice: PathBuf,
    pub filesystem: Option<FileSystem>,
}

#[derive(Clone, Debug, Serialize)]
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
