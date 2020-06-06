use regex::Regex;

#[test]
fn match_should_panic() {
    // in form of (raw, expected)
    let test_vector = vec![
        ("#[should_panic]", ""),
        (r#"#[should_panic(expected = "hello")]"#, "hello"),
        (r#"#[should_panic(expected="hello")]"#, "hello"),
        (r#"#[should_panic(expected ="hello")]"#, "hello"),
        (r#"#[should_panic(expected = "hello")]"#, "hello"),
        (r#"#[should_panic(expected =  "hello")]"#, "hello"),
        (
            r#"#[should_panic(expected = 
            "hello")]"#,
            "hello",
        ),
        (
            r#"#[should_panic(expected = "hello
world")]"#,
            "hello\nworld",
        ),
    ];

    // '.'  doesn't match newline as stated at https://docs.rs/regex/1.3.9/regex/index.html#matching-one-character
    let r = Regex::new(r#"^#\[should_panic(\(expected\s*=\s*"((?s).*)"\))?\]$"#).unwrap();
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

#[test]
fn match_should_panic_then_ignore() {
    // in form of (raw, expected, ignored)
    let test_vector = vec![
        (r"#[should_panic]", "", false),
        (r#"#[should_panic(expected = "hello")]"#, "hello", false),
        (r#"#[should_panic(expected="hello")]"#, "hello", false),
        (r#"#[should_panic(expected ="hello")]"#, "hello", false),
        (r#"#[should_panic(expected = "hello")]"#, "hello", false),
        (r#"#[should_panic(expected =  "hello")]"#, "hello", false),
        (
            r#"#[should_panic(expected = 
            "hello")]"#,
            "hello",
            false,
        ),
        (
            r#"#[should_panic(expected = "hello
world")]"#,
            "hello\nworld",
            false,
        ),
        (r"#[should_panic]#[ignore]", "", true),
        (r"#[should_panic] #[ignore]", "", true),
        (
            r"#[should_panic]
#[ignore]",
            "",
            true,
        ),
        ("#[ignore]", "", true),
        (r"#[ignore]#[should_panic]", "", true),
        (
            r"#[ignore]
#[should_panic]",
            "",
            true,
        ),
        (
            r#"#[ignore]
#[should_panic(expected = "hello")]"#,
            "hello",
            true,
        ),
    ];

    let (should_panic_then_ignore, ignore_then_should_panic) = {
        // '.'  doesn't match newline as stated at
        // https://docs.rs/regex/1.3.9/regex/index.html#matching-one-character
        let should_panic = r#"#\[should_panic(\(expected\s*=\s*"((?s).*)"\))?\]"#;
        let ignore = r"#\[ignore\]";

        let should_panic_then_ignore = format!(r"^{}\s*({})?$", should_panic, ignore);
        let ignore_then_should_panic = format!(r"^({})\s*({})?$", ignore, should_panic);

        (should_panic_then_ignore, ignore_then_should_panic)
    };

    let r1 = Regex::new(&should_panic_then_ignore).unwrap();
    let r2 = Regex::new(&ignore_then_should_panic).unwrap();

    let run =
        |&(msg, should_panic_expect, ignored_expected), pattern: &Regex, expect_idx, ignore_idx| {
            let groups = match pattern.captures(msg) {
                Some(v) => v,
                None => return false,
            };

            println!("groups: {:?}", groups);

            let should_panic_got = groups.get(expect_idx).map_or("", |m| m.as_str());
            assert_eq!(
                should_panic_expect, should_panic_got,
                "unexpected msg for should_panic"
            );

            let ignored_got = groups.get(ignore_idx);
            assert_eq!(
                ignored_expected,
                ignored_got.is_some(),
                "mismatched #[ignore]"
            );

            true
        };

    for (i, c) in test_vector.iter().enumerate() {
        println!("--{}", c.0);
        assert!(run(c, &r1, 2, 3,) || run(c, &r2, 4, 1), "#{} failed", i);
    }
}
