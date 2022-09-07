//
// String stream
//

use std::{convert::TryInto, iter::FromIterator};

use crate::stream::stream_traits::*;

// an input string stream
struct StringIStream {
   data: Vec<u8>, 
   pos: usize,
}

impl StringIStream {
    pub fn new(init_data: &Vec<u8>) -> StringIStream {
        StringIStream{data: init_data.clone(), pos: 0}
    }
}

impl IStream for StringIStream {
        // read
        fn read(&mut self, size: u32) -> Result<Vec<u8>,Error> {
            if self.pos >= self.data.len() {
                return Err(Error::Eof);
            }
            let mut size = size as usize;
            if (size + self.pos) > self.data.len() {
                size = self.data.len() - self.pos;
            }

            let mut res = Vec::<u8>::with_capacity(size.try_into().unwrap());
            for i in self.pos..self.pos+size {
                res.push(self.data[i as usize]);
            }
            self.pos += size;
            Ok(res)
        }

        // restart reading from head
        fn restart(mut self) {
            self.pos = 0;
        }
}

// an output string stream
struct StringOStream {
    data: String,
}

impl OStream for StringOStream {
    // write a block
    fn write_block(&mut self, data: &Vec<u8>) -> Result<(),Error>
    {
        for c in data {
            self.data.push((*c) as char);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_empty() {
        let data = Vec::<u8>::new();
        let mut ss = StringIStream::new(&data);
        let res = ss.read(3);
        assert_eq!(res, Err(Error::Eof));
    }

    #[test]
    fn read_block() {
        let data = Vec::<u8>::from([0x30, 0x31, 0x32]);
        let mut ss = StringIStream::new(&data);
        let res = ss.read(3);
        assert_eq!(res.unwrap(), data);
    }

    #[test]
    fn read_block_the_eof() {
        let data = Vec::<u8>::from([0x30, 0x31, 0x32]);
        let mut ss = StringIStream::new(&data);
        let res = ss.read(3);
        assert_eq!(res.unwrap(), data);

        let res = ss.read(3);
        assert_eq!(res, Err(Error::Eof));
    }

    #[test]
    fn read_off() {
        let data = Vec::<u8>::from([0x30, 0x31, 0x32, 0x33]);
        let mut ss = StringIStream::new(&data);

        let res = ss.read(1);
        assert_eq!(res.unwrap(), vec![0x30]);

        let res = ss.read(3);
        assert_eq!(res.unwrap(), vec![0x31, 0x32, 0x33]);
    }

    #[test]
    fn read_off_eof() {
        let data = Vec::<u8>::from([0x30, 0x31, 0x32, 0x33]);
        let mut ss = StringIStream::new(&data);

        let res = ss.read(1);
        assert_eq!(res.unwrap(), vec![0x30]);

        let res = ss.read(3);
        assert_eq!(res.unwrap(), vec![0x31, 0x32, 0x33]);

        let res = ss.read(1);
        assert_eq!(res, Err(Error::Eof));
    }

    #[test]
    fn read_less_than_buffer() {
        let data = Vec::<u8>::from([0x30, 0x31, 0x32]);
        let mut ss = StringIStream::new(&data);
        let res = ss.read(5);
        assert_eq!(res.unwrap(), vec![0x30, 0x31, 0x32]);
    }

    #[test]
    fn write_ostream() {
        let mut os = StringOStream{ data: "".to_owned()};
        let data = Vec::<u8>::from([0x30, 0x31, 0x32]);
        
        assert_eq!(os.write_block(&data), Ok(()));
        assert_eq!(os.data, "012".to_owned());
    }
}