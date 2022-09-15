
use crate::signature::signature::Signature;
use crate::stream;
use crate::signature::strong;

pub struct OriginalPart {
    pub index: u32,
    pub data: Vec<u8>
}

pub fn extract_original_part(sign: &Signature, stream: &mut dyn stream::stream_traits::IStream) ->Option<Vec<OriginalPart>> {
    let mut res: Vec<OriginalPart> = Vec::new();
    while let Ok(block) = stream.read(sign.size()) {
        let strong = strong::calculate_strong(&block);
        if let Some(index) = sign.find_index(strong) {
            res.push( OriginalPart { index: index, data: block } );
        }
    }

    Some(res)
}