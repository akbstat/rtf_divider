// use crate::detecter::{Report5Detecter, TypeDetecter};
use anyhow::{Ok, Result};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use super::RTF_EXTENTION;

pub struct Report5Divider<'a> {
    filename: &'a str,
    pagesize: usize,
    bytes: &'a [u8],
    header_pos: (usize, usize),
    page_pos_list: Vec<(usize, usize)>,
}

impl<'a> Report5Divider<'a> {
    pub fn new(
        filename: &'a str,
        bytes: &'a [u8],
        pagesize: usize,
    ) -> Result<Option<Report5Divider<'a>>> {
        let header_pos = (0usize, 0usize);
        let page_pos_list = vec![];
        Ok(Some(Report5Divider {
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
        let trowd_value = [92u8, 116, 114, 111, 119, 100];
        for (head, _) in self.bytes.iter().enumerate() {
            let tail = head + 5;
            if !(tail < self.bytes.len()) {
                break;
            };
            if trowd_value.eq(&self.bytes[head..tail + 1]) {
                self.header_pos.1 = head;
                break;
            }
        }
    }
    /// get the position of each pages, store the result in page_pos_list
    fn divide_pages(&mut self) {
        let page_value = [92u8, 112, 97, 103, 101];
        let mut page_pos = (self.header_pos.1, 0usize);
        let mut head = self.header_pos.1;
        while head < self.bytes.len() - 4 {
            let mut tail = head + 4;
            if page_value.eq(&self.bytes[head..tail + 1]) {
                while self.bytes[tail] != 125 {
                    tail += 1;
                }
                page_pos.1 = head;
                self.page_pos_list.push(page_pos.clone());
                head = tail + 1;
                page_pos.0 = head;
            } else {
                head += 1;
            }
        }
        // handle the last page
        page_pos.1 = self.bytes.len() - 1;
        self.page_pos_list.push(page_pos);
    }

    fn write(&self, range: (usize, usize), dest: &Path) -> Result<()> {
        let mut f = fs::OpenOptions::new().create(true).write(true).open(dest)?;
        f.write(&self.bytes[self.header_pos.0..self.header_pos.1])?;
        let first_page = self.page_pos_list[range.0];
        let last_page = if range.1 > self.page_pos_list.len() {
            self.page_pos_list[self.page_pos_list.len() - 1]
        } else {
            self.page_pos_list[range.1 - 1]
        };
        f.write(&self.bytes[first_page.0..last_page.1])?;
        f.write(&[125u8, 125u8])?;
        f.flush().unwrap();
        Ok(())
    }
    fn out_file_path(&self, dest: &Path, filename: &str, index: usize) -> PathBuf {
        PathBuf::from(dest).join(format!("{}_part_{:0>4}{}", filename, index, RTF_EXTENTION))
    }
}
