// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    reverse(&ltrim(&reverse(&ltrim(input))))
}

fn ltrim(input: &str) -> String {
    const SPACE: u8 = b' ';
    let mut first = 0;
    for (i, item) in input.bytes().enumerate() {
        if item != SPACE {
            first = i;
            break;
        }
    }

    input[first..].into()
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn compose_me(input: &str) -> String {
    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    String::from(input).replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
    }
}
