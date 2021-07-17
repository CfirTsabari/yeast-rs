use std::{thread::sleep, time::Duration};
use yeast_rs::yeast;
/// possible output:
/// '''ignore
/// NgqS4Rc
/// NgqS4Rc.0
/// NgqS4Rd
/// NgqS4Rd.0
/// '''
fn main() {
    println!("{}", yeast());
    println!("{}", yeast());
    sleep(Duration::from_millis(1));
    println!("{}", yeast());
    println!("{}", yeast());
}
