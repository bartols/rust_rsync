use std::collections::HashMap;

struct Chunk {
    strong: String,
    index: u32
}

pub struct Signature {
    sign_map: HashMap<i64, Chunk>
}

impl Signature {
    pub fn new() -> Signature {
        Signature{ sign_map: HashMap::new() }
    }

    pub fn add(&mut self, weak: i64, idx:u32, strng:&String ) {
        let chunk = Chunk{ strong: strng.clone(), index: idx };
        self.sign_map.insert(weak, chunk);
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        assert!(true);
    }

}