
use std::os::unix::fs;

use crate::configfs::Configfs;


pub enum UsbFunction {
    MTP,
    ADB,
}


pub struct Functionfs {
    name: String,
    c_fs: &'static Configfs,
    function_type: UsbFunction,
}

impl Functionfs {
    pub fn new(name: &str, function_type: UsbFunction) -> Self {
            
        let myself = Functionfs {
            name: name.to_string(),
            c_fs: Configfs::build(),
            function_type,
        }
        let ret = myself.mount_function();


        myself
    }


    fn mount_function(&self) -> Result<(), String> {
        match self.function_type {
            UsbFunction::MTP => {
                fs::mkdir_all(format!("{}/usb_gadget/g1/functions/{}", self.c_fs.mount_point(), self.name))
                    .map_err(|e| format!("Failed to create MTP function directory: {}", e))?;

                fs::symlink(original, link);
            }
            UsbFunction::ADB => {
                // ADB function specific initialization
            }
        }
        Ok(())
    }
}

impl Drop for Functionfs {
    fn drop(&mut self) {
        // 在这里执行清理操作，例如卸载函数、删除配置等
        println!("Cleaning up Functionfs: {}", self.name);
    }
}