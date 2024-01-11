// use crate::detecter::{Report5Detecter, TypeDetecter};
use anyhow::{Ok, Result};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use super::RTF_EXTENTION;
const PAGE_HEAD: &[u8] =
    r"\pard\sect\sectd\linex0\endnhere\pgwsxn16837\pghsxn11905\lndscpsxn".as_bytes();
const FIRST_PAGE_HEAD: &[u8] =
    r"\sectd\linex0\endnhere\pgwsxn16837\pghsxn11905\lndscpsxn\pgnrestart\pgnstarts1".as_bytes();

pub struct ReportDivider<'a> {
    filename: &'a str,
    pagesize: usize,
    bytes: &'a [u8],
    header_pos: (usize, usize),
    page_pos_list: Vec<(usize, usize)>,
}

impl<'a> ReportDivider<'a> {
    pub fn new(
        filename: &'a str,
        bytes: &'a [u8],
        pagesize: usize,
    ) -> Result<Option<ReportDivider<'a>>> {
        let header_pos = (0usize, 0usize);
        let page_pos_list = vec![];
        Ok(Some(ReportDivider {
            filename,
            pagesize,
            bytes,
            header_pos,
            page_pos_list,
        }))
    }

    pub fn divide(&mut self, dest: &Path) -> Result<()> {
        if !dest.exists() {
            fs::create_dir_all(dest)?;
        }

        let mut file_part_index = 1;

        self.find_header_pos();
        self.divide_pages();

        let mut i = 0;
        while i < self.page_pos_list.len() {
            self.write(
                (i, i + self.pagesize),
                self.out_file_path(dest, self.filename, file_part_index)
                    .as_path(),
            )?;
            file_part_index += 1;
            i += self.pagesize;
        }
        if i < self.page_pos_list.len() {
            self.write(
                (i, i + self.pagesize),
                self.out_file_path(dest, self.filename, file_part_index)
                    .as_path(),
            )?;
        }
        Ok(())
    }

    /// get the position of rtf header, store the result in header_pos
    fn find_header_pos(&mut self) {
        let sectd_value = r"\sectd".as_bytes();
        for (head, _) in self.bytes.iter().enumerate() {
            let tail = head + sectd_value.len() - 1;
            if !(tail < self.bytes.len()) {
                break;
            };
            if sectd_value.eq(&self.bytes[head..tail + 1]) {
                self.header_pos.1 = head;
                break;
            }
        }
    }
    /// get the position of each pages, store the result in page_pos_list
    fn divide_pages(&mut self) {
        let headery = r"\headery".as_bytes();
        let headery_len = headery.len();
        let sectd = r"\sect\sectd".as_bytes();
        let sectd_len = sectd.len();
        let mut page_pos = (self.header_pos.1, 0usize);
        let mut head = self.header_pos.1;

        while head + sectd_len < self.bytes.len() {
            // find the position of "\headery"
            while head + headery_len < self.bytes.len()
                && self.bytes[head..head + headery_len].ne(headery)
            {
                head += 1;
            }
            page_pos.0 = head;

            // find the position of next "\pard\sect\sectd"
            while head + sectd_len < self.bytes.len()
                && self.bytes[head..head + sectd_len].ne(sectd)
            {
                head += 1;
            }
            page_pos.1 = head;
            self.page_pos_list.push(page_pos.clone());
            head += 1;
        }
        if head < self.bytes.len() - 1 {
            if let Some(tail) = self.page_pos_list.pop() {
                self.page_pos_list.push((tail.0, self.bytes.len() - 1))
            }
        }
        // let p = self.page_pos_list[3];
        // println!("{:?}", String::from_utf8(self.bytes[p.0..p.1].to_owned()))
    }

    fn write(&self, range: (usize, usize), dest: &Path) -> Result<()> {
        let mut f = fs::OpenOptions::new().create(true).write(true).open(dest)?;
        f.write(&self.bytes[self.header_pos.0..self.header_pos.1])?;
        f.write(FIRST_PAGE_HEAD)?;
        let first = range.0;
        let last = if range.1 < self.page_pos_list.len() {
            range.1
        } else {
            self.page_pos_list.len()
        };

        for i in first..last {
            let page = self.page_pos_list.get(i).unwrap();
            if i.ne(&first) {
                f.write(PAGE_HEAD)?;
            }

            f.write(&self.bytes[page.0..page.1])?;
        }
        f.write(&[125u8])?;
        f.flush().unwrap();
        Ok(())
    }
    fn out_file_path(&self, dest: &Path, filename: &str, index: usize) -> PathBuf {
        PathBuf::from(dest).join(format!("{}_part_{:0>4}{}", filename, index, RTF_EXTENTION))
    }
}
