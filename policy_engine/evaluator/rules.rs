use crate::parser::ast::Rule;

/// Checks if a rule matches the incoming event.
/// (Simplified SIS MVP version)
pub fn evaluate_rule(rule: &Rule, event: &str) -> bool {
    rule.target == event
}
