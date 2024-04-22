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
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
        // TODO: Make this an if let statement whose value is "Some" type

    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None; range];

        for i in 1..=range {
            optional_integers.push(Some(i as i8));
        }

        let mut cursor = range as i8;

        while let Some(integer) = optional_integers.pop() {

            cursor -= 1;
        }

        assert_eq!(0, 0);
    }
}
