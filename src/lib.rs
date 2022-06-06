pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    let mut result = String::new();
    let input = input.as_ref();

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
