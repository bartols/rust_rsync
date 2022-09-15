use std::collections::HashMap;

struct Chunk {
    strong: String,
    index: u32,
}

pub struct Signature {
    sign_map: HashMap<i32, Chunk>,
    size: u32
}

impl Signature {
    pub fn new(s: &u32) -> Signature {
        Signature{ sign_map: HashMap::new(), size: *s }
    }

    pub fn add(&mut self, weak: i32, idx:u32, strng:String ) {
        println!("add sign {} -> {} idx {}", weak, strng, idx);
        let chunk = Chunk{ strong: strng, index: idx };
        self.sign_map.insert(weak, chunk);
    }
    
    pub fn find(&self, weak: i32) -> Option<(String, u32)> {
        let value = self.sign_map.get(&weak);
        if let Some(v) = value {
            return Some( (v.strong.clone(), v.index) );
        } else {
            None
        }
    }

    pub fn find_index(&self, strong: String) -> Option<u32> {
        for (_, value) in self.sign_map.iter() {
            if value.strong == strong {
                return Some(value.index);
            }
        }
        None
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn is_valid(&self) -> bool {
        !self.sign_map.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_item() {
        let mut sig = Signature::new(&3);
        sig.add(1, 0, "pippo".to_owned());

        assert_eq!(sig.find(1).unwrap().0, "pippo".to_owned());
    }
}