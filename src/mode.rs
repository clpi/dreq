pub type Modes<'a> = Vec<Mode<'a>>;

#[derive(Debug, Default)]
pub struct Mode<'a> {
    name: &'a str,
    active: bool,
}

impl<'a> Mode<'a> {
    pub fn new(name: &'a str, active: bool) -> Self
    {
        Self { name, active }
    }
}

