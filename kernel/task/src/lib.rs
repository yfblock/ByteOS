use linkme::distributed_slice;

#[distributed_slice]
pub static TASKS: [fn ()] = [..];

pub fn test() {
    for f in TASKS {
        f();
    }
}