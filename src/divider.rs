use anyhow::{anyhow, Ok, Result};
use std::{fs, path::Path};

use crate::detecter::{Report5Detecter, TypeDetecter};

use self::{report::ReportDivider, report5::Report5Divider};

pub mod report;
pub mod report5;

const DEFAULT_PAGE_SIZE: usize = 10;
const RTF_EXTENTION: &str = ".rtf";

pub trait KindDivider {
    // fn set_pagesize(self, pagesize: usize) -> Self;
    fn divide(&mut self, dest: &Path) -> Result<()>;
}

pub struct RTFDivider {
    filename: String,
    pagesize: usize,
    bytes: Vec<u8>,
}

impl RTFDivider {
    pub fn new(target: &Path) -> Result<Option<RTFDivider>> {
        if !target.exists() || target.is_dir() {
            return Err(anyhow!("Error: file path {:?} is invalid", target));
        }
        let bytes = fs::read(target)?;
        Ok(Some(RTFDivider {
            filename: target.file_stem().unwrap().to_string_lossy().to_string(),
            pagesize: DEFAULT_PAGE_SIZE,
            bytes,
        }))
    }
    pub fn set_pagesize(mut self, pagesize: usize) -> Self {
        self.pagesize = pagesize;
        self
    }
    pub fn divide(&mut self, dest: &Path) -> Result<()> {
        if Report5Detecter::new(&self.bytes).type_match() {
            let mut divider =
                Report5Divider::new(&self.filename, &self.bytes, self.pagesize)?.unwrap();
            divider.divide(dest)?;
            return Ok(());
        }
        let mut divider = ReportDivider::new(&self.filename, &self.bytes, self.pagesize)?.unwrap();
        divider.divide(dest)?;
        Ok(())
    }
}

#[cfg(test)]
mod divider_test {
    use super::*;
    #[test]
    fn test_divider() {
        // let pdf =
        //     Path::new(r"D:\Studies\ak112\303\stats\CSR\product\output\l-16-02-08-06-ecog-ss.rtf");
        // let dest = Path::new(r"D:\Studies\ak112\303\stats\CSR\product\output\rtf-divided");
        // let divider = RTFDivider::new(pdf).unwrap().unwrap();
        // divider.set_pagesize(50).divide(dest).unwrap();

        let pdf = Path::new(
            r"D:\Studies\ak112\303\stats\CSR\product\output\asco\t-14-01-03-01-dm-fas.rtf",
        );
        let dest = Path::new(r"D:\Studies\ak112\303\stats\CSR\product\output\asco\rtf_divided");
        let divider = RTFDivider::new(pdf).unwrap().unwrap();
        divider.set_pagesize(50).divide(dest).unwrap();
    }
}
