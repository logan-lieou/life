use rand::Rng;

fn life(choice: bool) -> bool {
   let mut rng = rand::thread_rng();
   if choice == false { return false; };
   let x: i32 = rng.gen_range(0, 2);
   return if x == 0 { true } else { false };
}

fn main() {
   let x = life(true);
   println!("{:?}", x);
}
