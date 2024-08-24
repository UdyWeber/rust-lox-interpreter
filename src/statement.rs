use crate::expressions::Expr;
use crate::lexer::Token;

type StmtArgs = Vec<Stmt>;
// Methods is the structure Stmt::Function
type Methods = Vec<Stmt>;
type FunctionParams = Vec<Token>;

pub trait StmtVisitor<R> {
    fn visit_block_stmt(&self, stmt: &Stmt) -> R;
    fn visit_class_stmt(&self, stmt: &Stmt) -> R;
    fn visit_expression_stmt(&self, stmt: &Stmt) -> R;
    fn visit_function_stmt(&self, stmt: &Stmt) -> R;
    fn visit_if_stmt(&self, stmt: &Stmt) -> R;
    fn visit_print_stmt(&self, stmt: &Stmt) -> R;
    fn visit_return_stmt(&self, stmt: &Stmt) -> R;
    fn visit_variable_stmt(&self, stmt: &Stmt) -> R;
    fn visit_while_stmt(&self, stmt: &Stmt) -> R;
}

pub enum Stmt {
    Print { expression: Expr },
    Block { statements: StmtArgs },
    Expression { expression: Expr },
    While { condition: Expr, body: Box<Stmt> },
    Return { keyword: Token, value: Expr },
    Variable { name: Token, initializer: Expr },
    If { condition: Expr, then_branch: Box<Stmt>, else_branch: Box<Stmt> },
    Function { name: Token, params: FunctionParams, body: StmtArgs },
    Class { name: Token, super_class: Expr, methods: Methods },
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &impl StmtVisitor<R>) -> R {
        match self {
            Stmt::Print { .. } => visitor.visit_print_stmt(self),
            Stmt::Block { .. } => visitor.visit_block_stmt(self),
            Stmt::Expression { .. } => visitor.visit_expression_stmt(self),
            Stmt::While { .. } => visitor.visit_while_stmt(self),
            Stmt::Return { .. } => visitor.visit_return_stmt(self),
            Stmt::Variable { .. } => visitor.visit_variable_stmt(self),
            Stmt::If { .. } => visitor.visit_if_stmt(self),
            Stmt::Function { .. } => visitor.visit_function_stmt(self),
            Stmt::Class { .. } => visitor.visit_class_stmt(self),
        }
    }
}