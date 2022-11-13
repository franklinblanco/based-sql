use std::{path::Path, net::Ipv4Addr};

use surrealdb::{Datastore, Session, Error};

use crate::utils::{as_datastore_file_path, as_datastore_address};

#[allow(dead_code)]
pub struct SurrealConnection {
    datastore: Datastore,
    session: Session,
}

/// The three possible ways to open a surrealDB datastore at the moment. Defaults to memory.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConnectionType<'a> {
    #[default]
    Memory,
    File(&'a Path),
    TK(Ipv4Addr)
}

#[allow(dead_code, unused)]
impl SurrealConnection {
    /// Creates a datastore with the specified connection type. Specify a namespace name otherwise specify None and it will asign "my_namespace" as the namespace name.
    async fn create<'a>(connection_type: ConnectionType<'a>, namespace: Option<&'a str>) -> Result<SurrealConnection, Error> {
        let ds = match connection_type {
            ConnectionType::Memory => Datastore::new("memory").await?,
            ConnectionType::File(path) => Datastore::new(as_datastore_file_path(path).as_str()).await?,
            ConnectionType::TK(address) => Datastore::new(as_datastore_address(&address).as_str()).await?,
        };
        let sess = Session::for_db(match namespace {
            Some(session_name) => session_name,
            None => "my_namespace",
        }, "my_database");
        Ok(SurrealConnection {
            datastore: ds,
            session: sess,
        })
    }
}