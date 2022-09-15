use std::{fmt::Write};
use std::str;

#[derive(Clone)]
pub enum Part {
    Index(u32),
    Block(Vec<u8>)
}

#[derive(Default)]
pub struct Delta {
    parts: Vec<Part>
}

impl Delta {
    pub fn new() -> Delta {
        Delta { parts: Vec::new() }
    }

    pub fn add_changes(&mut self, data: Vec<u8>) {
        self.parts.push(Part::Block(data));
    }

    pub fn add_index(&mut self, index: u32) {
        self.parts.push(Part::Index(index));
    }

    pub fn add_byte(&mut self, b: u8) {
        if self.parts.is_empty() {
            self.add_changes(vec![b]);
            return;
        }

        match self.parts.last_mut().unwrap() {
            Part::Index(_) => self.add_changes(vec![b]),
            Part::Block(bl) => bl.push(b)
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.parts.is_empty()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Part> + 'a {
        self.parts.iter().cloned()
    }

    pub fn  dump(&self) -> String {
        let mut res = String::new();
        for part in &self.parts {
            match part {
                Part::Index(i) => write!(&mut res, "<b*{}*>", i).unwrap(),
                Part::Block(b) => res += str::from_utf8(b).unwrap()
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dump_part() {
        let mut delta = Delta::new();
        delta.add_changes(vec![b'a', b'b', b'c']);

        assert_eq!(delta.dump(), "abc".to_owned());
    }

    #[test]
    fn dump_index() {
        let mut delta = Delta::new();
        delta.add_index(0);

        assert_eq!(delta.dump(), "<b*0*>".to_owned());
    }

    #[test]
    fn dump_part_index() {
        let mut delta = Delta::new();
        delta.add_changes(vec![b'a', b'b', b'c']);
        delta.add_index(0);

        assert_eq!(delta.dump(), "abc<b*0*>".to_owned());
    }
    
    #[test]
    fn dump_index_part() {
        let mut delta = Delta::new();
        delta.add_index(0);
        delta.add_changes(vec![b'a', b'b', b'c']);

        assert_eq!(delta.dump(), "<b*0*>abc".to_owned());
    }

    #[test]
    fn dump_part_index_part() {
        let mut delta = Delta::new();
        delta.add_index(0);
        delta.add_changes(vec![b'a', b'b', b'c']);
        delta.add_index(0);

        assert_eq!(delta.dump(), "<b*0*>abc<b*0*>".to_owned());
    }

    #[test]
    fn dump_part_index_different_part() {
        let mut delta = Delta::new();
        delta.add_index(0);
        delta.add_changes(vec![b'a', b'b', b'c']);
        delta.add_index(1);

        assert_eq!(delta.dump(), "<b*0*>abc<b*1*>".to_owned());
    }

    #[test]
    fn dump_add_byte() {
        let mut delta = Delta::new();
        delta.add_byte(b'a');
        assert_eq!(delta.dump(), "a".to_owned());
    }

    #[test]
    fn dump_add_bytes() {
        let mut delta = Delta::new();
        delta.add_byte(b'a');
        delta.add_byte(b'b');
        delta.add_byte(b'c');
        assert_eq!(delta.dump(), "abc".to_owned());
    }

    #[test]
    fn dump_add_byte_to_block() {
        let mut delta = Delta::new();
        delta.add_changes(vec![b'a', b'b', b'c']);
        delta.add_byte(b'd');
        assert_eq!(delta.dump(), "abcd".to_owned());
    }

    
    #[test]
    fn dump_add_byte_to_part() {
        let mut delta = Delta::new();
        delta.add_index(0);
        delta.add_byte(b'd');
        assert_eq!(delta.dump(), "<b*0*>d".to_owned());
    }
}