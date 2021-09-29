#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

#[allow(dead_code)]
enum Month {
    January = 1,
    February = 2,
    March = 3,
    December = 12,
}

fn say_something(weekday: Weekday) {
    if weekday == Weekday::Friday {
        println!("TGIF!");
    } else {
        println!("まだ{:?}か", weekday);
    }
}

fn main() {
    say_something(Weekday::Friday);

    assert_eq!(Month::March as isize, 3);
}
