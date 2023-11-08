// Copyright 2023 Sebastian Dobe <sebastiandobe@mailbox.org>
#![recursion_limit = "256"]

use std::str::FromStr;

pub mod constants;
pub mod error_response;
pub mod password_hasher;
pub mod utils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DbType {
    Sqlite,
    Postgres,
    // Mysql,
}

impl FromStr for DbType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = if s.starts_with("sqlite:") {
            Self::Sqlite
        } else if s.starts_with("postgresql://") {
            Self::Postgres
        } else {
            panic!("You provided an unknown database type, please check the DATABASE_URL");
        };

        Ok(res)
    }
}
