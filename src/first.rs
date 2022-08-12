use derive_builder::Builder;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node {
    elem: i32,
    next: List,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct List {
    head: Link,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list() {
        let list = Elem(1, Box::new(Elem(2, Box::new(Elem(3, Box::new(Empty))))));
        //assert_eq!(list, List::Elem(1, Box::new(2)));
    }
}
