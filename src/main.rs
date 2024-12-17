
extern "C" {
    fn test() -> ();
    fn t2() -> ();
}

fn main() {
    // println!("Hello, world!");
    println!("before");
    unsafe {
        test();
        t2();
    }
    println!("after");
}
