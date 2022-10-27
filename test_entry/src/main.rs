mod module;

fn main() {
    println!("Hello, world!");
    println!("------------- Test Start -------------");
    vfscore::itor_trait();
    // println!("------------- Test Ramfs -------------");
    // ramfs::itor_trait();     // 如果不加这一行 ramfs 可能就不会编译 就无法使用了 ...
                                // 该如何避免来自编译器的优化 如果不能避免那还要插入代码
                                // 非常不友好，会造成一大堆杂乱代码，也会增加耦合性。
                                // 目前测试可以使用一个 use 来代替这些代码

    // ramfs::test();              // 起到上面代码相同的作用，只是为了让编译器将那个模块加入编译
                                // TODO: 有没有一种其他的办法来做到这些？ 
    // println!("------------- Test Fatfs -------------");
    // fatfs::test();
    println!("------------- Test Task --------------");
    // let sum = task::test();
    let sum = task::TASKS.iter().fold(0, |b, f| b + f());
    println!("len of function array: {}", task::TASKS.len());
    println!("sum of functions: {}", sum);
    println!("------------- Test End ---------------");
}
