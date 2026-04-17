use rustix::fs::{CWD, Mode, OFlags, RawDir, openat};

use std::mem::MaybeUninit;
use std::path::PathBuf;

#[allow(unused)]

pub struct Udc {
    dir: PathBuf,
}

pub fn get_udc() -> Option<Vec<String>> {
    let fd = openat(
        CWD,
        c"/sys/class/udc",
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .ok()?;

    let mut buf = [MaybeUninit::uninit(); 4096];
    let mut iter = RawDir::new(fd, &mut buf);
    let mut udcs = vec![];

    while let Some(entry_result) = iter.next() {
        let entry = entry_result.ok()?;

        let file_name = entry.file_name().to_string_lossy();

        if file_name == "." || file_name == ".." {
            continue;
        }

        udcs.push(file_name.into_owned());
    }

    Some(udcs)
}


// fn udc_start() {
//     let udcs = get_udc().unwrap_or_default();
//     println!("Available UDCs: {:?}", udcs);
// }

// fn udc_stop() {
//     println!("Stopping UDC...");
// }
