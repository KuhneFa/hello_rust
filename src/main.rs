use ferris_says::say;
use std::io::{stdout, BufWriter};


fn main() {
    	let stdout = stdout();
	let message = String::from("Hello fellow Rustaceans!");
	let width = message.chars().count();

	let mut writer = BufWriter::new(stdout.lock());
	say(&message, width, &mut writer).unwrap();
	println!("Hello, world!");
    	use ferris_says::say;
        
        let v = {
              let mut x = 1;
	      x + 2
	      // x += 2
	      // x
       };
       assert_eq!(v,3);
       println!("Success!");
       euler();
}

fn euler(){
	let mut k1 = 1.0;
	let mut k2 = 1.0;
	
	for k in 1..20{
		if k1 > 0.0 {
		k1 *= k as f64;
		k2 += 1.0 / k1 ;
		}
	}
	println!("euler is: {}", k1);
}
