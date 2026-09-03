#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::parser;
    use crate::compiler;

    #[test]
    fn test_lexer_integers() {
        let tokens = lexer::lex("42 1000 0").unwrap();
        assert_eq!(tokens.len(), 4); // 3 integers + EOF
    }

    #[test]
    fn test_lexer_keywords() {
        let tokens = lexer::lex("fn let mut if else").unwrap();
        assert!(tokens.iter().any(|t| matches!(t.kind, crate::lexer::token::TokenKind::Fn)));
        assert!(tokens.iter().any(|t| matches!(t.kind, crate::lexer::token::TokenKind::Let)));
    }

    #[test]
    fn test_lexer_operators() {
        let tokens = lexer::lex("+ - * / == != && ||").unwrap();
        assert!(tokens.len() > 0);
    }

    #[test]
    fn test_parser_simple_function() {
        let tokens = lexer::lex("fn add(a:Int,b:Int)=>a+b").unwrap();
        let program = parser::parse(tokens).unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_parser_variable() {
        let tokens = lexer::lex("let x => 10").unwrap();
        let program = parser::parse(tokens).unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_parser_if_statement() {
        let tokens = lexer::lex("if x > 10 { print(\"yes\") }").unwrap();
        let program = parser::parse(tokens).unwrap();
        assert!(program.items.len() > 0);
    }

    #[test]
    fn test_runtime_arithmetic() {
        let tokens = lexer::lex("fn main()=>print(2+3)").unwrap();
        let program = parser::parse(tokens).unwrap();
        // Should execute without error
        let result = compiler::compile_and_run(program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_runtime_factorial() {
        let source = "fn factorial(n:Int)=>n<=1?1:n*factorial(n-1); fn main()=>print(factorial(5))";
        let tokens = lexer::lex(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let result = compiler::compile_and_run(program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_runtime_conditional() {
        let tokens = lexer::lex("fn main()=>print(10>5?1:0)").unwrap();
        let program = parser::parse(tokens).unwrap();
        let result = compiler::compile_and_run(program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_array() {
        let tokens = lexer::lex("let arr => [1,2,3]").unwrap();
        let program = parser::parse(tokens).unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_parser_struct() {
        let tokens = lexer::lex("struct Point { x:Int y:Int }").unwrap();
        let program = parser::parse(tokens).unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_parser_enum() {
        let tokens = lexer::lex("enum Color { Red Green Blue }").unwrap();
        let program = parser::parse(tokens).unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_one_liner_hello() {
        let source = "#require <io>; fn main()=>print(\"Hello XP\")";
        let tokens = lexer::lex(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        assert!(program.items.len() > 0);
    }

    #[test]
    fn test_lexer_strings() {
        let tokens = lexer::lex(r#""hello world""#).unwrap();
        let string_token = tokens.iter().find(|t| matches!(t.kind, crate::lexer::token::TokenKind::String));
        assert!(string_token.is_some());
    }

    #[test]
    fn test_lexer_comments() {
        let source = "42 // comment\n100";
        let tokens = lexer::lex(source).unwrap();
        // Comments should be skipped
        assert!(tokens.iter().any(|t| matches!(t.kind, crate::lexer::token::TokenKind::Integer)));
    }

    #[test]
    fn test_parser_multiple_statements() {
        let source = "let x => 10; let y => 20; fn main()=>print(x)";
        let tokens = lexer::lex(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        assert!(program.items.len() >= 2);
    }

    #[test]
    fn test_runtime_variables() {
        let source = "fn test()=>10; fn main()=>print(test())";
        let tokens = lexer::lex(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let result = compiler::compile_and_run(program);
        assert!(result.is_ok());
    }
}
