#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use iso::Iso;
        assert!(Iso::open("data/test.iso").is_ok());
    }
}