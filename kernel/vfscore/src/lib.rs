#![no_std]

pub struct TestFlag {
    pub short: char,
    pub name: &'static str
}

// inventory::submit! {
//     TestFlag { short: 'v', name: "verbose"}
// }

inventory::collect!(TestFlag);

pub fn itor_trait() {
    for flag in inventory::iter::<TestFlag> {
        // println!("-{}, --{}", flag.short, flag.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
