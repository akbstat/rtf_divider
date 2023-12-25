#[cfg(test)]
mod test {
    use rtf_divider::RTFDivider;
    use std::{fs::read_dir, path::Path};

    #[test]
    fn divide_do_test() {
        let file_dir = r"D:\网页下载文件\dingtalk\test-lyh\test\l-16-02-07-11-dd-ss.rtf";
        let dest = r"D:\网页下载文件\dingtalk\test-lyh\test\example\result1";
        let divider = RTFDivider::new(Path::new(file_dir)).unwrap().unwrap();
        divider.set_pagesize(5).divide(Path::new(dest)).unwrap();
        let file_list = read_dir(dest).unwrap();
        assert_eq!(file_list.count(), 2);
    }

    #[test]
    fn divide_skip_test() {
        let file_dir = r"D:\网页下载文件\dingtalk\test-lyh\test\L14_3_2_2.rtf";
        let dest = r"D:\网页下载文件\dingtalk\test-lyh\test\example\result2";
        let divider = RTFDivider::new(Path::new(file_dir)).unwrap().unwrap();
        divider.set_pagesize(50).divide(Path::new(dest)).unwrap();
        let file_list = read_dir(dest).unwrap();
        assert_eq!(file_list.count(), 1);
    }
}
