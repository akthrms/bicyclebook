type UserName = String;

type Id = i64;

type Timestamp = i64;

type User = (Id, UserName, Timestamp);

fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
    (id, name, created)
}

#[allow(unused_variables)]
fn main() {
    let id = 400;
    let now = 4567890123;
    let user = new_user(String::from("mika"), id, now);

    let bad_user = new_user(String::from("kazuki"), now, id);
}
