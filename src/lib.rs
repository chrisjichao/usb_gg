mod configfs;
mod functionfs;
mod usb_gadget_error;


mod udc; 
mod mtp;
use mtp::MtpFunction;



pub fn run() {
    let mut mtp =MtpFunction::new("/dev/mtp_usb");
    mtp.start();
    println!("MTP function started. Press Enter to stop...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    // mtp.stop(); // 这里不需要显式调用 stop，因为 MtpFunction 实现了 Drop trait，会在作用域结束时自动调用 stop
}