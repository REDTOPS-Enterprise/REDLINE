#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Float, // Although not yet fully supported, it's in the docs.
    String,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::String => "std::string".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
}

impl ToString for BinaryOperator {
    fn to_string(&self) -> String {
        match self {
            BinaryOperator::Add => "+".to_string(),
            BinaryOperator::Subtract => "-".to_string(),
            BinaryOperator::Multiply => "*".to_string(),
            BinaryOperator::Divide => "/".to_string(),
            BinaryOperator::Equal => "==".to_string(),
            BinaryOperator::NotEqual => "!=".to_string(),
            BinaryOperator::GreaterThan => ">".to_string(),
            BinaryOperator::LessThan => "<".to_string(),
            BinaryOperator::GreaterThanEqual => ">=".to_string(),
            BinaryOperator::LessThanEqual => "<=".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Call(String, Vec<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Declaration {
        is_mutable: bool, // var vs val
        name: String,
        data_type: Type,
        initializer: Expression,
    },
    If {
        condition: Expression,
        consequence: Vec<Statement>, // 'then' block
        alternative: Option<Vec<Statement>>, // 'else' block
    },
    Print(Expression),
    FunctionDefinition {
        name: String,
        params: Vec<(String, Type)>, // (name, type)
        return_type: Type,
        body: Vec<Statement>,
    },
    Return(Option<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}
