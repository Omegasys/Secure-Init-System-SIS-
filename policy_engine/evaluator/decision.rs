use crate::parser::ast::{PolicyAction, Rule};

#[derive(Debug, Clone)]
pub enum Decision {
    Allow,
    Deny,
    Require,
    Audit,
    Quarantine,
}

impl Decision {
    pub fn from(rule: &Rule) -> Self {
        match rule.action {
            PolicyAction::Allow => Decision::Allow,
            PolicyAction::Deny => Decision::Deny,
            PolicyAction::Require => Decision::Require,
            PolicyAction::Audit => Decision::Audit,
            PolicyAction::Quarantine => Decision::Quarantine,
        }
    }
}
