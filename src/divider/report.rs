// use crate::detecter::{Report5Detecter, TypeDetecter};
use anyhow::{Ok, Result};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use super::RTF_EXTENTION;

const SECTD: &[u8] = r"\sectd".as_bytes();
const SECT: &[u8] = r"\sect".as_bytes();
const SECT_SECTD: &[u8] = r"\sect\sectd".as_bytes();
const PAGE_PAR: &[u8] = r"{\page\par}".as_bytes();

pub struct ReportDivider<'a> {
    filename: &'a str,
    pagesize: usize,
    bytes: &'a [u8],
    header_pos: (usize, usize),
    page_pos_list: Vec<(usize, usize)>,
    using_page_break: bool,
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
            using_page_break: false,
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
        for (head, _) in self.bytes.iter().enumerate() {
            let tail = head + SECTD.len() - 1;
            if !(tail < self.bytes.len()) {
                break;
            };
            if SECTD.eq(&self.bytes[head..tail + 1]) {
                self.header_pos.1 = head;
                break;
            }
        }
    }
    /// get the position of each pages, store the result in page_pos_list
    fn divide_pages(&mut self) {
        let mut page_pos = (self.header_pos.1, 0usize);
        let mut head = self.header_pos.1;

        let mut end_of_last_page = 0;

        while head + SECT_SECTD.len() < self.bytes.len() {
            if PAGE_PAR.eq(self.bytes.get(head..head + PAGE_PAR.len()).unwrap()) {
                self.using_page_break = true;
                page_pos.1 = head + PAGE_PAR.len() + 1;
                end_of_last_page = page_pos.1;
                self.page_pos_list.push(page_pos.clone());
                page_pos.0 = page_pos.1;
            }

            if !self.using_page_break
                && SECT_SECTD.eq(self.bytes.get(head..head + SECT_SECTD.len()).unwrap())
            {
                page_pos.1 = head;
                self.page_pos_list.push(page_pos.clone());
                page_pos.0 = head;
            }
            head += 1;
        }

        if head < self.bytes.len() - 1 {
            self.page_pos_list
                .push((end_of_last_page, self.bytes.len() - 1))
        }

        // while head + sectd_len < self.bytes.len() {
        //     // find the position of "\headery"
        //     while head + headery_len < self.bytes.len()
        //         && self.bytes[head..head + headery_len].ne(headery)
        //     {
        //         head += 1;
        //     }
        //     page_pos.0 = head;

        //     // find the position of next "\pard\sect\sectd"
        //     while head + sectd_len < self.bytes.len()
        //         && self.bytes[head..head + sectd_len].ne(sectd)
        //     {
        //         head += 1;
        //     }
        //     page_pos.1 = head;
        //     self.page_pos_list.push(page_pos.clone());
        //     head += 1;
        // }
        // if head < self.bytes.len() - 1 {
        //     if let Some(tail) = self.page_pos_list.pop() {
        //         self.page_pos_list.push((tail.0, self.bytes.len() - 1))
        //     }
        // }
        // let p = self.page_pos_list[3];
        // println!("{:?}", String::from_utf8(self.bytes[p.0..p.1].to_owned()))
    }

    fn write(&self, range: (usize, usize), dest: &Path) -> Result<()> {
        let mut f = fs::OpenOptions::new().create(true).write(true).open(dest)?;
        f.write(&self.bytes[self.header_pos.0..self.header_pos.1])?;
        // f.write(FIRST_PAGE_HEAD)?;
        let first = range.0;
        let last = if range.1 < self.page_pos_list.len() {
            range.1
        } else {
            self.page_pos_list.len()
        };

        for i in first..last {
            let (mut page_start, mut page_end) = self.page_pos_list.get(i).unwrap();

            // compress \sect symbol of each first page to avoid unexpected page break
            if i == first {
                let mut j = page_start;
                while (j + SECT_SECTD.len()).lt(&page_end) {
                    if self
                        .bytes
                        .get(j..j + SECT_SECTD.len())
                        .unwrap()
                        .eq(SECT_SECTD)
                    {
                        page_start = j + SECT.len();
                        break;
                    }
                    if self.bytes.get(j..j + SECTD.len()).unwrap().eq(SECTD) {
                        break;
                    }
                    j += 1;
                }
            }

            // when using {\page\par} to break page, last page need to depress this symbol
            if i == last - 1 && self.using_page_break {
                let mut j = page_end;
                while j.gt(&page_start) {
                    if j.lt(&(self.bytes.len() - PAGE_PAR.len()))
                        && self.bytes.get(j..j + PAGE_PAR.len()).unwrap().eq(PAGE_PAR)
                    {
                        page_end = j;
                        break;
                    }
                    j -= 1;
                }
            }
            f.write(&self.bytes[page_start..page_end])?;
        }
        f.write(&[125u8])?;
        f.flush().unwrap();
        Ok(())
    }
    fn out_file_path(&self, dest: &Path, filename: &str, index: usize) -> PathBuf {
        PathBuf::from(dest).join(format!("{}_part_{:0>4}{}", filename, index, RTF_EXTENTION))
    }
}
