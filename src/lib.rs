pub mod fs;
mod unix_fs;
mod nop_fs;
mod sys_common;
mod fd;
mod cvt;

#[cfg(test)]
mod tests {
    use crate::fs;

    #[test]
    fn it_works() {
        let f = fs::File::create("/tmp/a").unwrap(); // TODO: need cross-platform fs.rs, https://github.com/rust-lang/rust/blob/master/library/std/src/fs.rs
        println!("f = {:?}", f);
    }
}
