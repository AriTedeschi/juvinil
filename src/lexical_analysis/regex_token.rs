use super::token::TokenType;

pub struct RegexToken {
    pub regex_template: &'static str,
    pub token_type: TokenType,
}

impl RegexToken {
    pub const fn new(regex_template: &'static str, token_type: TokenType) -> Self {
        Self {
            regex_template,
            token_type,
        }
    }
}

pub const REGEX_TOKEN_MAP: [RegexToken; 3] = [
    RegexToken::new(r#"^[+-]?([0-9]*[.])?[0-9]+$"#, TokenType::NUMBER),
    RegexToken::new(r#"^"[^"]*"$"#, TokenType::STRING),
    RegexToken::new(r#"^[a-zA-Z_]+$"#, TokenType::ID),
];
