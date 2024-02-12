static EFFECTS_PATH: &str = "effects";

pub mod prelude {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_test() {
        assert_eq!(EFFECTS_PATH, "effects");
    }
}
