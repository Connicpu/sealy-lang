#[derive(Debug)]
pub struct TypeName<'input> {
    pub parts: Vec<&'input str>,
}
