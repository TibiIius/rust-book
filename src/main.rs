// unsafe traits hold methods that may contain unsafe code
unsafe trait IAmUnsafe {}

unsafe impl IAmUnsafe for i32 {}

fn main() {}
