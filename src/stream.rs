use tantivy::tokenizer::Token;

#[derive(Debug)]
pub struct CangjieTokenStream<'a> {
    result: Vec<(usize, usize, &'a str)>,
    index: usize,
    token: Token,
}

impl<'a> CangjieTokenStream<'a> {
    pub fn new(result: Vec<(usize, usize, &'a str)>) -> Self {
        CangjieTokenStream {
            result,
            index: 0,
            token: Token::default(),
        }
    }
}

impl<'a> ::tantivy::tokenizer::TokenStream for CangjieTokenStream<'a> {
    fn advance(&mut self) -> bool {
        if self.index < self.result.len() {
            let token = &self.result[self.index];

            self.token = Token {
                offset_from: token.0,
                offset_to: token.1,
                position: token.0,
                text: token.2.to_lowercase(),
                position_length: token.1 - token.0,
            };

            self.index += 1;
            true
        } else {
            false
        }
    }

    fn token(&self) -> &::tantivy::tokenizer::Token {
        &self.token
    }

    fn token_mut(&mut self) -> &mut ::tantivy::tokenizer::Token {
        &mut self.token
    }
}
