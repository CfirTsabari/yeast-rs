use std::{thread::sleep, time::Duration};
use yeast_rs::yeast;
/// possible output:
/// '''ignore
/// AHMmBX2sBHEqC1UnAk
/// AHMmBX2sBHEqC1UnAk_A.
/// AHMmBX2sBHEqC1UnB.
/// AHMmBX2sBHEqC1UnB._A.
/// '''
fn main() {
    println!("{}", yeast());
    println!("{}", yeast());
    sleep(Duration::from_millis(1));
    println!("{}", yeast());
    println!("{}", yeast());
}
