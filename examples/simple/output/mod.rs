// DO NOT EDIT THIS FILE
// This file is generated by https://github.com/tacogips/async-graphql-reverse
mod objects;
pub use objects::*;
mod input_objects;
pub use input_objects::*;
mod unions;
pub use unions::*;
mod scalars;
pub use scalars::*;
mod interfaces;
pub use interfaces::*;
mod enums;
use async_graphql::*;
pub use enums::*;
pub fn schema() -> Schema<Query, Mutation, EmptySubscription> {
    Schema::new(Query {}, Mutation {}, EmptySubscription)
}
