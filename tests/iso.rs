#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use iso::Iso;

        let iso = Iso::open("data/test.iso");
        assert!(iso.is_ok());
        println!("{:#?}", iso)
    }
}