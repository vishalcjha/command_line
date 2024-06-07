#![allow(dead_code)]
#![allow(unused_variables)]

trait CanBeDoneForStr<Item: ?Sized> {
    fn eval(&self, variable: &Item) -> bool;
}

impl CanBeDoneForStr<i32> for i32 {
    fn eval(&self, variable: &i32) -> bool {
        todo!()
    }
}

impl CanBeDoneForStr<str> for str {
    fn eval(&self, variable: &str) -> bool {
        todo!()
    }
}
