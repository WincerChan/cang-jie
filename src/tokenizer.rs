use crate::{options::TokenizerOption, stream::CangjieTokenStream};
use jieba_rs::{Jieba, Token, TokenizeMode};
use log::trace;
use std::sync::Arc;
use tantivy::tokenizer::BoxTokenStream;

#[derive(Clone, Debug)]
pub struct CangJieTokenizer {
    /// Separation algorithm provider
    pub worker: Arc<Jieba>,
    /// Separation config
    pub option: TokenizerOption,
}

impl Default for CangJieTokenizer {
    fn default() -> Self {
        CangJieTokenizer {
            worker: Arc::new(Jieba::empty()),
            option: TokenizerOption::Default { hmm: false },
        }
    }
}

fn normalized_token<'a>(text: &str, tokens: Vec<Token<'a>>) -> Vec<(usize, usize, &'a str)> {
    let mut chars: Vec<(usize, char)> = text.char_indices().collect();
    // append zero char to last, case of last token' end is len(text).
    chars.push((text.len(), '\0'));
    tokens
        .into_iter()
        .map(|token| (chars[token.start].0, chars[token.end].0, token.word))
        .collect()
}

impl ::tantivy::tokenizer::Tokenizer for CangJieTokenizer {
    /// Cut text into tokens
    fn token_stream<'a>(&self, text: &'a str) -> BoxTokenStream<'a> {
        let result = match self.option {
            TokenizerOption::All => {
                let tokens = self.worker.tokenize(text, TokenizeMode::Default, true);
                normalized_token(text, tokens)
            }
            TokenizerOption::Default { hmm: use_hmm } => {
                let tokens = self.worker.tokenize(text, TokenizeMode::Default, use_hmm);
                normalized_token(text, tokens)
            }
            TokenizerOption::ForSearch { hmm: use_hmm } => {
                let tokens = self.worker.tokenize(text, TokenizeMode::Search, use_hmm);
                normalized_token(text, tokens)
            }
            TokenizerOption::Unicode => {
                let mut offset = 0usize;
                text.chars()
                    .map(|ch| {
                        let start = offset;
                        offset += ch.len_utf8();
                        (start, offset, &text[start..offset])
                    })
                    .collect()
            }
        };
        trace!("{:?}->{:?}", text, result);
        BoxTokenStream::from(CangjieTokenStream::new(result))
    }
}
