fn main() {
  
  // let width1 = 30;
  // let height1 = 50;
  let rec1 = (30, 50);

  println!("The aread of rectangle is {} square pixels.", 
           area(rec1)
  );
}

fn area(dim: (u32, u32)) -> u32 {
  dim.0 * dim.1
}

/*
fn area(width: u32, height: u32) -> u32 {
  width * height

}*/
