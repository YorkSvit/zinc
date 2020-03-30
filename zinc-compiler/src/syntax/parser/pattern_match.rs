//!
//! The match pattern parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::expression::terminal::Parser as TerminalOperandParser;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
use crate::syntax::tree::pattern_match::builder::Builder as MatchPatternBuilder;
use crate::syntax::tree::pattern_match::Pattern as MatchPattern;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Start,
    PathOperatorOrEnd,
    PathOperand,
}

impl Default for State {
    fn default() -> Self {
        State::Start
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: MatchPatternBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(MatchPattern, Option<Token>), Error> {
        loop {
            match self.state {
                State::Start => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Literal(lexical::Literal::Boolean(boolean)),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder
                                .set_boolean_literal(BooleanLiteral::new(location, boolean));
                            return Ok((self.builder.finish(), None));
                        }
                        Token {
                            lexeme: Lexeme::Literal(lexical::Literal::Integer(integer)),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder
                                .set_integer_literal(IntegerLiteral::new(location, integer));
                            return Ok((self.builder.finish(), None));
                        }
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder
                                .set_binding(Identifier::new(location, identifier.name));
                            self.state = State::PathOperatorOrEnd;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Underscore),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder.set_is_wildcard();
                            return Ok((self.builder.finish(), None));
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_match_pattern(
                                location, lexeme,
                            )));
                        }
                    }
                }
                State::PathOperatorOrEnd => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleColon),
                            location,
                        } => {
                            self.builder
                                .push_path_operator(ExpressionOperator::Path, location);
                            self.state = State::PathOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::PathOperand => {
                    let (expression, location, next) =
                        TerminalOperandParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.push_path_operand(expression, location);
                    self.state = State::PathOperatorOrEnd;
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
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::pattern_match::variant::Variant as MatchPatternVariant;
    use crate::syntax::tree::pattern_match::Pattern as MatchPattern;

    #[test]
    fn ok_literal_boolean() {
        let input = "true";

        let expected = Ok((
            MatchPattern::new(
                Location::new(1, 1),
                MatchPatternVariant::BooleanLiteral(BooleanLiteral::new(
                    Location::new(1, 1),
                    lexical::BooleanLiteral::r#true(),
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_literal_integer() {
        let input = "42";

        let expected = Ok((
            MatchPattern::new(
                Location::new(1, 1),
                MatchPatternVariant::IntegerLiteral(IntegerLiteral::new(
                    Location::new(1, 1),
                    lexical::IntegerLiteral::new_decimal("42".to_owned()),
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_binding() {
        let input = "value";

        let expected = Ok((
            MatchPattern::new(
                Location::new(1, 1),
                MatchPatternVariant::Binding(Identifier::new(
                    Location::new(1, 1),
                    "value".to_owned(),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 6))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_path() {
        let input = "data::Inner::VALUE";

        let expected = Ok((
            MatchPattern::new(
                Location::new(1, 1),
                MatchPatternVariant::Path(ExpressionTree::new(
                    Location::new(1, 12),
                    ExpressionTreeNode::operator(ExpressionOperator::Path),
                    Some(ExpressionTree::new(
                        Location::new(1, 5),
                        ExpressionTreeNode::operator(ExpressionOperator::Path),
                        Some(ExpressionTree::new(
                            Location::new(1, 1),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 1), "data".to_owned()),
                            )),
                            None,
                            None,
                        )),
                        Some(ExpressionTree::new(
                            Location::new(1, 7),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 7), "Inner".to_owned()),
                            )),
                            None,
                            None,
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::new(1, 14),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::new(1, 14), "VALUE".to_owned()),
                        )),
                        None,
                        None,
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 19))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_wildcard() {
        let input = "_";

        let expected = Ok((
            MatchPattern::new(Location::new(1, 1), MatchPatternVariant::Wildcard),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
