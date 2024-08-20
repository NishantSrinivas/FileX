#[derive(Debug)]
pub enum Ops {
    Copy(String, String),
    Search(String, String),
    Count(String, String),
    NoOps
}

#[derive(Debug)]
pub struct Operation
{
    pub ops: Ops
}
