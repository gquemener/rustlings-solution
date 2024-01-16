// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        while let Some(integer) = optional_integers.pop().flatten() {
            assert_eq!(integer, range);
            range -= 1;
        }
    }
}
