pub struct TableDissolver<'a> {
    source: &'a str,
}

impl<'a> TableDissolver<'a> {
    pub fn new(source: &'a str) -> TableDissolver {
        TableDissolver {
            source
        }
    }
}
