use std::convert::TryInto;

pub mod stream;
pub mod signature;
pub mod delta;

use signature::signature::Signature;
use signature::weak;
use signature::strong;
use crate::delta::Delta;

pub fn signature(basis_file: &mut dyn stream::stream_traits::IStream, size: u32) -> Signature {
    let mut res = Signature::new();
    let mut index:u32 = 0;

    while let Ok(block) = basis_file.read(size) {
        if block.len() < size.try_into().unwrap() {
            continue;
        }
        let weak_sig = weak::calculate_weak(&block).checksum;
        let strong_sig = strong::calculate_strong(&block);
        
        let strong_opt = res.find(weak_sig);
        if let Some(s) = strong_opt {
            if s != strong_sig {
            // todo else ???
        }
        } else {
            res.add(weak_sig, index, strong_sig);
            index += 1;
        }
    }

    res
}

// pub fn delta(sign: &Signature, new_file: &mut dyn stream::stream_traits::IStream) -> Delta {

// }
