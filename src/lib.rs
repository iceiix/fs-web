pub mod fs;
mod sys_common;
mod fd;
mod cvt;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
