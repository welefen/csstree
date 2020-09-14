#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
       enum A {
            String
        };
        let a = A::String;
        assert_eq!(2 + 2, 4);
    }
}
