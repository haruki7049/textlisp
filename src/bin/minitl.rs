fn main() {
    println!("Hello, world!");
    // (i value)
    // (k value value_second)
    // (s value value_second value_third)
}

struct Parser;

#[derive(Default)]
struct AbstractSyntaxTree;

#[derive(Debug, Default, PartialEq)]
struct ConcreteSyntaxTree {
    expr: Vec<ConcreteSyntaxToken>,
}

#[derive(Debug, PartialEq)]
enum ConcreteSyntaxToken {
    Symbol(Symbol),
    Name(String),
}

#[derive(Debug, PartialEq)]
enum Symbol {
    Space,
    LeftParenthesis,
    RightParenthesis,
}

impl Parser {
    fn parse<T>(t: T) -> AbstractSyntaxTree
    where
        T: Into<String>,
    {
        let cst: ConcreteSyntaxTree = Self::tokenize(t);

        todo!()
    }

    fn tokenize<T>(t: T) -> ConcreteSyntaxTree
    where
        T: Into<String>,
    {
        let characters: Vec<char> = t.into().chars().collect();
        let mut reversed_characters: Vec<char> = characters.into_iter().rev().collect();
        let mut expr: Vec<ConcreteSyntaxToken> = Vec::new();
        let mut string_mode_is_on: bool = false; // Whether it parse source code as string or constant word
        let mut string_cache: String = String::new();

        while let Some(c) = reversed_characters.pop() {
            if string_mode_is_on {
                match c {
                    '(' => panic!("Unexpected token: {:?}", c),
                    ')' => {
                        expr.push(ConcreteSyntaxToken::Name(string_cache.clone()));
                        expr.push(ConcreteSyntaxToken::Symbol(Symbol::RightParenthesis));
                        string_mode_is_on = false;

                        // Clean up string_cache
                        string_cache.clear();
                    }
                    ' ' => {
                        expr.push(ConcreteSyntaxToken::Name(string_cache.clone()));
                        expr.push(ConcreteSyntaxToken::Symbol(Symbol::Space));
                        string_mode_is_on = false;

                        // Clean up string_cache
                        string_cache.clear();
                    }
                    _ => {
                        string_cache.push(c);
                    }
                }
            } else {
                // String mode off
                match c {
                    '(' => expr.push(ConcreteSyntaxToken::Symbol(Symbol::LeftParenthesis)),
                    ')' => expr.push(ConcreteSyntaxToken::Symbol(Symbol::RightParenthesis)),
                    ' ' => expr.push(ConcreteSyntaxToken::Symbol(Symbol::Space)),
                    'i' => expr.push(ConcreteSyntaxToken::Name(String::from('i'))),
                    'k' => expr.push(ConcreteSyntaxToken::Name(String::from('k'))),
                    's' => expr.push(ConcreteSyntaxToken::Name(String::from('s'))),
                    _ => {
                        string_mode_is_on = true;
                        string_cache.push(c);
                    }
                }
            }
        }

        ConcreteSyntaxTree { expr: expr }
    }
}

#[cfg(test)]
mod tests {
    mod parse {
        use crate::AbstractSyntaxTree;
        use crate::Parser;

        #[test]
        fn parse() {
            let code: &str = "(i value)";
            let _ast: AbstractSyntaxTree = Parser::parse(code);
            todo!();
        }
    }

    mod tokenize {
        use crate::ConcreteSyntaxToken;
        use crate::ConcreteSyntaxTree;
        use crate::Parser;
        use crate::Symbol;

        #[test]
        fn tokenize_with_more_spaces() {
            let code: &str = "( i ( i value ) )";
            let token: ConcreteSyntaxTree = Parser::tokenize(code);
            assert_eq!(
                token,
                ConcreteSyntaxTree {
                    expr: vec![
                        ConcreteSyntaxToken::Symbol(Symbol::LeftParenthesis),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from('i')),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Symbol(Symbol::LeftParenthesis),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from('i')),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value")),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Symbol(Symbol::RightParenthesis),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Symbol(Symbol::RightParenthesis),
                    ],
                }
            );
        }

        #[test]
        fn tokenize_recurse() {
            let code: &str = "(i (i value))";
            let token: ConcreteSyntaxTree = Parser::tokenize(code);
            assert_eq!(
                token,
                ConcreteSyntaxTree {
                    expr: vec![
                        ConcreteSyntaxToken::Symbol(Symbol::LeftParenthesis),
                        ConcreteSyntaxToken::Name(String::from('i')),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Symbol(Symbol::LeftParenthesis),
                        ConcreteSyntaxToken::Name(String::from('i')),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value")),
                        ConcreteSyntaxToken::Symbol(Symbol::RightParenthesis),
                        ConcreteSyntaxToken::Symbol(Symbol::RightParenthesis),
                    ],
                }
            );

            let code: &str = "(k value (k value value_second))";
            let token: ConcreteSyntaxTree = Parser::tokenize(code);
            assert_eq!(
                token,
                ConcreteSyntaxTree {
                    expr: vec![
                        ConcreteSyntaxToken::Symbol(Symbol::LeftParenthesis),
                        ConcreteSyntaxToken::Name(String::from('k')),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value")),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Symbol(Symbol::LeftParenthesis),
                        ConcreteSyntaxToken::Name(String::from('k')),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value")),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value_second")),
                        ConcreteSyntaxToken::Symbol(Symbol::RightParenthesis),
                        ConcreteSyntaxToken::Symbol(Symbol::RightParenthesis),
                    ],
                }
            );

            let code: &str = "(s value value_second (s value value_second value_third))";
            let token: ConcreteSyntaxTree = Parser::tokenize(code);
            assert_eq!(
                token,
                ConcreteSyntaxTree {
                    expr: vec![
                        ConcreteSyntaxToken::Symbol(Symbol::LeftParenthesis),
                        ConcreteSyntaxToken::Name(String::from('s')),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value")),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value_second")),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Symbol(Symbol::LeftParenthesis),
                        ConcreteSyntaxToken::Name(String::from('s')),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value")),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value_second")),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value_third")),
                        ConcreteSyntaxToken::Symbol(Symbol::RightParenthesis),
                        ConcreteSyntaxToken::Symbol(Symbol::RightParenthesis),
                    ],
                }
            );
        }

        #[test]
        fn tokenize() {
            let code: &str = "(i value)";
            let token: ConcreteSyntaxTree = Parser::tokenize(code);
            assert_eq!(
                token,
                ConcreteSyntaxTree {
                    expr: vec![
                        ConcreteSyntaxToken::Symbol(Symbol::LeftParenthesis),
                        ConcreteSyntaxToken::Name(String::from('i')),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value")),
                        ConcreteSyntaxToken::Symbol(Symbol::RightParenthesis),
                    ],
                }
            );

            let code: &str = "(k value value_second)";
            let token: ConcreteSyntaxTree = Parser::tokenize(code);
            assert_eq!(
                token,
                ConcreteSyntaxTree {
                    expr: vec![
                        ConcreteSyntaxToken::Symbol(Symbol::LeftParenthesis),
                        ConcreteSyntaxToken::Name(String::from('k')),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value")),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value_second")),
                        ConcreteSyntaxToken::Symbol(Symbol::RightParenthesis),
                    ],
                }
            );

            let code: &str = "(s value value_second value_third)";
            let token: ConcreteSyntaxTree = Parser::tokenize(code);
            assert_eq!(
                token,
                ConcreteSyntaxTree {
                    expr: vec![
                        ConcreteSyntaxToken::Symbol(Symbol::LeftParenthesis),
                        ConcreteSyntaxToken::Name(String::from('s')),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value")),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value_second")),
                        ConcreteSyntaxToken::Symbol(Symbol::Space),
                        ConcreteSyntaxToken::Name(String::from("value_third")),
                        ConcreteSyntaxToken::Symbol(Symbol::RightParenthesis),
                    ],
                }
            );
        }
    }
}
