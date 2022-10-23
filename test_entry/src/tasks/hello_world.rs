use linkme::distributed_slice;

use task::TASKS;

#[distributed_slice(TASKS)]
fn blink() {
    println!("Hello World!");
}