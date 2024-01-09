mod shapes;
use shapes::{rectangle::Rect, circle::Circle, area::Area};


fn main() {
  let rect = Rect {
    x: 0.0,
    y: 0.0,
    width: 4.0,
    height: 3.0
  };
  let circle = Circle{
    x: 0.0,
    y: 0.0,
    radius: 10.0
  };

  let default_rect = Rect::default();

  println!("{}", rect.area());
  println!("{}", circle.area());
  println!("{}", default_rect.area());
  println!("{}", default_rect);
  default_rect.to_string();
  println!("{}", default_rect)
}