#[derive(Debug, PartialEq)]
pub enum Error {
    Eof,
    FileNotExist,
    GenError
}

// an input stream trait
pub trait IStream {
    // read
    fn read(&mut self, size: u32) -> Result<Vec<u8>,Error>;

    // restart reading from head
    fn restart(&mut self);
}

// an output stream trait
pub trait OStream {
    // write a block
    fn write_block(&mut self, data: &Vec<u8>) -> Result<(), Error>;
}
