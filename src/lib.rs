pub static WORDS : &'static str = include_str!("words.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_some_words() {
        assert!( WORDS.len() > 0 );
    }
}
