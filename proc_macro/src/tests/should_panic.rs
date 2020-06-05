use regex::Regex;

#[test]
fn match_expected() {
    // in form of (raw, expected)
    let test_vector = vec![
        ("#[should_panic]", ""),
        (r#"#[should_panic(expected = "hello")]"#, "hello"),
        (r#"#[should_panic(expected="hello")]"#, "hello"),
        (r#"#[should_panic(expected ="hello")]"#, "hello"),
        (r#"#[should_panic(expected = "hello")]"#, "hello"),
        (r#"#[should_panic(expected =  "hello")]"#, "hello"),
    ];

    let r = Regex::new(r#"#\[should_panic(\(expected\s*=\s*"(.*)"\))?\]"#).unwrap();
    for c in test_vector {
        let got = r
            .captures(c.0)
            .expect("invalid #[should_panic] attribute")
            .get(2)
            .map_or("", |m| m.as_str());

        let expected = c.1;
        assert_eq!(expected, got);
    }
}
