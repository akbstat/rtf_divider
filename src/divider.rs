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
