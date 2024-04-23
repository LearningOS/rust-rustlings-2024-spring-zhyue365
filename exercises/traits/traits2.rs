trait AppendBar {
    fn append_bar(self) -> Self;
}

// Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        self.into_iter().map(|s| s + "Bar").collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();

        assert_eq!(foo.pop().unwrap(), String::from("FooBar"));

    }
}