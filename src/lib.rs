pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    let mut result = String::new();
    let input = input.as_ref();

    for i in (0..(input.len()-3)).step_by(3) {
        let i1 = input[i] >> 2;
        let i2 = ((input[i] & 0b00000011) << 4) | (input[i+1] >> 4);
        let i3 = ((input[i+1] & 0b00001111) << 2) | (input[i+2] >> 6);
        let i4 = input[i+2] & 0b00111111;

        println!("[b1 b2 b3] = [{} {} {}] [i1 i2 i3 i4]=[{} {} {} {}]", input[i], input[i+1], input[i+2], i1, i2, i3, i4);
    }

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
