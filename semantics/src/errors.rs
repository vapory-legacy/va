//! Semantic errors.

/// Errors for things that may arise in a valid Fe AST.
#[derive(Debug, PartialEq)]
pub enum SemanticError {
    BreakWithoutLoop,
    ContinueWithoutLoop,
    MissingReturn,
    NotAnExpression,
    NotSubscriptable,
    UnassignableExpression,
    UndefinedValue { value: String },
    UnexpectedReturn,
    TypeError,
}
