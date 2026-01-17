//! The Abstract Syntax Tree (AST) for the REDLINE language.
//! Each node in the tree represents a construct in the code, like a statement or an expression.
use serde::Serialize;

/// Represents the fundamental data types in REDLINE.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Void, // Represents the absence of a return value
    List(Box<Type>),
    Class(String), // Represents a user-defined class type
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "double".to_string(),
            Type::String => "std::string".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Void => "void".to_string(),
            Type::List(inner) => {
                // If the list contains class objects, it's a list of smart pointers.
                if let Type::Class(class_name) = &**inner {
                    format!("std::vector<std::shared_ptr<{}>>", class_name)
                } else {
                    format!("std::vector<{}>", inner.to_string())
                }
            },
            Type::Class(name) => format!("std::shared_ptr<{}>", name),
        }
    }
}

/// Represents a literal value in the source code.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

/// Represents a binary operator.
#[derive(Debug, PartialEq, Clone, Serialize)]
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
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Expression {
    Literal(Literal),
    ListLiteral(Vec<Expression>),
    Identifier(String),
    BinaryOp { op: BinaryOperator, left: Box<Expression>, right: Box<Expression> },
    /// A function or method call. `callee` is the expression being called.
    Call { callee: Box<Expression>, args: Vec<Expression> },
    Index { list: Box<Expression>, index: Box<Expression> },
    /// Member access, e.g., `my_object.member`.
    Get { object: Box<Expression>, name: String },
    /// The `this` keyword.
    This,
    /// Heap allocation, e.g., `new MyClass()`.
    New { class_name: String, args: Vec<Expression> },
}

/// Represents a single member of a class (either a variable or a function).
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum ClassMember {
    Variable(Statement), // Using Declaration statement
    Method(Statement),   // Using FunctionDefinition statement
    Constructor(Statement), // Represents the 'init' method
}

/// Represents a statement. A statement is a piece of code that performs an action.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Statement {
    Import(String),
    Declaration { is_public: bool, is_mutable: bool, name: String, data_type: Type, initializer: Expression },
    Assignment { target: Expression, value: Expression },
    If { condition: Expression, consequence: Vec<Statement>, alternative: Option<Vec<Statement>> },
    While { condition: Expression, body: Vec<Statement> },
    For { iterator: String, start: Expression, end: Expression, body: Vec<Statement> },
    Print(Expression),
    Expression(Expression),
    FunctionDefinition { is_public: bool, name: String, params: Vec<(String, Type)>, return_type: Type, body: Vec<Statement> },
    Return(Option<Expression>),
    /// A class definition.
    Class { is_public: bool, name: String, members: Vec<ClassMember> },
    /// A try-catch block.
    TryCatch { try_block: Vec<Statement>, catch_var: String, catch_block: Vec<Statement> },
}

/// The root of the AST, representing the entire program as a list of statements.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}
