use std::sync::RwLock;

use tera::Tera;
pub use tera::Context;

lazy_static! {
  static ref TERA: RwLock<Tera> = {
    let tera = compile_templates!("views/**/*");
    RwLock::new(tera)
  };
}

pub fn view(path: &str, context: Context) -> String {
  TERA
    .read()
    .and_then(|tera| {
      Ok(tera.render(path, &context).unwrap_or_else(|err| {
        println!("rendering error: {:?}", err);
        "rendering error".to_owned()
      }))
    })
    .unwrap()
}
