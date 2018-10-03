use juniper::{FieldResult, RootNode};

#[derive(GraphQLEnum)]
enum Breed {
  Shiba,
  Corgi,
  Other
}

#[derive(GraphQLEnum)]
enum Gender {
  Male,
  Female
}

#[derive(GraphQLObject)]
#[graphql(description = "Puppies!")]
struct Puppy {
  name: String,
  breed: Breed,
  gender: Gender
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New Puppy?!")]
struct PuppyInput {
  name: String,
  breed: Breed,
  gender: Gender
}

pub struct QueryRoot;
graphql_object!(QueryRoot: () |&self| {
  field puppy(&executor, name: String) -> FieldResult<Puppy> {
    Ok(Puppy {
      name: name,
      breed: Breed::Shiba,
      gender: Gender::Male
    })
  }
});

pub struct MutationRoot;
graphql_object!(MutationRoot: () |&self| {
  field createPuppy(&executor, new_puppy: PuppyInput) -> FieldResult<Puppy> {
    Ok(Puppy {
      name: new_puppy.name,
      breed: new_puppy.breed,
      gender: new_puppy.gender
    })
  }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}
