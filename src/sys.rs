pub mod fs {
    use libc::c_int;

    pub struct FileDesc {
        fd: c_int,
    }
}

