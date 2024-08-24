use crate::lexer::{Literal, Token};

type ExprArguments = Vec<Box<Expr>>;


pub enum Expr {
    Assign { name: Token, value: Box<Expr> },
    LiteralExpr { value: Literal },
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Call { callee: Box<Expr>, paren: Token, arguments: ExprArguments },
    Get { object: Box<Expr>, name: Token },
    Grouping { expression: Box<Expr> },
    Logical { left: Box<Expr>, right: Box<Expr>, operator: Token },
    Set { object: Box<Expr>, name: Token, value: Box<Expr> },
    Super { keyword: Token, method: Token },
    This { keyword: Token },
    Unary { operator: Token, right: Box<Expr> },
    Variable { name: Token },
}

impl Expr {
    pub fn accept<R>(&self, expr_visitor: &impl Visitor<R>) -> R {
        match self {
            Expr::Binary { .. } => expr_visitor.visit_binary_expression(self),
            Expr::LiteralExpr { .. } => expr_visitor.visit_literal_expression(self),
            Expr::Assign { .. } => expr_visitor.visit_assign_expression(self),
            Expr::Call { .. } => expr_visitor.visit_call_expression(self),
            Expr::Get { .. } => expr_visitor.visit_get_expression(self),
            Expr::Grouping { .. } => expr_visitor.visit_grouping_expression(self),
            Expr::Logical { .. } => expr_visitor.visit_logical_expression(self),
            Expr::Set { .. } => expr_visitor.visit_set_expression(self),
            Expr::Super { .. } => expr_visitor.visit_super_expression(self),
            Expr::This { .. } => expr_visitor.visit_this_expression(self),
            Expr::Unary { .. } => expr_visitor.visit_unary_expression(self),
            Expr::Variable { .. } => expr_visitor.visit_variable_expression(self),
        }
    }
}

pub trait Visitor<R> {
    fn visit_binary_expression(&self, expr: &Expr) -> R;
    fn visit_literal_expression(&self, expr: &Expr) -> R;
    fn visit_assign_expression(&self, expr: &Expr) -> R;
    fn visit_call_expression(&self, expr: &Expr) -> R;
    fn visit_get_expression(&self, expr: &Expr) -> R;
    fn visit_grouping_expression(&self, expr: &Expr) -> R;
    fn visit_logical_expression(&self, expr: &Expr) -> R;
    fn visit_set_expression(&self, expr: &Expr) -> R;
    fn visit_super_expression(&self, expr: &Expr) -> R;
    fn visit_this_expression(&self, expr: &Expr) -> R;
    fn visit_unary_expression(&self, expr: &Expr) -> R;
    fn visit_variable_expression(&self, expr: &Expr) -> R;
}

