#[derive(GraphQLObject)]
#[graphql(description = "Customer")]
pub struct Customer {
  pub id:         String,
  pub first_name: String,
  pub last_name:  String,
  pub email:      String
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New Customer")]
pub struct NewCustomer {
  pub first_name: String,
  pub last_name:  String,
  pub email:      String
}
