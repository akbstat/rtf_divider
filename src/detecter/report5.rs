use crate::detecter::TypeDetecter;

// if field tag does not show IN 2 trowd tag, then deternmine rtf does not generated by report 5
const FILED_TAG: &[u8] = r"\field".as_bytes();
const TROWD_TAG: &[u8] = r"\trowd".as_bytes();

pub struct Report5Detecter<'a> {
    data: &'a [u8],
}

impl<'a> TypeDetecter for Report5Detecter<'a> {
    fn type_match(&self) -> bool {
        let mut trowd_count = 0;
        if self.data.len() < 6 {
            return false;
        }
        let mut tail = 5;
        while trowd_count < 5 && tail < self.data.len() {
            let head_trowd = tail + 1 - TROWD_TAG.len();
            let head_field = tail + 1 - FILED_TAG.len();
            if self.data[head_field..tail + 1].eq(FILED_TAG) {
                return true;
            }
            if self.data[head_trowd..tail + 1].eq(TROWD_TAG) {
                trowd_count += 1;
            }
            tail += 1;
        }
        false
    }
}

impl<'a> Report5Detecter<'a> {
    pub fn new(data: &[u8]) -> Report5Detecter {
        Report5Detecter { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn report5_detect_test() {
        let file_dir = r"D:\网页下载文件\dingtalk\rtfs\202-113\l-16-02-04-06-03-comb-anpr-ss.rtf";
        let f = fs::read(file_dir).unwrap();
        let detecter = Report5Detecter::new(&f[..]);
        let expect = true;
        assert_eq!(expect, detecter.type_match());

        let file_dir = r"D:\网页下载文件\dingtalk\rtfs\112-101\t14_3_1_21_teae_wit_soc_saf_en.rtf";
        let f = fs::read(file_dir).unwrap();
        let detecter = Report5Detecter::new(&f[..]);
        let expect = true;
        assert_eq!(expect, detecter.type_match());

        let file_dir = r"D:\网页下载文件\dingtalk\rtfs\112-101\L14_3_2_2.rtf";
        let f = fs::read(file_dir).unwrap();
        let detecter = Report5Detecter::new(&f[..]);
        let expect = false;
        assert_eq!(expect, detecter.type_match());
    }
}
