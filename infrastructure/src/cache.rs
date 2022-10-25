use skytable::{
    actions::Actions,
    sync::Connection,
    types::{FromSkyhashBytes, IntoSkyhashBytes},
};

use application::prelude::Cache;

pub struct SkyTableCache<'a> {
    host: &'a str,
    port: u16,
}

impl<'a> SkyTableCache<'a> {
    pub fn new(host: &'a str, port: u16) -> Self {
        SkyTableCache { host, port }
    }
}

impl<'a> Cache for SkyTableCache<'a> {
    fn get<T: FromSkyhashBytes>(&self, key: &str) -> Result<T, skytable::error::Error> {
        Connection::new(self.host, self.port)?.get(key)
    }

    fn set<T: IntoSkyhashBytes>(
        &self,
        key: &str,
        value: T,
    ) -> Result<bool, skytable::error::Error> {
        Connection::new(self.host, self.port)?.set(key, value)
    }

    fn update<T: IntoSkyhashBytes>(
        &self,
        key: &str,
        value: T,
    ) -> Result<(), skytable::error::Error> {
        Connection::new(self.host, self.port)?.update(key, value)
    }

    fn del(&self, key: &str) -> Result<u64, skytable::error::Error> {
        Connection::new(self.host, self.port)?.del(key)
    }
}
