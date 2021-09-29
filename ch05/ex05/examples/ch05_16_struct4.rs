struct UserName(String);

struct Id(i64);

struct Timestamp(i64);

#[allow(dead_code)]
type User = (Id, UserName, Timestamp);

#[allow(dead_code)]
fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
    (id, name, created)
}

#[allow(unused_variables)]
fn main() {
    let id = Id(400);
    let now = Timestamp(4567890123);

    // let bad_user = new_user(UserName(String::from("kazuki")), now, id);
}
