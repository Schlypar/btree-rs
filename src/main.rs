use std::rc::Rc;

use btree::{AlreadyPresent, BTree, Comparator};

fn main() -> Result<(), AlreadyPresent> {
    let comp = Comparator::new(Rc::new(|a: &i32, b: &i32| a.cmp(b)));

    let mut btree: BTree<i32, &str> = match BTree::new(2, comp) {
        Some(tree) => tree,
        None => panic!(),
    };

    btree.insert((1, "first"))?;

    btree.insert((2, "second"))?;

    btree.insert((3, "third"))?;

    btree.insert((4, "fourth"))?;

    btree.insert((5, "fifth"))?;

    btree.insert((7, "sixth"))?;

    btree.insert((8, "seventh"))?;

    btree.insert((9, "eighth"))?;

    btree.insert((-1, "minus first"))?;

    btree.insert((-2, "minus second"))?;

    btree.insert((-3, "minus third"))?;

    btree.insert((-4, "minus fourth"))?;

    btree.insert((-5, "-minus fifth"))?;

    btree.insert((-7, "minus sixth"))?;

    btree.insert((-8, "minus seventh"))?;

    btree.insert((-9, "minus eighth"))?;

    println!("{}", btree);

    Ok(())
}
