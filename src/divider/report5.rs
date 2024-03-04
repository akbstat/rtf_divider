// use crate::detecter::{Report5Detecter, TypeDetecter};
use anyhow::Result;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use super::{field, RTF_EXTENTION};

const TROWD: &[u8] = r"\trowd".as_bytes();
const PAGE: &[u8] = r"\page".as_bytes();
const FIELD: &[u8] = r"{\field".as_bytes();
const CLOSE_BRACE: u8 = b'}';
const SPACE: u8 = b' ';
const NEWLINE: u8 = b'\n';
const RETURN: u8 = b'\r';

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
        for (head, _) in self.bytes.iter().enumerate() {
            let tail = head + 5;
            if !(tail < self.bytes.len()) {
                break;
            };
            if TROWD.eq(&self.bytes[head..tail + 1]) {
                self.header_pos.1 = head;
                break;
            }
        }
    }
    /// get the position of each pages, store the result in page_pos_list
    fn divide_pages(&mut self) {
        let mut page_pos = (self.header_pos.1, 0usize);
        let mut head = self.header_pos.1;
        while head < self.bytes.len() - 4 {
            let mut tail = head + 4;
            if PAGE.eq(&self.bytes[head..tail + 1]) {
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
        let content = &self.bytes[first_page.0..last_page.1];
        let mut content_start = 0;
        // remove auto field informations
        for (i, _) in content.iter().enumerate() {
            if i < FIELD.len() {
                continue;
            }
            if let Some(mark) = content.get(i - FIELD.len()..i) {
                if FIELD.eq(mark) {
                    let mut fill_close_brace = false;
                    let mut content_end = i - FIELD.len() - 1;
                    while let Some(c) = content.get(content_end) {
                        if SPACE.ne(c) && NEWLINE.ne(c) && RETURN.ne(c) {
                            if CLOSE_BRACE.eq(c) {
                                fill_close_brace = true;
                            } else {
                                content_end += 1;
                            }
                            break;
                        }
                        content_end -= 1;
                    }
                    f.write(&content[content_start..content_end])?;
                    let field_area_start = i - FIELD.len();
                    let field = field::handle_field(content.get(field_area_start..).unwrap());
                    content_start = field.tail + field_area_start + 1;
                    if let Some(page) = field.page {
                        f.write(page.as_bytes())?;
                        if fill_close_brace {
                            f.write(&[CLOSE_BRACE])?;
                        }
                    }
                    continue;
                }
            }
        }

        f.write(&content[content_start..])?;
        // f.write(content)?;
        f.write(&[125u8, 125u8])?;
        f.flush().unwrap();
        Ok(())
    }
    fn out_file_path(&self, dest: &Path, filename: &str, index: usize) -> PathBuf {
        PathBuf::from(dest).join(format!("{}_part_{:0>4}{}", filename, index, RTF_EXTENTION))
    }
}

// fn extract_page_number(source: &[u8]) -> Vec<u8> {
//     let mut tail = source.len() - 1;
//     let mut page_number_start = 0;
//     let mut page_number_end = tail;
//     while let Some(c) = source.get(tail) {
//         if CLOSE_BRACE.ne(c) && NEWLINE.ne(c) && SPACE.ne(c) && RETURN.ne(c) {
//             page_number_end = tail + 1;
//             break;
//         }
//         tail -= 1;
//     }
//     while let Some(c) = source.get(tail) {
//         if SPACE.ne(c) {
//             page_number_end = tail + 1;
//             break;
//         }
//         tail -= 1;
//     }

//     while let Some(c) = source.get(tail) {
//         if SPACE.eq(c) {
//             page_number_start = tail + 1;
//             break;
//         }
//         tail -= 1;
//     }
//     source
//         .get(page_number_start..page_number_end)
//         .unwrap()
//         .to_owned()
// }

#[cfg(test)]
mod test_report5 {
    // use super::*;

    // #[test]
    // fn extract_page_number_test() {
    //     let source = r"{\field{\*\fldinst {\rtlch\fcs1 \af44 \ltrch\fcs0 \f44\cf1\insrsid16154060 \hich\af44\dbch\af31505\loch\f44  PAGE }}{\fldrslt {\rtlch\fcs1 \af44 \ltrch\fcs0 \f44\cf1\lang1024\langfe1024\noproof\insrsid16019468 \hich\af44\dbch\af31505\loch\f44 2}}}
    //     ".as_bytes();
    //     assert_eq!(
    //         "2",
    //         String::from_utf8_lossy(&extract_page_number(source)).to_string()
    //     );

    //     let source = &fs::read(Path::new(
    //         r"D:\Studies\ak112\303\stats\CSR\product\output\asco\t-14-01-03-01-dm-fas.rtf",
    //     ))
    //     .unwrap()[50274..50523];
    //     assert_eq!(
    //         "2",
    //         String::from_utf8_lossy(&extract_page_number(source)).to_string()
    //     );
    // }
}
