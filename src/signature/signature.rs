use std::collections::HashMap;

struct Chunk {
    strong: String,
    index: u32,
}

pub struct Signature {
    sign_map: HashMap<i32, Chunk>
}

impl Signature {
    pub fn new() -> Signature {
        Signature{ sign_map: HashMap::new() }
    }

    pub fn add(&mut self, weak: i32, idx:u32, strng:String ) {
        let chunk = Chunk{ strong: strng, index: idx };
        self.sign_map.insert(weak, chunk);
    }
    
    pub fn find(&self, weak: i32) -> Option<String> {
        let value = self.sign_map.get(&weak);
        if let Some(v) = value {
            return Some(v.strong.clone());
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_item() {
        let mut sig = Signature::new();
        sig.add(1, 0, "pippo".to_owned());

        assert_eq!(sig.find(1).unwrap(), "pippo".to_owned());
    }
}