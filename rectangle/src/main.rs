#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  
  // let width1 = 30;
  // let height1 = 50;
  // let rec1 = (30, 50);
  
  let rect1 = Rectangle {
    width: 30, 
    height: 50,
  };

  // rect1 is the owner of the rectangle data. 
  // This value can be borrowed to a reference. 
  //
  // Since the rference is not the owner, the data is not out of 
  // scope and not dropped. 

  println!("The aread of rectangle is {} square pixels.", 
           area(&rect1)
  );
}

fn area(rectangle: &Rectangle) -> u32 {
  println!("{}", rectangle);
  rectangle.width * rectangle.height
}

/*
fn area(dim: (u32, u32)) -> u32 {
  dim.0 * dim.1
}
*/

/*
fn area(width: u32, height: u32) -> u32 {
  width * height

}*/
