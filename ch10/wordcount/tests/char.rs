use std::io::Cursor;
use wordcount::{count, CountOption};

#[macro_use]
mod utils;

#[test]
fn char_count_works() {
    let input = Cursor::new(b"abadracadabra");
    let freqs = count(input, CountOption::Char);

    assert_map!(freqs, {"a" => 6, "b" => 2, "c" => 1, "d" => 2, "r" => 2});
}

#[test]
fn char_count_utf8() {
    let input = Cursor::new(
        r#"
天地玄黃
宇宙洪荒
日月盈昃
辰宿列張
"#,
    );

    let freqs = count(input, CountOption::Char);

    for (_, count) in freqs {
        assert_eq!(count, 1);
    }
}
