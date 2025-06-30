use crate::lib::Fuse;

#[test]
fn multibyte_chars_indices() {
    let needle = "f";
    let s = "®f∮";

    let fuse = Fuse::default();
    let pat = fuse.create_pattern(needle);
    let x = fuse
        .search(pat.as_ref(), s)
        .expect("search should return a result");
    let r = &x.ranges[0];

    assert_eq!(&s[r.start..r.end], needle);
}
