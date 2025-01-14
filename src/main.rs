use std::fs;

use juvinil::{error::JuvinilResult, lexical_analysis::lex, syntax_analysis::parser::Parser};

fn main() {
    tracing_subscriber::fmt().pretty().init();

    if let Err(err) = run("test_inputs/test.jv") {
        tracing::error!("{}", err);
        std::process::exit(1);
    }
}

fn run(file_path: &str) -> JuvinilResult<()> {
    tracing::info!("--------READING INPUT--------");
    let file = fs::read_to_string(file_path)?;
    tracing::info!("Successfully read contents of file {}", file_path);

    tracing::info!("--------LEXICAL ANALYSIS--------");
    let tokens = lex::tokenize(file)?;
    tracing::info!("Successfully tokenized file contents");

    tracing::info!("--------SYNTAX ANALYSIS--------");
    let mut parser = Parser::new(tokens)?;
    parser.parse()?;
    tracing::info!("Successfully parsed file contents");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_operators_ok() {
        let file_content = fs::read_to_string("test_inputs/operators.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");

        let mut parser = Parser::new(tokens.unwrap()).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Should be OK");
    }

    #[test]
    fn types_ok() {
        let file_content = fs::read_to_string("test_inputs/types.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");

        let mut parser = Parser::new(tokens.unwrap()).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Should be OK");
    }

    #[test]
    fn strings_ok() {
        let file_content = fs::read_to_string("test_inputs/strings.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");

        let mut parser = Parser::new(tokens.unwrap()).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Should be OK");
    }

    #[test]
    fn if_ok() {
        let file_content = fs::read_to_string("test_inputs/if.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");

        let mut parser = Parser::new(tokens.unwrap()).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Should be OK");
    }

    #[test]
    fn for_ok() {
        let file_content = fs::read_to_string("test_inputs/for.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");

        let mut parser = Parser::new(tokens.unwrap()).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Should be OK");
    }
}
