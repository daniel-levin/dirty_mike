use io_uring::{opcode, types, IoUring};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use std::{fs, io};

fn main() -> io::Result<()> {
    let my_bin_path = PathBuf::from("./dustbin");
    if !my_bin_path.exists() {
        std::fs::create_dir(&my_bin_path)?;
    }

    let dustbin = std::fs::File::open(&my_bin_path)?;

    let mut ring = IoUring::new(3)?;

    let bindir = PathBuf::from("/bin");
    let binaries = bindir
        .read_dir()?
        .map(|p| {
            let entry = p.unwrap();
            let origin = entry.path();

            let src = CString::new(origin.to_str().unwrap()).unwrap().into_raw();
            let dest = CString::new(entry.file_name().as_bytes())
                .unwrap()
                .into_raw();
            let op = opcode::SymlinkAt::new(types::Fd(dustbin.as_raw_fd()), src, dest);

            op.build().user_data(1)
        })
        .collect::<Vec<_>>();

    for entry in &binaries {
        unsafe {
            ring.submission()
                .push(entry)
                .expect("submission queue is full");
        }
        ring.submit()?;
    }

    let cqe = ring.completion().next().expect("completion queue is empty");
    dbg!(cqe);

    Ok(())
}
