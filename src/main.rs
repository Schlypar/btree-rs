use std::path::Path;

use lab::db::*;
use lab::db::{goods::*, person::*};
use lab::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Inter {
    inter: i32,
}

#[derive(Debug)]
struct Comparator {}

fn main() -> Result<(), Error> {
    let path = Path::new("test");
    // FileHandler::create_file(path)?;
    let mut db = DataBase::new(path.into())?;

    // db = db.add_record(Crate::new(
    //     Person::new("LAST".into(), "Rep1".into(), "Bat1".into(), 142000),
    //     Person::new("Per2".into(), "Rep2".into(), "Bat2".into(), 142007),
    //     "AVasdawdsad".into(),
    //     "Bcadsadsa".into(),
    //     1,
    // ))?;

    // db = db.add_record(Crate::new(
    //     Person::new("Per3".into(), "Rep3".into(), "Bat3".into(), 142000),
    //     Person::new("Per4".into(), "Rep4".into(), "Bat4".into(), 142007),
    //     "BBB".into(),
    //     "AAA".into(),
    //     1,
    // ))?;

    // db.test_output()?;
    // println!("-----------------------");
    // db.delete_record(0)?;
    db.test_output()?;
    // println!("-----------------------");

    Ok(())
}
