use std::collections::HashMap;
use std::path::Path;
pub use common::*;

enum ParseStatus {
    Out,
    In,
    PrefixMatchOne,
    SuffixMatchOne,
}

pub enum TokenType {
    HTML,
    VAR,
    FOREACH,
}

pub struct Template {
    pub dir: String,
    pub name: String,
    pub suffix: String,
    pub tokens: Vec<(TokenType, String)>,
    pub vars: HashMap<String, String>,
}

// todo : 1. {} in var, 2.err when not ParseStatus::Out in the end.
#[allow(unused_variables)]
fn file_to_tokens(path: &Path) -> Vec<(TokenType, String)> {
    let mut token = String::new();
    let mut tokens = Vec::new();
    let mut parse_status = ParseStatus::Out;

    let characters: Vec<(usize, char)> = cat(path).char_indices().collect();
    for character in characters {
        let (unused_pos, utf8_char) = character;
        match parse_status {
            ParseStatus::Out => {
                if utf8_char == '{' {
                    parse_status = ParseStatus::PrefixMatchOne;
                } else {
                    token.push(utf8_char);
                }
            },
            ParseStatus::In => { 
                if utf8_char == '}' {
                    parse_status = ParseStatus::SuffixMatchOne;
                } else if utf8_char != ' ' {
                    token.push(utf8_char);
                }
            },
            ParseStatus::PrefixMatchOne => {
                if utf8_char == '{' {
                    parse_status = ParseStatus::In;
                    tokens.push((TokenType::HTML, token));
                    token = String::new();
                } else {
                    parse_status = ParseStatus::Out;
                    token.push('{'); 
                    token.push(utf8_char); 
                }
            },
            ParseStatus::SuffixMatchOne => {
                if utf8_char == '}' {
                    parse_status = ParseStatus::Out;
                    tokens.push((TokenType::VAR, token));
                    token = String::new();
                } else {
                    parse_status = ParseStatus::Out;
                    token.push('}');
                    token.push(utf8_char);
                }
            },
        }
    }
    tokens.push((TokenType::HTML, token));
    return tokens;
}

impl Template {
    pub fn new() -> Template {
        Template { 
            dir: "template".to_string(),
            name: String::new(),
            suffix: "html".to_string(),
            tokens: Vec::new(), 
            vars: HashMap::new(),
        }
    }

    pub fn set_template(&mut self, path: &str) {
        self.name = path.to_string();
        let mut path_string = String::new();
        path_string.push_str(&self.dir);
        path_string.push_str("/");
        path_string.push_str(path);
        path_string.push_str(".");
        path_string.push_str(&self.suffix);
        self.tokens = file_to_tokens(Path::new(&path_string));
    }

    pub fn assign(&mut self, var: &str, data: String) {
        self.vars.insert(var.to_string(), data);
    }

    pub fn render(&mut self) -> String {
        let mut template_content = String::new();
        for token in self.tokens.iter() {
            let &(ref token_type, ref var) = token;
            match *token_type {
                TokenType::HTML => {
                    template_content.push_str(var);
                },
                TokenType::VAR => {
                    let c = self.vars.get("var").unwrap();
                    template_content.push_str(c);
                },
                TokenType::FOREACH => {},
            }
        }
        return template_content;
    }
}
