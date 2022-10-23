// use linkme::distributed_slice;
use header::linkme as linkme;

#[header::linkme::distributed_slice(task::TASKS)]
#[linkme(crate = linkme)]
fn test_func() {
    println!("test in other crate");
}

pub fn test() {

}