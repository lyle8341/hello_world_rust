pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use std::arch::x86_64;

    #[test]
    fn iterator_demo() {
        let v = vec![1, 2, 3];
        let mut it = v.iter();

        let r = it.next();
        // assert_eq!(it.next(), Some(1));
        // assert_eq!(it.next(), Some(2));
        // assert_eq!(it.next(), Some(3));
    }
}
