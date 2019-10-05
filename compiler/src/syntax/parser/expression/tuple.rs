//!
//! The tuple-like expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::syntax::ExpressionParser;
use crate::syntax::TupleExpressionBuilder;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    ParenthesisLeft,
    ExpressionOrParenthesisRight,
    CommaOrParenthesisRight,
}

impl Default for State {
    fn default() -> Self {
        State::ParenthesisLeft
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: TupleExpressionBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
        loop {
            match self.state {
                State::ParenthesisLeft => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            self.state = State::ExpressionOrParenthesisRight;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["("],
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::ExpressionOrParenthesisRight => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            return Ok(self.builder.finish());
                        }
                        Some(Ok(..)) => {
                            let expression = ExpressionParser::default().parse(stream.clone())?;
                            self.builder.push_expression(expression);
                            self.state = State::CommaOrParenthesisRight;
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::CommaOrParenthesisRight => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        })) => {
                            self.builder.set_comma();
                            self.state = State::ExpressionOrParenthesisRight;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        })) => return Ok(self.builder.finish()),
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![",", ")"],
                                lexeme,
                            )));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical;
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::Literal;
    use crate::syntax::TupleExpression;

    #[test]
    fn ok_unit() {
        let input = r#"()"#;

        let expected = Ok(Expression::new(
            Location::new(1, 1),
            vec![ExpressionElement::new(
                Location::new(1, 1),
                ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                    Location::new(1, 1),
                    lexical::Literal::Unit,
                ))),
            )],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_expression() {
        let input = r#"(1)"#;

        let expected = Ok(Expression::new(
            Location::new(1, 1),
            vec![ExpressionElement::new(
                Location::new(1, 2),
                ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                    Location::new(1, 2),
                    lexical::Literal::Integer(IntegerLiteral::new_decimal("1".to_owned())),
                ))),
            )],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_single() {
        let input = r#"(1,)"#;

        let expected = Ok(Expression::new(
            Location::new(1, 1),
            vec![ExpressionElement::new(
                Location::new(1, 1),
                ExpressionObject::Operand(ExpressionOperand::Tuple(TupleExpression::new(
                    Location::new(1, 1),
                    vec![Expression::new(
                        Location::new(1, 2),
                        vec![ExpressionElement::new(
                            Location::new(1, 2),
                            ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                                Location::new(1, 2),
                                lexical::Literal::Integer(IntegerLiteral::new_decimal(
                                    "1".to_owned(),
                                )),
                            ))),
                        )],
                    )],
                ))),
            )],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"(1, 2, 3)"#;

        let expected = Ok(Expression::new(
            Location::new(1, 1),
            vec![ExpressionElement::new(
                Location::new(1, 1),
                ExpressionObject::Operand(ExpressionOperand::Tuple(TupleExpression::new(
                    Location::new(1, 1),
                    vec![
                        Expression::new(
                            Location::new(1, 2),
                            vec![ExpressionElement::new(
                                Location::new(1, 2),
                                ExpressionObject::Operand(ExpressionOperand::Literal(
                                    Literal::new(
                                        Location::new(1, 2),
                                        lexical::Literal::Integer(IntegerLiteral::new_decimal(
                                            "1".to_owned(),
                                        )),
                                    ),
                                )),
                            )],
                        ),
                        Expression::new(
                            Location::new(1, 5),
                            vec![ExpressionElement::new(
                                Location::new(1, 5),
                                ExpressionObject::Operand(ExpressionOperand::Literal(
                                    Literal::new(
                                        Location::new(1, 5),
                                        lexical::Literal::Integer(IntegerLiteral::new_decimal(
                                            "2".to_owned(),
                                        )),
                                    ),
                                )),
                            )],
                        ),
                        Expression::new(
                            Location::new(1, 8),
                            vec![ExpressionElement::new(
                                Location::new(1, 8),
                                ExpressionObject::Operand(ExpressionOperand::Literal(
                                    Literal::new(
                                        Location::new(1, 8),
                                        lexical::Literal::Integer(IntegerLiteral::new_decimal(
                                            "3".to_owned(),
                                        )),
                                    ),
                                )),
                            )],
                        ),
                    ],
                ))),
            )],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
