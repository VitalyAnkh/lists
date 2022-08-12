#[macro_use]
extern crate lists;
use derive_builder::Builder;

#[derive(Builder, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MyList {
    head: Option<i32>,
}

fn main() {
    let mut x = 5;
    println!("Hello, world!");
    let list1 = MyListBuilder::default().head(Some(1)).build();
    println!("{:?}", list1);
}
