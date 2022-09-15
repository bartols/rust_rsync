use rsync;
use rsync::stream::stream_traits::IStream;
use rsync::stream::string_stream;

fn apply_patch(original: &str, modified: &str, size: u32) {
    assert!(true);

    // input stream
    let original_vec = original.as_bytes().to_vec();
    let mut istream_orig = string_stream::StringIStream::new(&original_vec);

    // calculate sign
    let sign = rsync::signature(&mut istream_orig, size);
    assert!(sign.is_valid());

    // delta
    let modified_vec = modified.as_bytes().to_vec();
    let mut istream_mod = string_stream::StringIStream::new(&modified_vec);
    let delta = rsync::delta(&sign, &mut istream_mod);
    assert!(delta.is_valid());

    // patch
    let mut os = string_stream::StringOStream::new();
    istream_orig.restart();
    assert!(rsync::patch(&sign, &mut istream_orig, &delta, &mut os));
    assert_eq!(*os.result(), modified.to_owned());
}

#[test]
fn equal() {
    apply_patch("abc", "abc", 3);
}

#[test]
fn block_change() {
	apply_patch("abc", "abcd", 3);
}

#[test]
fn block_change_long(){
	apply_patch("abc", "abcdefffffffffff", 3);
}

#[test]
fn change_block() {
	apply_patch("abc", "ddabc", 3);
}

#[test]
fn block_change_block() {
	apply_patch("abc", "abcddabc", 3);
}

#[test]
fn change_block_change() {
	apply_patch("abc", "wwwwabczzzz", 3);
}

#[test]
fn two_block() {
	apply_patch("abcdef", "abcdef", 3);
}

#[test]
fn two_block_change() {
	apply_patch("abcdef", "abcdefd", 3);
}

#[test]
fn long_text() {
	apply_patch("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Lorem ipsum acta est. Vestibulum eu sapien felis. Aenean id turpis. Vestibulum id turbis.", 
				"Lorem ipsum sit amet, consectetur dolor adipiscing elit. Gaude Lorem ipsum acta est. Sapien felis. Aenean id turpis. Vestibulum id turbis.", 11);
}
