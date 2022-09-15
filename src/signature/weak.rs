// weak checksum
#[derive(Default, Copy, Clone)]
pub struct WeakChecksum {
    pub checksum:i32,
    size: usize,
    sum1: i32,
    sum2: i32
}

const BASE:i32 = 65521;

pub fn calculate_weak(data:&Vec<u8>) -> WeakChecksum {
    let mut chk = WeakChecksum{ checksum:0, size:data.len(), sum1:1, sum2:0 };
    for ch in data {
        chk.sum1 = (chk.sum1 + (*ch as i32)) % BASE;
        chk.sum2 = (chk.sum2 + chk.sum1) % BASE;
    }
    chk.checksum = (chk.sum2 << 16) | chk.sum1;
    chk
}

pub fn roll_checksum(checksum: &WeakChecksum, remove:u8, add:u8) -> WeakChecksum {
    let mut chk:WeakChecksum = *checksum;
    
    chk.sum1 += (add as i32) - (remove as i32);
    if chk.sum1 >= BASE {
        chk.sum1 -= BASE;
    } else if chk.sum1 < 0 {
        chk.sum1 += BASE;
    }

    chk.sum2 = ((chk.sum2 - (chk.size as i32) * (remove as i32) + chk.sum1 - 1) as i32) % BASE;
    if chk.sum2 < 0 {
        chk.sum2 += BASE;
    }
    chk.checksum = (chk.sum2 << 16) | chk.sum1;
    chk
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let data = Vec::<u8>::new();
        let chk = calculate_weak(&data);
        assert_eq!(chk.checksum, 1);
    }

    #[test]
    fn test_roll() {
        let data1: Vec<u8> = "abc".as_bytes().to_vec();
        let chk1 = calculate_weak(&data1);

        let data2: Vec<u8> = "wab".as_bytes().to_vec();
        let chk2 = calculate_weak(&data2);
        let chk3 = roll_checksum(&chk2, 'w' as u8, 'c' as u8 );

        assert_eq!(chk1.checksum, chk3.checksum)
    }

}