pub enum CharacterSet {
    Standard, //The standard character set (uses + and /). RFC3548
    UrlSafe, //The URL safe character set (uses - and _). RFC3548
    Crypt, //todo
    Bcrypt, //todo
    ImapMutf7, //todo
    BinHex, //todo
}

pub struct Config {
    char_set: CharacterSet
}

impl Config {
    pub const fn new(char_set: CharacterSet, pad: bool) -> Self {
        Config{
            char_set: char_set
        }
    }
}

const URL_SAFE_CHARS: [char; 64] = [ 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_'];


const STANDARD: Config = Config::new(CharacterSet::Standard, true);
const STANDARD_NO_PAD: Config = Config::new(CharacterSet::Standard, false);
const URL_SAFE: Config = Config::new(CharacterSet::UrlSafe, true);
const URL_SAFE_NO_PAD: Config = Config::new(CharacterSet::UrlSafe, false);

pub fn encode_config<T: AsRef<[u8]>>(input: T, config: Config) -> String {
    let mut result = String::new();
    let input = input.as_ref();

    if input.len() == 0 {
        return result;
    }

    let chars = match config.char_set {
        CharacterSet::UrlSafe => URL_SAFE_CHARS,
        _ => panic!("Not implemented")
    };

    if input.len() >= 3 {
        for i in (0..=(input.len()-3)).step_by(3) {
            let i1 = input[i] >> 2;
            let i2 = ((input[i] & 0b00000011) << 4) | (input[i+1] >> 4);
            let i3 = ((input[i+1] & 0b00001111) << 2) | (input[i+2] >> 6);
            let i4 = input[i+2] & 0b00111111;

            result.push(chars[i1 as usize]);
            result.push(chars[i2 as usize]);
            result.push(chars[i3 as usize]);
            result.push(chars[i4 as usize]);
        }
    }

    match input.len() % 3 {
        1 => {
            let i = input.len() - 1;
            let i1 = input[i] >> 2;
            let i2 = (input[i] & 0b00000011) << 4;

            result.push(chars[i1 as usize]);
            result.push(chars[i2 as usize]);

            result.push('=');
            result.push('=');
        }
        2 => {
            let i = input.len() - 2;
            let i1 = input[i] >> 2;
            let i2 = ((input[i] & 0b00000011) << 4) | (input[i+1] >> 4);
            let i3 = (input[i+1] & 0b00001111) << 2;

            result.push(chars[i1 as usize]);
            result.push(chars[i2 as usize]);
            result.push(chars[i3 as usize]);

            result.push('=');
        }
        _ => ()
    }

    result
}

pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    encode_config(input, STANDARD)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_coding_test() {
        let input = "";
        let expected_output = "".to_string();
        let real_output = encode_config(input, URL_SAFE);
        assert_eq!(real_output, expected_output);

        let input = "f";
        let expected_output = "Zg==".to_string();
        let real_output = encode_config(input, URL_SAFE);
        assert_eq!(real_output, expected_output);

        let input = "fo";
        let expected_output = "Zm8=".to_string();
        let real_output = encode_config(input, URL_SAFE);
        assert_eq!(real_output, expected_output);

        let input = "foo";
        let expected_output = "Zm9v".to_string();
        let real_output = encode_config(input, URL_SAFE);
        assert_eq!(real_output, expected_output);

        let input = "foob";
        let expected_output = "Zm9vYg==".to_string();
        let real_output = encode_config(input, URL_SAFE);
        assert_eq!(real_output, expected_output);

        let input = "fooba";
        let expected_output = "Zm9vYmE=".to_string();
        let real_output = encode_config(input, URL_SAFE);
        assert_eq!(real_output, expected_output);

        let input = "foobar";
        let expected_output = "Zm9vYmFy".to_string();
        let real_output = encode_config(input, URL_SAFE);
        assert_eq!(real_output, expected_output);
    }
}
