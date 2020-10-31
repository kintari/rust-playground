#[allow(dead_code)]
#[allow(unused_variables)]


pub struct Scanner<'a> {
    text : &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>
}

#[derive(Debug)]
pub enum Token<'a> {
    Word(&'a str),
    EOF
}

impl <'a> Scanner<'a> {
    
    fn peek(&mut self) -> Option<&(usize,char)> {
        return self.chars.peek();
    }

    fn isalpha(ch: char) -> bool {
        return (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z');
    }

    pub fn new(text: &str) -> Scanner {
        return Scanner { text, chars: text.char_indices().peekable()  };
    }

    pub fn scan(&mut self) -> Option<Token> {
        while let Some(&(start,x)) = self.peek() {
            if x == ' ' {
                while let Some(_) = self.peek() {
                    self.chars.next();
                };                
            }
            else if Scanner::isalpha(x) {
                let end = loop {
                    if let Some((i,x)) = self.chars.next() {
                        if !Scanner::isalpha(x) {
                            break i;
                        }
                    }
                    else {
                        break self.text.len();
                    }
                };
                let foo = &self.text[start..end];
                return Some(Token::Word(foo))                
            }
        }
        return None;
    }

}

fn main() {
    let mut sc = Scanner::new("foo bar baz");

    if let Some(token) = sc.scan() {
        println!("{:?}", token);
    }

    if let Some(token) = sc.scan() {
        println!("{:?}", token);
    }

    if let Some(token) = sc.scan() {
        println!("{:?}", token);
    }
}
