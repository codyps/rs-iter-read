use std::io::{self, Read};

pub struct T<S: Iterator<Item=u8>>(pub S);

impl <S: Iterator<Item=u8>> Read for T<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut ct = 0usize;
        for b in buf.iter_mut() {
            if let Some(v) = self.0.next() {
                *b = v;
                ct += 1;
            } else {
                break;
            }
        }

        Ok(ct)
    }
}


#[test]
fn basic() {
    use std::io::Cursor;
    let mut r = T(vec![4u8, 6, 2].into_iter());
    let mut buf = [0u8; 4];
    assert_eq!(r.read(&mut buf).unwrap(), 3);
    assert_eq!(buf[0], 4);
    assert_eq!(buf[1], 6);
    assert_eq!(buf[2], 2);
}
