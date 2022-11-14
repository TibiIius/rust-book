// this function takes a generic type `T` as an argument
// T's size is not known at compile time, so we need to specify it with `<t: ?Sized>`
fn do_something_with_dynamically_sized_type<T: ?Sized>(t: &T) {}

fn main() {}
