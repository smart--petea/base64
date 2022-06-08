const CHAR_64: [char; 64] = [ 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_'];

pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    let mut result = String::new();
    let input = input.as_ref();

    if input.len() == 0 {
        return result;
    }

    if input.len() >= 3 {
        for i in (0..=(input.len()-2)).step_by(3) {
            let i1 = input[i] >> 2;
            let i2 = ((input[i] & 0b00000011) << 4) | (input[i+1] >> 4);
            let i3 = ((input[i+1] & 0b00001111) << 2) | (input[i+2] >> 6);
            let i4 = input[i+2] & 0b00111111;

            result.push(CHAR_64[i1 as usize]);
            result.push(CHAR_64[i2 as usize]);
            result.push(CHAR_64[i3 as usize]);
            result.push(CHAR_64[i4 as usize]);
        }
    }

    match input.len() % 3 {
        1 => {
            println!("input.len={}", input.len());
            let i = input.len() - 1;
            let i1 = input[i] >> 2;
            let i2 = (input[i] & 0b00000011) << 4;

            result.push(CHAR_64[i1 as usize]);
            result.push(CHAR_64[i2 as usize]);

            result.push('=');
            result.push('=');
        }
        2 => {
            let i = input.len() - 2;
            let i1 = input[i] >> 2;
            let i2 = ((input[i] & 0b00000011) << 4) | (input[i+1] >> 4);
            let i3 = (input[i+1] & 0b00001111) << 2;

            result.push(CHAR_64[i1 as usize]);
            result.push(CHAR_64[i2 as usize]);
            result.push(CHAR_64[i3 as usize]);

            result.push('=');
        }
        _ => ()
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test() {
        let input = "";
        let expected_output = "".to_string();
        let real_output = encode(input);
        assert_eq!(real_output, expected_output);

        let input = "f";
        let expected_output = "Zg==".to_string();
        let real_output = encode(input);
        assert_eq!(real_output, expected_output);

        let input = "fo";
        let expected_output = "Zm8=".to_string();
        let real_output = encode(input);
        assert_eq!(real_output, expected_output);

        let input = "foo";
        let expected_output = "Zm9v".to_string();
        let real_output = encode(input);
        assert_eq!(real_output, expected_output);
    }
}
