use linkme::distributed_slice;

#[distributed_slice(task::TASKS)]
fn test_func() {
    println!("test in other crate");
}

pub fn test() {

}