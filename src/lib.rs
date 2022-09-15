pub mod stream;
mod signature;
mod delta;
mod extract;

use crate::signature::signature::Signature;
use signature::weak;
use signature::strong;
use crate::delta::Delta;
use crate::delta::Part;
use stream::string_stream;

pub fn signature(basis_file: &mut dyn stream::stream_traits::IStream, size: u32) -> Signature {
    let mut res = Signature::new(&size);
    let mut index:u32 = 0;

    while let Ok(block) = basis_file.read(size) {
        if block.len() < size.try_into().unwrap() {
            continue;
        }
        let weak_sig = weak::calculate_weak(&block).checksum;
        let strong_sig = strong::calculate_strong(&block);
        
        let strong_opt = res.find(weak_sig);
        if let Some((s, _)) = strong_opt {
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

pub fn delta(sign: &Signature, new_file: &mut dyn stream::stream_traits::IStream) -> Delta {
    let mut res = Delta::default();
    let mut roll = false;
    let mut weak = weak::WeakChecksum::default();
    let mut remove_byte = 0x00;
    let mut block:Vec<u8> = Vec::new();

    loop {
        if roll {
            if let Ok(b) = new_file.read(1) {
                weak = weak::roll_checksum(&weak, remove_byte, b[0]);
                block.push(b[0]);
            } else {
                res.add_changes(block);
                break;
            }
        } else {
            // read block
            // if let Ok(block) = new_file.read(sign.size()) {
            //     weak = weak::calculate_weak(&block);
            // } else {
            //     break;
            // }
            block = match new_file.read(sign.size()) {
                Ok(b) => b,
                Err(_) => break
            };
        }

        roll = false;

        if let Some((strong, index)) = sign.find(weak.checksum) {
            let strong_block = strong::calculate_strong(&block);
            if strong == strong_block {
                res.add_index(index);
                continue;
            }
        }

        // not found
        roll = true;
        remove_byte = block[0];
        block.remove(0);
        res.add_byte(remove_byte);
    }

    res
}

pub fn patch(sign: &Signature, basis_file: &mut dyn stream::stream_traits::IStream, delta: &Delta, output_file: &mut dyn stream::stream_traits::OStream) -> bool {
    let original_part = match extract::extract_original_part(sign, basis_file) {
        None => return false,
        Some(original_part) => original_part
    };

    for part in delta.iter() {
        match part {
            Part::Index(index) => {
                let it = match original_part.iter().find(|&o| o.index == index) {
                    None => return false,
                    Some(it) => it
                };
                if let Err(_) = output_file.write_block(&it.data) {
                    return false;
                }
            }
            Part::Block(block) => {
                // if new
                if let Err(_) = output_file.write_block(&block) {
                    return false;
                }
            }
        }
    }

    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta() {
        let size:u32 = 3;

         // input stream
        let original_vec = "abc".as_bytes().to_vec();
        let mut istream_orig = string_stream::StringIStream::new(&original_vec);

        // calculate sign
        let sign = signature(&mut istream_orig, size);
        assert!(sign.is_valid());

        // delta
        let modified_vec = "abc".as_bytes().to_vec();
        let mut istream_mod = string_stream::StringIStream::new(&modified_vec);
        let delta = delta(&sign, &mut istream_mod);
        assert!(delta.is_valid());
    }
}