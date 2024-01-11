#[cfg(test)]
mod test {
    use rtf_divider::RTFDivider;
    use std::{fs::read_dir, path::Path};

    #[test]
    fn divide_do_test() {
        let file_dir = r"D:\网页下载文件\dingtalk\rtfs\202-113\l-16-02-08-05-ecg-ss.rtf";
        let dest = r"D:\网页下载文件\dingtalk\rtfs\202-113\result";
        let divider = RTFDivider::new(Path::new(file_dir)).unwrap().unwrap();
        divider.set_pagesize(50).divide(Path::new(dest)).unwrap();
        let file_list = read_dir(dest).unwrap();
        assert_eq!(file_list.count(), 2);
    }

    #[test]
    fn divide_skip_test() {
        let file_dir = r"D:\网页下载文件\dingtalk\rtfs\112-101\T14_1_1.rtf";
        let dest = r"D:\网页下载文件\dingtalk\rtfs\112-101\result";
        let divider = RTFDivider::new(Path::new(file_dir)).unwrap().unwrap();
        divider.set_pagesize(2).divide(Path::new(dest)).unwrap();
        let file_list = read_dir(dest).unwrap();
        assert_eq!(file_list.count(), 2);
    }
}
