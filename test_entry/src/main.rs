mod module;

fn main() {
    println!("Hello, world!");
    println!("------------- Test Start -------------");
    vfscore::itor_trait();
    println!("------------- Test Task --------------");
    let sum = task::TASKS.iter().fold(0, |b, f| b + f());
    println!("len of function array: {}", task::TASKS.len());
    println!("sum of functions: {}", sum);
    println!("------------- Test End ---------------");
}
