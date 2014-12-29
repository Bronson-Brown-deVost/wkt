pub enum Token {
    Comma,
    Number(f64),
    ParenClose,
    ParenOpen,
    Word(String),
}

pub fn tokenize(input: &str) -> Tokenizer {
    Tokenizer {text: String::from_str(input)}
}

/*
fn is_whitespace(char) -> bool {
}
*/

struct Tokenizer {
    text: String
}

impl Iterator<Token> for Tokenizer {
    fn next(&mut self) -> Option<Token> {
        let popped_char = self.pop_char();
        if popped_char.is_none() {
            return None
        }
        let next_char = popped_char.unwrap();

        match next_char {
            '\0' => None,
            '(' => Some(Token::ParenOpen),
            ')' => Some(Token::ParenClose),
            ',' => Some(Token::Comma),
            '\n' | '\r' | '\t' | ' ' => self.next(),
            c if c.is_numeric() => {
                let x: f64 = from_str(c.to_string().as_slice()).unwrap();
                Some(Token::Number(x))
            }
            c => {
                let word = c.to_string() + self.finish_word().as_slice();
                Some(Token::Word(word))
            }
        }
    }
}

impl Tokenizer {
    fn pop_char(&mut self) -> Option<char> {
        if self.text.is_empty() {
            None
        } else {
            self.text.remove(0)
        }
    }

    fn finish_word(&mut self) -> String {
        let popped_char = self.pop_char();
        if popped_char.is_none() {
            return "".to_string()
        }
        let next_char = popped_char.unwrap();

        match next_char {
            '\0' | '(' | ')' | ',' => {
                self.text.insert(0, next_char);
                "".to_string()
            }
            '\n' | '\r' | '\t' | ' ' => "".to_string(),
            _ => next_char.to_string() + self.finish_word().as_slice(),
        }
    }

    /*
    fn finish_number(&mut self) -> String {
    }
    */
}

#[test]
fn test_tokenizer() {
    let test_str = "hello";
    match tokenize(test_str).next().unwrap() {
        Token::Word(n) => assert_eq!(n, test_str.to_string()),
        _ => panic!("fail")
    }
}
