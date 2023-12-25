use crate::detecter::TypeDetecter;

const REPORT5_HEADER: &[u8] = r#"\rtf1\adeflang1025\ansi\ansicpg1252\uc2\adeff0\deff0\stshfdbch31505\stshfloch31506\stshfhich31506\stshfbi31507\deflang1033\deflangfe2052\themelang1033\themelangfe0\themelangcs0"#.as_bytes();

const REPORT5_HEADER_SIZE: usize = REPORT5_HEADER.len();

pub struct Report5Detecter<'a> {
    data: &'a [u8],
}

impl<'a> TypeDetecter for Report5Detecter<'a> {
    fn type_match(&self) -> bool {
        self.data[1..REPORT5_HEADER_SIZE + 1].eq(REPORT5_HEADER)
    }
}

impl<'a> Report5Detecter<'a> {
    pub fn new(data: &'a [u8]) -> Report5Detecter {
        Report5Detecter { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn report5_detect_test() {
        let file_dir = r"D:\网页下载文件\dingtalk\test-lyh\test\l-16-02-07-11-dd-ss.rtf";
        let f = fs::read(file_dir).unwrap();
        let detecter = Report5Detecter::new(&f[..]);
        let expect = true;
        assert_eq!(expect, detecter.type_match());

        let file_dir = r"D:\网页下载文件\dingtalk\test-lyh\test\L14_3_2_2.rtf";
        let f = fs::read(file_dir).unwrap();
        let detecter = Report5Detecter::new(&f[..]);
        let expect = false;
        assert_eq!(expect, detecter.type_match());
    }
}
