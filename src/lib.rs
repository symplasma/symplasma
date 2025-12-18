pub fn hello() -> &'static str {
    "Hello from symplasma!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(hello(), "Hello from symplasma!");
    }
}
