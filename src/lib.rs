mod unix_fs;
mod sys_common;
mod fd;
mod cvt;

#[cfg(test)]
mod tests {
    use crate::fs;

    #[test]
    fn it_works() {
        //let f = fs::File::create("/tmp/a").unwrap(); // TODO: need cross-platform fs.rs, https://github.com/rust-lang/rust/blob/master/library/std/src/fs.rs
        // TODO: the trait `From<&str>` is not implemented for `&Path`
        let f = fs::File::open("/etc/motd", &fs::OpenOptions::new()).unwrap();
        println!("f = {:?}", f);
    }
}
