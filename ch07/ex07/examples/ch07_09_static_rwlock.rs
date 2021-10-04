use lazy_static::lazy_static;
use std::collections::HashSet;
use std::error::Error;
use std::sync::RwLock;

lazy_static! {
    pub static ref DOGS: RwLock<HashSet<&'static str>> = {
        let dogs = ["柴", "トイプードル"].iter().cloned().collect();
        RwLock::new(dogs)
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    {
        let ds = DOGS.read()?;
        assert!(ds.contains("柴"));
        assert!(ds.contains("トイプードル"));
    }

    fn stringify(x: impl ToString) -> String {
        x.to_string()
    }

    DOGS.write()?.insert("ブル・テリア");

    std::thread::spawn(move || {
        DOGS.write()
            .map(|mut ds| ds.insert("コーギー"))
            .map_err(stringify)
    })
    .join()
    .expect("Thread error")?;

    assert!(DOGS.read().map_err(stringify)?.contains("ブルテリア"));
    assert!(DOGS.read().map_err(stringify)?.contains("コーギー"));

    Ok(())
}
