use crate::parser::ast::{Policy, Rule};
use crate::evaluator::decision::Decision;
use crate::evaluator::rules::evaluate_rule;

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn evaluate(policy: &Policy, event: &str) -> Decision {
        for rule in &policy.rules {
            if evaluate_rule(rule, event) {
                return Decision::from(rule);
            }
        }

        Decision::Deny
    }
}
