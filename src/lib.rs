use std::io::Write;
//todo forgiving-base64 decode. https://infra.spec.whatwg.org/#forgiving-base64-decode
//
//
//todo: pub const fn decode_allow_trailing_bits(self, allow: bool) -> Config for Config
//todo: check the structure of project at some resemblance with official one
#[derive(Debug, Clone, Copy)]
pub enum CharacterSet {
    Standard, //The standard character set (uses + and /). RFC3548
    UrlSafe, //The URL safe character set (uses - and _). RFC3548
    Crypt, //The crypt(3) character set (uses ./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz). Not standardized, but folk wisdom on the net asserts that this alphabet is what crypt uses.
    Bcrypt, //The bcrypt character set (uses ./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789).
    ImapMutf7, //The character set used in IMAP-modified UTF-7 (uses + and ,). See RFC 3501
    BinHex, // The character set used in BinHex 4.0 files. See BinHex 4.0 Definition
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    char_set: CharacterSet,
    pad: bool,
}

impl Config {
    pub const fn new(char_set: CharacterSet, pad: bool) -> Self {
        Config{
            char_set: char_set,
            pad: pad,
        }
    }

    pub const fn pad (self, pad: bool) -> Config {
        Self::new(self.char_set, pad)
    }
}

const URL_SAFE_U8: [u8; 64] = [ 'A' as u8, 'B' as u8, 'C' as u8, 'D' as u8, 'E' as u8, 'F' as u8, 'G' as u8, 'H' as u8, 'I' as u8, 'J' as u8, 'K' as u8, 'L' as u8, 'M' as u8, 'N' as u8, 'O' as u8, 'P' as u8, 'Q' as u8, 'R' as u8, 'S' as u8, 'T' as u8, 'U' as u8, 'V' as u8, 'W' as u8, 'X' as u8, 'Y' as u8, 'Z' as u8, 'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8, 'f' as u8, 'g' as u8, 'h' as u8, 'i' as u8, 'j' as u8, 'k' as u8, 'l' as u8, 'm' as u8, 'n' as u8, 'o' as u8, 'p' as u8, 'q' as u8, 'r' as u8, 's' as u8, 't' as u8, 'u' as u8, 'v' as u8, 'w' as u8, 'x' as u8, 'y' as u8, 'z' as u8, '0' as u8, '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8, '7' as u8, '8' as u8, '9' as u8, '-' as u8, '_' as u8];
const STANDARD_U8: [u8; 64] = [ 'A' as u8, 'B' as u8, 'C' as u8, 'D' as u8, 'E' as u8, 'F' as u8, 'G' as u8, 'H' as u8, 'I' as u8, 'J' as u8, 'K' as u8, 'L' as u8, 'M' as u8, 'N' as u8, 'O' as u8, 'P' as u8, 'Q' as u8, 'R' as u8, 'S' as u8, 'T' as u8, 'U' as u8, 'V' as u8, 'W' as u8, 'X' as u8, 'Y' as u8, 'Z' as u8, 'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8, 'f' as u8, 'g' as u8, 'h' as u8, 'i' as u8, 'j' as u8, 'k' as u8, 'l' as u8, 'm' as u8, 'n' as u8, 'o' as u8, 'p' as u8, 'q' as u8, 'r' as u8, 's' as u8, 't' as u8, 'u' as u8, 'v' as u8, 'w' as u8, 'x' as u8, 'y' as u8, 'z' as u8, '0' as u8, '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8, '7' as u8, '8' as u8, '9' as u8, '+' as u8, '/' as u8];
const CRYPT_U8: [u8; 64] = [ '.' as u8, '/' as u8, '0' as u8, '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8, '7' as u8, '8' as u8, '9' as u8, 'A' as u8, 'B' as u8, 'C' as u8, 'D' as u8, 'E' as u8, 'F' as u8, 'G' as u8, 'H' as u8, 'I' as u8, 'J' as u8, 'K' as u8, 'L' as u8, 'M' as u8, 'N' as u8, 'O' as u8, 'P' as u8, 'Q' as u8, 'R' as u8, 'S' as u8, 'T' as u8, 'U' as u8, 'V' as u8, 'W' as u8, 'X' as u8, 'Y' as u8, 'Z' as u8, 'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8, 'f' as u8, 'g' as u8, 'h' as u8, 'i' as u8, 'j' as u8, 'k' as u8, 'l' as u8, 'm' as u8, 'n' as u8, 'o' as u8, 'p' as u8, 'q' as u8, 'r' as u8, 's' as u8, 't' as u8, 'u' as u8, 'v' as u8, 'w' as u8, 'x' as u8, 'y' as u8, 'z' as u8];
const BCRYPT_U8: [u8; 64] = [ '.' as u8, '/' as u8, 'A' as u8, 'B' as u8, 'C' as u8, 'D' as u8, 'E' as u8, 'F' as u8, 'G' as u8, 'H' as u8, 'I' as u8, 'J' as u8, 'K' as u8, 'L' as u8, 'M' as u8, 'N' as u8, 'O' as u8, 'P' as u8, 'Q' as u8, 'R' as u8, 'S' as u8, 'T' as u8, 'U' as u8, 'V' as u8, 'W' as u8, 'X' as u8, 'Y' as u8, 'Z' as u8, 'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8, 'f' as u8, 'g' as u8, 'h' as u8, 'i' as u8, 'j' as u8, 'k' as u8, 'l' as u8, 'm' as u8, 'n' as u8, 'o' as u8, 'p' as u8, 'q' as u8, 'r' as u8, 's' as u8, 't' as u8, 'u' as u8, 'v' as u8, 'w' as u8, 'x' as u8, 'y' as u8, 'z' as u8, '0' as u8, '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8, '7' as u8, '8' as u8, '9' as u8];
const IMAP_MUTF7_U8: [u8; 64] = [ 'A' as u8, 'B' as u8, 'C' as u8, 'D' as u8, 'E' as u8, 'F' as u8, 'G' as u8, 'H' as u8, 'I' as u8, 'J' as u8, 'K' as u8, 'L' as u8, 'M' as u8, 'N' as u8, 'O' as u8, 'P' as u8, 'Q' as u8, 'R' as u8, 'S' as u8, 'T' as u8, 'U' as u8, 'V' as u8, 'W' as u8, 'X' as u8, 'Y' as u8, 'Z' as u8, 'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8, 'f' as u8, 'g' as u8, 'h' as u8, 'i' as u8, 'j' as u8, 'k' as u8, 'l' as u8, 'm' as u8, 'n' as u8, 'o' as u8, 'p' as u8, 'q' as u8, 'r' as u8, 's' as u8, 't' as u8, 'u' as u8, 'v' as u8, 'w' as u8, 'x' as u8, 'y' as u8, 'z' as u8, '0' as u8, '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8, '7' as u8, '8' as u8, '9' as u8, '+' as u8, ',' as u8];
const BIN_HEX_U8: [u8; 64] = ['!' as u8, '"' as u8, '#' as u8, '$' as u8, '%' as u8, '&' as u8, '\'' as u8, '(' as u8, ')' as u8, '*' as u8, '+' as u8, ',' as u8, '-' as u8, '0' as u8, '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8, '8' as u8, '9' as u8, '@' as u8, 'A' as u8, 'B' as u8, 'C' as u8, 'D' as u8, 'E' as u8, 'F' as u8, 'G' as u8, 'H' as u8, 'I' as u8, 'J' as u8, 'K' as u8, 'L' as u8, 'M' as u8, 'N' as u8, 'P' as u8, 'Q' as u8, 'R' as u8, 'S' as u8, 'T' as u8, 'U' as u8, 'V' as u8, 'X' as u8, 'Y' as u8, 'Z' as u8, '[' as u8, '`' as u8, 'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8, 'f' as u8, 'h' as u8, 'i' as u8, 'j' as u8, 'k' as u8, 'l' as u8, 'm' as u8, 'p' as u8, 'q' as u8, 'r' as u8];
const EQUAL_U8: [u8; 1] = ['=' as u8];

const STANDARD: Config = Config::new(CharacterSet::Standard, true);
const STANDARD_NO_PAD: Config = Config::new(CharacterSet::Standard, false);
const URL_SAFE: Config = Config::new(CharacterSet::UrlSafe, true);
const URL_SAFE_NO_PAD: Config = Config::new(CharacterSet::UrlSafe, false);

pub fn encode_config_slice<T: AsRef<[u8]>>(
    input: T,
    config: Config,
    mut writer: &mut [u8]
) -> usize {
    let input = input.as_ref();
    let mut written = 0usize;

    if input.len() == 0 {
        return written;
    }

    let u8s = match config.char_set {
        CharacterSet::UrlSafe => URL_SAFE_U8,
        CharacterSet::Standard => STANDARD_U8,
        CharacterSet::Crypt => CRYPT_U8,
        CharacterSet::Bcrypt => BCRYPT_U8,
        CharacterSet::ImapMutf7 => IMAP_MUTF7_U8,
        CharacterSet::BinHex => BIN_HEX_U8,
        _ => panic!("Not implemented")
    };

    if input.len() >= 3 {
        for i in (0..=(input.len()-3)).step_by(3) {
            let i1 = (input[i] >> 2) as usize;
            let i2 = (((input[i] & 0b00000011) << 4) | (input[i+1] >> 4)) as usize;
            let i3 = (((input[i+1] & 0b00001111) << 2) | (input[i+2] >> 6)) as usize;
            let i4 = (input[i+2] & 0b00111111) as usize;

            match writer.write(&u8s[i1..i1+1]) {
                Ok(w) => written += w,
                Err(_err) => return written
            };

            match writer.write(&u8s[i2..i2+1]) {
                Ok(w) => written += w,
                Err(_err) => return written
            }

            match writer.write(&u8s[i3..i3+1]) {
                Ok(w) => written += w,
                Err(_err) => return written
            }

            match writer.write(&u8s[i4..i4+1]) {
                Ok(w) => written += w,
                Err(_err) => return written
            }
        }
    }

    match input.len() % 3 {
        1 => {
            let i = input.len() - 1;
            let i1 = (input[i] >> 2) as usize;
            let i2 = ((input[i] & 0b00000011) << 4) as usize;

            match writer.write(&u8s[i1..i1+1]) {
                Ok(w) => written += w,
                Err(_err) => return written
            };

            match writer.write(&u8s[i2..i2+1]) {
                Ok(w) => written += w,
                Err(_err) => return written
            };

            if config.pad {
                match writer.write(&EQUAL_U8[..]) {
                    Ok(w) => written += w,
                    Err(_err) => return written
                };


                match writer.write(&EQUAL_U8[..]) {
                    Ok(w) => written += w,
                    Err(_err) => return written
                };
            }
        }
        2 => {
            let i = input.len() - 2;
            let i1 = (input[i] >> 2) as usize;
            let i2 = (((input[i] & 0b00000011) << 4) | (input[i+1] >> 4)) as usize;
            let i3 = ((input[i+1] & 0b00001111) << 2) as usize;

            match writer.write(&u8s[i1..i1+1]) {
                Ok(w) => written += w,
                Err(_err) => return written
            };

            match writer.write(&u8s[i2..i2+1]) {
                Ok(w) => written += w,
                Err(_err) => return written
            };

            match writer.write(&u8s[i3..i3+1]) {
                Ok(w) => written += w,
                Err(_err) => return written
            }

            if config.pad {
                match writer.write(&EQUAL_U8[..]) {
                    Ok(w) => written += w,
                    Err(_err) => return written
                };
            }
        }
        _ => ()
    };

    written
}

pub fn encode_config_buf<T: AsRef<[u8]>>(
    input: T,
    config: Config,
    result: &mut String,
) {
    let mut buf = [5u8; 5];
    //todo compute the size of the buf
    encode_config_slice(input, config, &mut buf[..]);

    let buf = std::str::from_utf8(&buf[..]).unwrap();
    result.push_str(buf)
}

pub fn encode_config<T: AsRef<[u8]>>(input: T, config: Config) -> String {
    let mut buf = String::new();
    encode_config_buf(input, config, &mut buf);

    buf
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

    #[test]
    fn encode_coding_no_pad_test() {
        let input = "";
        let expected_output = "".to_string();
        let real_output = encode_config(input, URL_SAFE_NO_PAD);
        assert_eq!(real_output, expected_output);

        let input = "f";
        let expected_output = "Zg".to_string();
        let real_output = encode_config(input, URL_SAFE_NO_PAD);
        assert_eq!(real_output, expected_output);

        let input = "fo";
        let expected_output = "Zm8".to_string();
        let real_output = encode_config(input, URL_SAFE_NO_PAD);
        assert_eq!(real_output, expected_output);

        let input = "foo";
        let expected_output = "Zm9v".to_string();
        let real_output = encode_config(input, URL_SAFE_NO_PAD);
        assert_eq!(real_output, expected_output);

        let input = "foob";
        let expected_output = "Zm9vYg".to_string();
        let real_output = encode_config(input, URL_SAFE_NO_PAD);
        assert_eq!(real_output, expected_output);

        let input = "fooba";
        let expected_output = "Zm9vYmE".to_string();
        let real_output = encode_config(input, URL_SAFE_NO_PAD);
        assert_eq!(real_output, expected_output);

        let input = "foobar";
        let expected_output = "Zm9vYmFy".to_string();
        let real_output = encode_config(input, URL_SAFE_NO_PAD);
        assert_eq!(real_output, expected_output);
    }

    #[test]
    fn encode_config_slice_test() {
        let input = "";
        let mut real_output = [0u8; 0];
        let sz = encode_config_slice(input, URL_SAFE_NO_PAD, &mut real_output[..]);
        assert_eq!(sz, 0);

        let input = "f";
        let mut real_output = [0u8; 2];
        let sz = encode_config_slice(input, URL_SAFE_NO_PAD, &mut real_output[..]);
        assert_eq!(sz, 2);
        assert_eq!(real_output[0] as char, 'Z');
        assert_eq!(real_output[1] as char, 'g');

        let input = "f";
        let mut real_output = [0u8; 4];
        let sz = encode_config_slice(input, URL_SAFE, &mut real_output[..]);
        assert_eq!(sz, 4);
        assert_eq!(real_output[0] as char, 'Z');
        assert_eq!(real_output[1] as char, 'g');
        assert_eq!(real_output[2] as char, '=');
        assert_eq!(real_output[3] as char, '=');

        let input = "fo";
        let mut real_output = [0u8; 4];
        let sz = encode_config_slice(input, URL_SAFE, &mut real_output[..]);
        assert_eq!(sz, 4);
        assert_eq!(real_output[0] as char, 'Z');
        assert_eq!(real_output[1] as char, 'm');
        assert_eq!(real_output[2] as char, '8');
        assert_eq!(real_output[3] as char, '=');

        let input = "foo";
        let mut real_output = [0u8; 4];
        let sz = encode_config_slice(input, URL_SAFE, &mut real_output[..]);
        assert_eq!(sz, 4);
        assert_eq!(real_output[0] as char, 'Z');
        assert_eq!(real_output[1] as char, 'm');
        assert_eq!(real_output[2] as char, '9');
        assert_eq!(real_output[3] as char, 'v');
    }
}
