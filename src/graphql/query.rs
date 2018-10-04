use super::Customer;

use juniper::FieldResult;

pub struct QueryRoot;
graphql_object!(QueryRoot: () |&self| {
  field customer(&executor, id: String) -> FieldResult<Customer> {
    Ok(Customer {
      id: "1".into(),
      first_name: "Nate".into(),
      last_name: "Strandberg".into(),
      email: "nater540@gmail.com".into()
    })
  }
});
