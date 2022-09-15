// strong checksum
use sha1::{Sha1, Digest};
use std::{fmt::Write};

pub fn calculate_strong(data:&Vec<u8>) -> String {
    let mut hasher = Sha1::new();

    hasher.update(data);
    let result = hasher.finalize();
    let mut res_str = String::with_capacity(result.len()*2);
    for b in result {
        write!(&mut res_str, "{:02x}", b).unwrap();
    }
    res_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let data = Vec::<u8>::new();
        let chk = calculate_strong(&data);
        assert_eq!(chk, "da39a3ee5e6b4b0d3255bfef95601890afd80709".to_string());
    }

}