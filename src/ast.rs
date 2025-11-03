#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Number(i64),
    Call(Op, Vec<Node>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
