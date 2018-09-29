use super::CorgiState;
use super::render;

pub fn index(_: CorgiState) -> String {
  let mut context = render::Context::new();
  context.insert("name", "Lord Fabulous");

  render::view("index.html", context)
}
