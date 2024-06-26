/* It prints:
30 true*/
fn main() {
  struct Person (String, u32);
  #[allow(dead_code)]
  enum Visibility { Visible, Hidden, Collapsed }
  impl Person {
      fn age(&self) -> u32 {
          self.1
      }
  }
  impl Visibility {
      fn is_not_visible(&self) -> bool {
          match self {
              Visibility::Visible => false,
              _ => true,
          }
      }
  }
  println!("{} ", Person ("John".to_string(), 30).age());
  println!("{}", Visibility::Collapsed.is_not_visible());
  println!("{}", Visibility::Visible.is_not_visible());
}
