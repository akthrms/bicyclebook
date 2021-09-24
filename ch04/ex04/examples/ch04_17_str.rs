#[allow(unused_variables)]
fn main() {
    let s1 = "abc1";
    let s2 = "abc2";
    assert!(s1 < s2);
    assert!(s1 != s2);

    let s3 = "文字列を複数行に渡って書くと
    改行やスペースが入る";
    let s4 = "行末にバックスラッシュを付けると\
    改行などが入らない";

    assert_eq!(s3, "文字列を複数行に渡って書くと\n    改行やスペースが入る");
    assert_eq!(s4, "行末にバックスラッシュを付けると改行などが入らない");

    let s5 = "文字列に\"と\\を含める";
    let s6 = r#"文字列に"と\を含める"#;
    assert_eq!(s5, s6);

    let s7 = r###"このように#の数を増やすと"##"があっても大丈夫"###;
    assert_eq!(s7, "このように#の数を増やすと\"##\"があっても大丈夫");

    let s8 = "もちろん絵文字\u{1f600}も使える";

    let fruits = "あかりんご, あおりんご\nラズベリー, ブラックベリー";

    let mut lines = fruits.lines();
    let apple_line = lines.next();
    assert_eq!(apple_line, Some("あかりんご, あおりんご"));
    assert_eq!(lines.next(), Some("ラズベリー, ブラックベリー"));
    assert_eq!(lines.next(), None);

    if let Some(apples) = apple_line {
        assert!(apples.starts_with("あか"));
        assert!(apples.contains("りんご"));
        assert_eq!(apples.find("あお"), Some(17));

        let mut apple_iter = apples.split(",");
        assert_eq!(apple_iter.next(), Some("あかりんご"));

        let green = apple_iter.next();
        assert_eq!(green, Some(" あおりんご"));
        assert_eq!(green.map(str::trim), Some("あおりんご"));

        assert_eq!(apple_iter.next(), None);
    } else {
        unreachable!();
    }

    let s1 = "a";
    let s2 = "あ";
    let s3 = "😀";
    let s4 = "🇯🇵";

    assert_eq!(s1.len(), 1);
    assert_eq!(s2.len(), 3);
    assert_eq!(s3.len(), 4);
    assert_eq!(s4.len(), 8);

    let s = "abcあいう";
    assert_eq!(s.get(0..1), Some("a"));
    assert_eq!(s.get(3..6), Some("あ"));
    assert_eq!(s.get(3..4), None);

    let s = "かか\u{3099}く";
    println!("{}", s);

    let mut iter = s.chars();
    assert_eq!(iter.next(), Some('か'));
    assert_eq!(iter.next(), Some('か'));
    assert_eq!(iter.next(), Some('\u{3099}'));
    assert_eq!(iter.next(), Some('か'));
    assert_eq!(iter.next(), None);

    let utf8: [u8; 4] = [0x61, 0xe3, 0x81, 0x82];
    assert_eq!(std::str::from_utf8(&utf8), Ok("aあ"));

    let bad_utf8: [u8; 2] = [0x81, 0x33];
    let result2 = std::str::from_utf8(&bad_utf8);
    assert!(result2.is_err());
    println!("{:?}", result2);

    let mut s1 = "abcあいう".to_string();

    let s2 = s1.as_mut_str();

    s2.make_ascii_uppercase();
    assert_eq!(s2, "ABCあいう");

    let b = unsafe { s2.as_bytes_mut() };
    b[3] = b'*';
    b[4] = b'a';
    b[5] = b'*';

    assert_eq!(s1, "ABC*a*いう");
}
