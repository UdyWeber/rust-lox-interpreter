use crate::expressions::{Expr, Visitor};

struct AstPrinter {}

// TODO: Implement complete printer later on...
impl AstPrinter {
    fn print(&self, expr: &Expr) {
        println!("{}", expr.accept(self));
    }

    fn parenthesize(&self, name: &str, arguments: Vec<&Box<Expr>>) -> String {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name);

        arguments.iter().for_each(|e| {
            builder.push(' ');
            builder.push_str(e.accept(self).as_str())
        });

        builder.push(')');
        return builder;
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expression(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, right, operator } => self.parenthesize(operator.lexeme.as_str(), vec![left, right]),
            _ => panic!("Fudeu")
        }
    }
    fn visit_literal_expression(&self, expr: &Expr) -> String {
        match expr {
            Expr::LiteralExpr { value } => value.to_string(),
            _ => panic!("Fudeu 2")
        }
    }

    fn visit_assign_expression(&self, expr: &Expr) -> String {
        String::new()
    }

    fn visit_call_expression(&self, expr: &Expr) -> String {
        String::new()
    }

    fn visit_get_expression(&self, expr: &Expr) -> String {
        String::new()
    }

    fn visit_grouping_expression(&self, expr: &Expr) -> String {
        match expr {
            Expr::Grouping { expression } => self.parenthesize("group", vec!(expression)),
            _ => panic!("Wrong type")
        }
    }

    fn visit_logical_expression(&self, expr: &Expr) -> String {
        String::new()
    }

    fn visit_set_expression(&self, expr: &Expr) -> String {
        String::new()
    }

    fn visit_super_expression(&self, expr: &Expr) -> String {
        String::new()
    }

    fn visit_this_expression(&self, expr: &Expr) -> String {
        String::new()
    }

    fn visit_unary_expression(&self, expr: &Expr) -> String {
        match expr {
            Expr::Unary { right, operator } => self.parenthesize(operator.lexeme.as_str(), vec!(right)),
            _ => panic!("Wrong...")
        }
    }

    fn visit_variable_expression(&self, expr: &Expr) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::Expr::{Binary, Grouping, LiteralExpr, Unary};
    use crate::lexer::{Literal, Token, TokenType};
    use super::*;


    #[test]
    fn string_literal_binary() {
        let visitor = AstPrinter {};
        let l1 = LiteralExpr { value: Literal::String(String::from("Shermak")) };
        let l2 = LiteralExpr { value: Literal::String(String::from("Jaw")) };
        let bi = Binary {
            left: Box::new(l1),
            operator: Token::new(TokenType::PLUS, "+", Literal::String(String::from("+")), 1),
            right: Box::new(l2),
        };

        visitor.print(&bi);
        assert_eq!("(+ Shermak Jaw)", bi.accept(&visitor));
    }

    #[test]
    fn book_example() {
        let visitor = AstPrinter {};
        let unary = Unary {
            right: Box::from(LiteralExpr { value: Literal::Number(123_f32, 0) }),
            operator: Token::new(
                TokenType::MINUS,
                String::from("-").as_str(),
                Literal::String(String::from("-")),
                1,
            )
        };
        let grouping = Grouping {
            expression: Box::from(LiteralExpr { value: Literal::Number(45.67, 2) })
        };
        let bi = Binary {
            operator: Token::new(TokenType::STAR, "*", Literal::String(String::from("*")), 1),
            left: Box::new(unary),
            right: Box::new(grouping),
        };

        visitor.print(&bi);
        assert_eq!("(* (- 123) (group 45.67))", bi.accept(&visitor));
    }
}