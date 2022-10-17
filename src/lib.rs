//! # must depend on [sea_orm]
//! ```rs
//! #[automatically_derived]
//! impl sea_orm::prelude::ColumnTrait for Column {
//! ```
//! > BUT ALSO WITH [async_graphql],[seaography]

// use for marco
pub use async_graphql;
pub use sea_orm;
pub use seaography;

// use for common source files
pub use async_graphql::SimpleObject;
pub use async_trait;
pub use data_dict::*;
pub use sea_orm::entity::prelude::*;
pub use seaography::macros::{Filter, QueryRoot, RelationsCompact};

// use for main
pub use async_graphql::{
    dataloader::DataLoader, EmptyMutation, EmptySubscription, ObjectType, Response, Schema,
};
pub use sea_orm::Database;

pub struct OrmDataloader {
    pub db: DatabaseConnection,
}

pub async fn get_schema<QueryRoot: ObjectType + 'static>(
    query_root: QueryRoot,
) -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    let database = Database::connect("sqlite://sakila.db").await.unwrap();
    let orm_dataloader: DataLoader<OrmDataloader> = DataLoader::new(
        OrmDataloader {
            db: database.clone(),
        },
        tokio::spawn,
    );
    let schema = Schema::build(query_root, EmptyMutation, EmptySubscription)
        .data(database)
        .data(orm_dataloader)
        .finish();

    schema
}
