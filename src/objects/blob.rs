use crate::objects::Object;
use std::io::BufRead;

/// This object carry the content of a file.
pub struct Blob {
    pub data: Vec<u8>,
}

impl Blob {
    pub fn new(data: Vec<u8>) -> Blob {
        Blob { data }
    }
}

impl Object for Blob {
    fn dump(&self) -> Vec<u8> {
        let header = format!("blob {}\0", self.data.len());
        let mut res = vec![];
        res.reserve(self.data.len() + header.len());
        res.append(&mut header.into_bytes());
        res.append(&mut self.data.clone());
        res
    }

    fn from<R: BufRead>(mut reader: R) -> Box<Blob> {
        let mut buff = vec![];
        reader.read_until(0, &mut buff).unwrap();
        assert!(std::str::from_utf8(&buff[..5])
            .unwrap()
            .starts_with("blob "));
        buff.clear();
        reader.read_to_end(&mut buff).unwrap();
        Box::new(Blob::new(buff))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blob_dump() {
        let blob = Blob::new(String::from("Hey").into_bytes());
        let dump = blob.dump();
        assert_eq!(dump.len(), 10);
        assert_eq!(std::str::from_utf8(&dump).unwrap(), "blob 3\0Hey");
    }
}
