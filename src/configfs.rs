use std::sync::OnceLock;
use std::{fs};

use crate::usb_gadget_error::UGError;

use rustix::mount::{ MountFlags, mount};
use std::path::Path;

#[allow(unused)]
#[allow(non_snake_case)] // 告诉编译器：别管我的命名
pub struct Configfs {
    bcdDevice: u16,
    bcdUSB: u16,
    bDeviceClass: u8,
    bDeviceProtocol: u8,
    bDeviceSubClass: u8,
    bMaxPacketSize0: u8,
    idProduct: u16,
    idVendor: u16,
    max_speed: String,
}

impl Default for Configfs {
    fn default() -> Self {
        Self {
            bcdDevice: 0x0100,
            bcdUSB: 0x0200,
            bDeviceClass: 0x00,
            bDeviceProtocol: 0x00,
            bDeviceSubClass: 0x00,
            bMaxPacketSize0: 64,
            idProduct: 0x0001,
            idVendor: 0x1d6b, // Linux Foundation
            max_speed: "high".to_string(),
        }
    }
}

impl Configfs {
    fn global() -> &'static Self {
        static INSTANCE: OnceLock<Configfs> = OnceLock::new();
        INSTANCE.get_or_init(|| Configfs { ..Self::default() })
    }

    pub fn build() -> &'static Self {
        let myself = Self::global();

        if !myself.is_configfs_supported() {
            panic!("Configfs is not supported on this system.");
        }



        match myself.check_configfs_mounted_path() {
            Some(path) => {
                println!("Configfs is already mounted at: {}", path);
                myself
            }
            None => {
                myself.mount_configfs_at("/root/cfg").expect("Failed to mount configfs");
                myself

            }
        }
    }

    fn is_configfs_supported(&self) -> bool {
        fs::read_to_string("/proc/filesystems")
            .map(|s| s.contains("configfs"))
            .unwrap_or(false)
    }

    fn check_configfs_mounted_path(&self) -> Option<String> {
        let content = fs::read_to_string("/proc/self/mounts").ok()?;

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[2] == "configfs" {
                return Some(parts[1].to_string());
            }
        }
        None
    }

    pub fn mount_configfs_at(&self, target: &str) -> Result<(), UGError> {
        let mount_point = if target.is_empty() {
            "/sys/kernel/config"
        } else {
            target
        };

        println!("Mounting configfs at {}", mount_point);

        let path = Path::new(mount_point);
        if !path.exists() {
            fs::create_dir_all(path).map_err(|e| {
                println!("Failed to create mount point directory: {}", e);
                UGError::ConfigfsMountFailed
            })?;
        }

        mount(
            "none",
            mount_point,
            "configfs",
            MountFlags::empty(), // flags
            "",                  // data
        )
        .map_err(|e| {
            println!("Failed to mount configfs at {}: {}", mount_point, e);
            UGError::ConfigfsMountFailed
        })?;

        println!("Successfully mounted configfs at {}", mount_point);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configfs_new() {
        assert!(false);
    }
}
