use crate::shapes::{rectangle::Rect, circle::Circle, area::Area};

mod shapes;

fn main() {
  let rect = Rect {
    x: 0.0,
    y: 0.0,
    width: 0.0,
    height: 0.0
  };
  let circle = Circle{
    x: 0.0,
    y: 0.0,
    radius: 10.0
  };
  println!("{}", rect.area());
  println!("{}", circle.area());
}