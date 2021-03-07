pub use derive_entity::Entity;

pub struct Select {
    pub entity: &'static str,
    pub columns: Vec<&'static str>,
    pub limit: u64,
    pub unique: bool,
}

impl Default for Select {
    fn default() -> Self {
        Select {
            entity: "",
            columns: vec![],
            limit: 100,
            unique: false,
        }
    }
}

impl Select {
    pub fn set_limit(&mut self, limit: u64) -> &mut Self {
        self.limit = limit;
        self
    }
    pub fn set_unique(&mut self) -> &mut Self {
        self.unique = true;
        self
    }
    pub fn set_columns(&mut self, columns: Vec<&'static str>) -> &mut Self {
        self.columns = columns;
        self
    }
}

pub trait Sql {
    fn to_sql(&self) -> String;
}

impl Sql for Select {
    fn to_sql(&self) -> String {
        format!(
            "select{} {} from {} limit {};",
            if self.unique { " distinct" } else { "" },
            self.columns.join(","),
            self.entity,
            self.limit
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
