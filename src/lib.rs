pub mod fs;
mod nop_fs;
mod static_fs;
mod sys_common;

#[cfg(not(target_arch = "wasm32"))]
mod fd;
#[cfg(not(target_arch = "wasm32"))]
mod cvt;
#[cfg(not(target_arch = "wasm32"))]
mod mem_fs;
#[cfg(not(target_arch = "wasm32"))]
mod unix_fs;

#[cfg(test)]
mod tests {
    use crate::fs;

    #[test]
    fn it_works() {
        let f = fs::File::create("/tmp/a").unwrap(); // TODO: need cross-platform fs.rs, https://github.com/rust-lang/rust/blob/master/library/std/src/fs.rs
        println!("f = {:?}", f);
    }
}
