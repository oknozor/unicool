use std::io::Write;
use std::string::FromUtf8Error;

pub fn convert_non_ascii_to_unicode(input: &str) -> Result<String, FromUtf8Error> {
    let bytes = input.as_bytes();

    let mut buffer: Vec<u8> = vec![];
    let writer: &mut dyn Write = &mut buffer;

    let mut pos = 0;
    while pos < input.len() - 1 {
        match bytes[pos].is_ascii() {
            true => {
                writer.write(&bytes[pos..pos + 1]).unwrap();
            }
            false => {
                let unicode = format!("\\u{:x}{:x}", &bytes[pos], &bytes[pos + 1]);
                writer.write(unicode.as_bytes()).unwrap();
            }
        };
        pos += 1;
    }
    String::from_utf8(buffer)
}

#[cfg(test)]
mod tests {
    use crate::convert_non_ascii_to_unicode;

    #[test]
    fn single_characters() {
        assert_eq!(convert_non_ascii_to_unicode("à").unwrap(), "\\uc3a0");
        assert_eq!(convert_non_ascii_to_unicode("é").unwrap(), "\\uc3a9");
        assert_eq!(convert_non_ascii_to_unicode("û").unwrap(), "\\uc3bb");
        assert_eq!(convert_non_ascii_to_unicode("ç").unwrap(), "\\uc3a7");
        assert_eq!(convert_non_ascii_to_unicode("Ç").unwrap(), "\\uc387");
    }

    #[test]
    fn test_complete_sentence() {
        assert_eq!(
            convert_non_ascii_to_unicode("Ça, ça c'est vraiment toi!").unwrap(),
            "\\uc387\\u8761a, \\uc3a7\\ua761a c\'est vraiment toi"
        );
    }
}
