use crate::parser::ast::Condition;

/// Evaluates a condition against a runtime context (simplified).
pub fn evaluate_condition(condition: &Condition, context_value: &str) -> bool {
    match condition.op {
        crate::parser::ast::ConditionOp::Eq => {
            condition.value.as_deref() == Some(context_value)
        }

        crate::parser::ast::ConditionOp::Exists => {
            !context_value.is_empty()
        }

        crate::parser::ast::ConditionOp::Matches => {
            if let Some(v) = &condition.value {
                context_value.contains(v)
            } else {
                false
            }
        }

        _ => false, // extended ops later
    }
}
