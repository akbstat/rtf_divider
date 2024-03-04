const FLDINST: &[u8] = r"\*\fldinst".as_bytes();
const FLDRSLT: &[u8] = r"\fldrslt".as_bytes();
const OPEN_BRACE: u8 = b'{';
const CLOSE_BRACE: u8 = b'}';
const SPACE: u8 = b' ';

pub struct Field {
    pub page: Option<String>,
    pub tail: usize,
}

pub fn handle_field(data: &[u8]) -> Field {
    let mut field = Field {
        page: None,
        tail: 0,
    };
    let size = data.len();
    let mut i = 0;
    let mut fldinst = (0, 0);
    let mut fldrslt = (0, 0);
    let mut balance = 0;
    while i < size {
        if let Some(c) = data.get(i) {
            if OPEN_BRACE.eq(c) {
                balance -= 1;
            }
        }
        if let Some(c) = data.get(i) {
            if CLOSE_BRACE.eq(c) {
                balance += 1;
            }
        }
        if i < FLDINST.len() {
            i += 1;
            continue;
        }
        if let Some(byte) = data.get(i - FLDINST.len() + 1..i + 1) {
            if FLDINST.eq(byte) {
                while i < size {
                    if OPEN_BRACE.eq(data.get(i).unwrap()) {
                        balance -= 1;
                        fldinst.0 = i + 1;
                    }
                    if CLOSE_BRACE.eq(data.get(i).unwrap()) {
                        balance += 1;
                        fldinst.1 = i;
                        break;
                    }
                    i += 1;
                }
            }
        }
        if let Some(byte) = data.get(i - FLDRSLT.len() + 1..i + 1) {
            if FLDRSLT.eq(byte) {
                while i < size {
                    if OPEN_BRACE.eq(data.get(i).unwrap()) {
                        balance -= 1;
                        fldrslt.0 = i + 1;
                    }
                    if CLOSE_BRACE.eq(data.get(i).unwrap()) {
                        balance += 1;
                        fldrslt.1 = i;
                        break;
                    }
                    i += 1;
                }
            }
        }
        if balance == 0 {
            field.tail = i;
            break;
        }
        i += 1;
    }
    // findout page number
    if (0, 0).ne(&fldrslt) {
        let mut tail = 0;
        let mut i = fldrslt.1;
        let mut tail_part = true;
        while let Some(c) = data.get(i) {
            if SPACE.ne(c) && tail_part {
                tail = i;
                tail_part = !tail_part;
                i -= 1;
                continue;
            }
            if SPACE.eq(c) && (!tail_part) {
                let n = data.get(i + 1..tail).unwrap();
                field.page = Some(String::from_utf8(n.to_vec()).unwrap());
                return field;
            }
            i -= 1;
        }
    }
    field
}

#[cfg(test)]
mod field_test {
    use super::*;
    #[test]
    fn handle_field_test() {
        let source = r"{\field{\*\fldinst {\rtlch\fcs1 
            \af44\afs21 \ltrch\fcs0 \fs21\cf1\loch\af44\hich\af44\dbch\af13\insrsid9375243 \hich\af44\dbch\af13\loch\f44  PAGE }}{\fldrslt {\rtlch\fcs1 \af44\afs21 \ltrch\fcs0 \fs21\cf1\lang1024\langfe1024\loch\af44\hich\af44\dbch\af13\noproof\insrsid16413834 
            \hich\af44\dbch\af13\loch\f44 1}}}".as_bytes();
        let field = handle_field(source);
        assert_eq!(field.page, Some("1".into()));
        assert_eq!(field.tail, source.len() - 1);

        let source = r"{\field{\*\fldinst { PAGE }}} {".as_bytes();
        let field = handle_field(source);
        assert_eq!(field.page, None);
        assert_eq!(field.tail, source.len() - 3);
    }
}
