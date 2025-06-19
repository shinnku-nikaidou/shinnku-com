use crate::fuse::lib::Fuse;

#[test]
fn multibyte_chars() {
    let pat = "f";
    let s = &[
        "®∮ℕ⊆ℕ₀⊂ℤℚ",
        "😊🥺😉😍😘😚",
        "⡍⠜⠇⠑⠹ ⠺⠁⠎",
        "გთხოვთ",
        "ıntəˈnæʃənəl",
        "γνωρίζω ἀπὸ",
        "コンニチハ",
    ];

    assert!(
        Fuse::default()
            .search_text_in_iterable(pat, s.iter())
            .is_empty()
    );
}

#[test]
fn multibyte_chars_indices() {
    let needle = "f";
    let s = "®f∮";

    let fuse = Fuse::default();
    let pat = fuse.create_pattern(needle);
    let x = fuse.search(pat.as_ref(), s).unwrap();
    let r = &x.ranges[0];

    assert_eq!(&s[r.start..r.end], needle);
}

#[test]
fn full_match_higher() {
    let s = &["Syrup", "Syrup2", "Live", "Live2", "Live3"];

    let fuse = Fuse::default();

    let result1 = fuse.search_text_in_iterable("Syrup", s.iter());

    assert_eq!(result1.len(), 2);
    assert_eq!(result1[0].index, 0);
    assert_eq!(result1[1].index, 1);

    let result2 = fuse.search_text_in_iterable("live", s.iter());

    assert_eq!(result2.len(), 3);
    assert_eq!(result2[0].index, 2);
    assert_eq!(result2[1].score, result2[2].score);
}
