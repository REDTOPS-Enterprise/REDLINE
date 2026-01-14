//! The Abstract Syntax Tree (AST) for the REDLINE language.
//! Each node in the tree represents a construct in the code, like a statement or an expression.

/// Represents the fundamental data types in REDLINE.
#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    List(Box<Type>),
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "double".to_string(),
            Type::String => "std::string".to_string(),
            Type::Bool => "bool".to_string(),
            Type::List(inner) => format!("std::vector<{}>", inner.to_string()),
        }
    }
}

/// Represents a literal value in the source code.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

/// Represents a binary operator.
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Add, Subtract, Multiply, Divide,
    Equal, NotEqual, GreaterThan, LessThan, GreaterThanEqual, LessThanEqual,
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

/// Represents an expression. An expression is a piece of code that evaluates to a value.
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// A literal value, e.g., `10`, `"hello"`, `true`.
    Literal(Literal),
    /// A list literal, e.g., `[1, 2, 3]`.
    ListLiteral(Vec<Expression>),
    /// An identifier, e.g., a variable name like `x`.
    Identifier(String),
    /// A binary operation, e.g., `x + 5`.
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// A function call, e.g., `my_func(a, b)`.
    Call(String, Vec<Expression>),
    /// An index access expression, e.g., `my_list[0]`.
    Index {
        list: Box<Expression>,
        index: Box<Expression>,
    },
}

/// Represents a statement. A statement is a piece of code that performs an action.
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// A variable or constant declaration, e.g., `var x: int = 10`.
    Declaration {
        is_mutable: bool,
        name: String,
        data_type: Type,
        initializer: Expression,
    },
    /// An assignment to an existing variable or element, e.g., `x = 20` or `my_list[0] = 1`.
    Assignment {
        target: Expression,
        value: Expression,
    },
    /// An `if-else` statement.
    If {
        condition: Expression,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    },
    /// A `while` loop.
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    /// A `for` loop.
    For {
        iterator: String,
        start: Expression,
        end: Expression,
        body: Vec<Statement>,
    },
    /// A `print` statement.
    Print(Expression),
    /// A standalone expression statement, typically a function call.
    Expression(Expression),
    /// A function definition.
    FunctionDefinition {
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Statement>,
    },
    /// A `return` statement.
    Return(Option<Expression>),
}

/// The root of the AST, representing the entire program as a list of statements.
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}
