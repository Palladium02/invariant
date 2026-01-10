pub enum Item {
    Function(String, Vec<String>, Statement),
}

pub enum Statement {
    Block(Vec<Statement>),
    Binding(String, Expression),
    Expression(Expression),
    Return(Expression),
    Branch(Expression, Box<Statement>, Option<Box<Statement>>),
    While(Expression, Box<Statement>),
}

pub enum Expression {
    Integer(i64),
    Boolean(bool),
    Reference(String),
    Operation(Operation),
    Assignment(String, Box<Expression>),
    Call(String, Vec<Expression>),
}

pub enum Operation {
    Binary(BinaryOperation),
    Unary(UnaryOperation),
}

pub enum BinaryOperation {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    Less(Box<Expression>, Box<Expression>),
    LessEqual(Box<Expression>, Box<Expression>),
    Greater(Box<Expression>, Box<Expression>),
    GreaterEqual(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
}

pub enum UnaryOperation {
    Not(Box<Expression>),
}
