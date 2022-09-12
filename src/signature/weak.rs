// weak checksum
#[derive(Copy, Clone)]
pub struct WeakChecksum {
    pub checksum:u32,
    size: usize,
    sum1: u32,
    sum2: u32
}

const base:u32 = 65521;

pub fn calculate_weak(data:&Vec<u8>) -> WeakChecksum {
    let mut chk = WeakChecksum{ checksum:0, size:data.len(), sum1:0, sum2:0 };
    for ch in data {
        chk.sum1 = (chk.sum1 + (*ch as u32)) % base;
        chk.sum2 = (chk.sum2 + chk.sum1) % base;
    }
    chk.checksum = (chk.sum2 << 16) | chk.sum2;
    chk
}

pub fn roll_checksum(checksum: &WeakChecksum, add:u8, remove:u8) -> WeakChecksum {
    let mut chk:WeakChecksum = *checksum;
    
    chk.sum1 += ((add - remove) as u32);
    if chk.sum1 >= base {
        chk.sum1 -= base;
    } else if chk.sum1 < 0 {
        chk.sum1 += base;
    }

    chk.sum2 = ((chk.sum2 - (chk.size as u32) * (remove as u32) + chk.sum1 - 1) as u32) % base;
    if chk.sum2 < 0 {
        chk.sum2 += base;
    }
    chk.checksum = (chk.sum2 << 16) | chk.sum2;
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

}