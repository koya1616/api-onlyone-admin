/* It prints:
Ok(true)
Ok(12300000.0)
Err(ParseFloatError { kind: Invalid })
true
12300000
invalid float literal
*/
fn main() {
  println!("{:?}", "true".parse::<bool>());
  println!("{:?}", "1.23e7".parse::<f32>());
  println!("{:?}", "1.23y7".parse::<f32>());
  println!("{}", "true".parse::<bool>().unwrap());
  println!("{}", "1.23e7".parse::<f32>().unwrap());
  println!("{}", "1.23y7".parse::<f32>().unwrap_err());
}
