use vfscore::TestFlag;

inventory::submit! {
    TestFlag { short: 'r', name: "ramfs"}
}

#[allow(dead_code)]
pub fn itor_trait() {
    for flag in inventory::iter::<TestFlag> {
        println!("-{}, --{}", flag.short, flag.name);
    }
}

pub fn test() {
    println!("Hello World!");
}