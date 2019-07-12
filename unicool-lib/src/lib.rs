use std::io::Write;
use std::string::FromUtf8Error;


pub fn convert_non_ascii_to_unicode(input: &str) -> Result<String, FromUtf8Error> {
    let bytes = input.as_bytes();

    let mut buffer: Vec<u8> = vec![];
    let writer: &mut dyn Write = &mut buffer;

    let mut pos = 0;
    while pos < input.len() -1 {
        match bytes[pos].is_ascii() {
            true => {
                writer.write(&bytes[pos..pos + 1]).unwrap();
            }
            false => {
                let unicode = format!("\\u{{{:x}{:x}}}", &bytes[pos], &bytes[pos+1]);
                writer.write(unicode.as_bytes()).unwrap();
            }
        };
        pos += 1;
    };
    String::from_utf8(buffer)
}


#[cfg(test)]
mod tests {
    use crate::convert_non_ascii_to_unicode;

    #[test]
    fn escape_unicode() {
        let input = "<p>Félicitations, votre compte Exacode à bien été créé !</p>\n\n<p>Pour finaliser votre inscription, il vous suffit de de cliquer sur le lien suivant afin de créer votre mot de passe :</p>\n\n<p><a href=\"{0}\">Créer un mot de passe</a></p>\n\n<p>Exacode</p>";
        let expected = "<p>F\\u{c3a9}\\u{a96c}licitations, votre compte Exacode \\u{c3a0}\\u{a020} bien \\u{c3a9}\\u{a974}t\\u{c3a9}\\u{a920} cr\\u{c3a9}\\u{a9c3}\\u{c3a9}\\u{a920} !</p>\n\n<p>Pour finaliser votre inscription, il vous suffit de de cliquer sur le lien suivant afin de cr\\u{c3a9}\\u{a965}er votre mot de passe :</p>\n\n<p><a href=\"{0}\">Cr\\u{c3a9}\\u{a965}er un mot de passe</a></p>\n\n<p>Exacode</p>";
        assert_eq!(convert_non_ascii_to_unicode(input).unwrap(), expected) ;
    }
}
