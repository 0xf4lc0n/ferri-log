use skytable::types::{FromSkyhashBytes, IntoSkyhashBytes};

pub trait Cache {
    fn get<T: FromSkyhashBytes>(&self, key: &str) -> Result<T, skytable::error::Error>;

    fn set<T: IntoSkyhashBytes>(&self, key: &str, value: T)
        -> Result<bool, skytable::error::Error>;

    fn update<T: IntoSkyhashBytes>(
        &self,
        key: &str,
        value: T,
    ) -> Result<(), skytable::error::Error>;

    fn del(&self, key: &str) -> Result<u64, skytable::error::Error>;
}
