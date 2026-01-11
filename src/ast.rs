use std::range::Range;

pub struct Program {
    items: Vec<Item>,
}

impl Program {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}

pub enum Item {
    Function {
        name: String,
        arguments: Vec<String>,
        body: Statement,
        span: Range<usize>,
    },
}

pub enum Statement {
    Block {
        body: Vec<Statement>,
        span: Range<usize>,
    },
    Binding {
        bind_to: String,
        value: Expression,
        span: Range<usize>,
    },
    Expression {
        expression: Expression,
        span: Range<usize>,
    },
    Return {
        value: Expression,
        span: Range<usize>,
    },
    Branch {
        condition: Expression,
        then: Box<Statement>,
        otherwise: Option<Box<Statement>>,
        span: Range<usize>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
        span: Range<usize>,
    },
}

pub enum Expression {
    Integer {
        value: i64,
        span: Range<usize>,
    },
    Boolean {
        value: bool,
        span: Range<usize>,
    },
    Reference {
        name: String,
        span: Range<usize>,
    },
    Operation {
        operation: Operation,
        span: Range<usize>,
    },
    Assignment {
        assign_to: String,
        value: Box<Expression>,
        span: Range<usize>,
    },
    Call {
        name: String,
        arguments: Vec<Expression>,
        span: Range<usize>,
    },
}

pub enum Operation {
    Binary(BinaryOperation),
    Unary(UnaryOperation),
}

pub enum BinaryOperation {
    Add {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Sub {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Mul {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Div {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Equal {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    NotEqual {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Less {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    LessEqual {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Greater {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    GreaterEqual {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    And {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Or {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

pub enum UnaryOperation {
    Not(Box<Expression>),
}
