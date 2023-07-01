// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a hint.

fn dbl(arg: i32) -> i32 {
    arg * 2
}

#[cfg(test)]
mod tests {
    use crate::dbl;
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(6, dbl(3));
    }
}
