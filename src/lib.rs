use std::path::Path;

use anyhow::Result;
pub use divider::RTFDivider;
pub mod detecter;
pub mod divider;
pub mod rtf;

pub fn list_rtf(dir: &Path) -> Result<String> {
    let rtfs = rtf::list_rtf(dir)?;
    let data = serde_json::to_string(&rtfs)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn list_rtf_test() {
        let data = list_rtf(Path::new(r"D:\网页下载文件\dingtalk\202-113")).unwrap();
        assert!(data.len() > 0)
    }
}
