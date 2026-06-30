#[derive(Debug, Clone)]
pub enum PolicyAction {
    Allow,
    Deny,
    Require,
    Audit,
    Quarantine,
}

#[derive(Debug, Clone)]
pub enum ConditionOp {
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    Has,
    In,
    Exists,
    Matches,
    Signed,
    Trusted,
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub key: String,
    pub op: ConditionOp,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub action: PolicyAction,
    pub target: String,
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone)]
pub struct Policy {
    pub name: String,
    pub rules: Vec<Rule>,
}
