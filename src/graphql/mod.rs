mod query;
pub use self::query::QueryRoot as QueryRoot;

mod types;
pub use self::types::{Customer, NewCustomer};

use juniper::{FieldResult, RootNode};

pub struct MutationRoot;
graphql_object!(MutationRoot: () |&self| {
  field createCustomer(&executor, input: NewCustomer) -> FieldResult<Customer> {
    Ok(Customer {
      id: "1".into(),
      first_name: "Nate".into(),
      last_name: "Strandberg".into(),
      email: "nater540@gmail.com".into()
    })
  }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}
