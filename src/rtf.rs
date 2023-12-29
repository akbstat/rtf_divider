use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::{fs, os::windows::fs::MetadataExt, path::Path};

const RTF_EXTENTION: &str = "rtf";

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
enum Kind {
    Figure,
    Listing,
    Table,
}

impl Kind {
    pub fn new(name: &str) -> Option<Kind> {
        if is_rtf(name) {
            let prefix = name.as_bytes().get(0).unwrap();
            match prefix {
                108u8 => return Some(Kind::Listing),
                116u8 => return Some(Kind::Table),
                102u8 => return Some(Kind::Figure),
                _ => return None,
            }
        }
        None
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rtf {
    name: String,
    size: u64,
    kind: Kind,
}

pub fn list_rtf(p: &Path) -> Result<Vec<Rtf>> {
    let mut rtf = vec![];
    for f in fs::read_dir(p)? {
        let f = f?;
        if f.file_type()?.is_dir() {
            continue;
        }
        let filename = f.file_name().to_string_lossy().to_string();
        let meta = f.metadata()?;
        if let Some(kind) = Kind::new(&filename) {
            rtf.push(Rtf {
                name: filename,
                size: meta.file_size(),
                kind,
            });
        }
    }
    rtf.sort_by(|x, y| x.kind.partial_cmp(&y.kind).unwrap());
    Ok(rtf)
}

/// check if a file is rtf or not, using its filename
fn is_rtf(filename: &str) -> bool {
    let name_len = filename.len();
    // a valid file name must be (l|f|t).*\.rtf
    if name_len < 5 {
        return false;
    }
    if filename[name_len - RTF_EXTENTION.len()..].eq(RTF_EXTENTION) {
        return true;
    }
    false
}

#[cfg(test)]
mod rtf_test {
    use super::*;
    #[test]
    fn list_rtf_test() {
        let dir = Path::new(r"D:\网页下载文件\dingtalk\202-113");
        let rtfs = list_rtf(dir).unwrap();
        assert_eq!(rtfs.len(), 142)
    }

    #[test]
    fn is_rtf_test() {
        assert!(is_rtf("abc.rtf"));
        assert!(!is_rtf("abc.txt"));
        assert!(!is_rtf("abc"));
    }
}
