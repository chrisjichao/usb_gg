use std::thread;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::sync::Mutex;



use crate::functionfs::{ Functionfs};

pub struct MtpFunction {
    function: Functionfs,
    dev_node: String,
    is_running: Arc<AtomicBool>,
job_handle: Mutex<Option<thread::JoinHandle<()>>>,
}

impl MtpFunction {
    pub fn new(dev_node: &str) -> Self {
        MtpFunction {
            function: Functionfs::new("mtp", crate::functionfs::UsbFunction::MTP),
            dev_node: dev_node.to_string(),
            is_running: Arc::new(AtomicBool::new(false)),
            job_handle: Mutex::new(None),
        }
    }

    pub fn start(&mut self) {
        // self.function.start(); // 假设这个需要写权限
        self.is_running.store(true, Ordering::SeqCst);
        self.run();
    }

    fn run(&self) {
        let is_running_clone = Arc::clone(&self.is_running);
        let mut handle_lock = self.job_handle.lock().unwrap();
        
        *handle_lock = Some(thread::spawn(move || {
            println!("MTP 线程已启动...");
            while is_running_clone.load(Ordering::SeqCst) {
                // 实际的 MTP 处理逻辑
                thread::sleep(std::time::Duration::from_millis(500));
            }
            println!("MTP 线程已退出");
        }));
    }

    pub fn stop(&self) {
        // self.function.stop();
        // 1. 发送停止信号
        self.is_running.store(false, Ordering::SeqCst);

        // 2. 取出线程句柄并等待结束
        let handle = self.job_handle.lock().unwrap().take();
        if let Some(h) = handle {
            let _ = h.join();
        }
    }
}

impl Drop for MtpFunction {
    fn drop(&mut self) {
        self.stop();
    }
}