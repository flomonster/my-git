use crate::objects::Object;

pub struct Blob {
    data: Vec<u8>,
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
