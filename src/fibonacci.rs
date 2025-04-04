pub fn fibo_rec(mut n: i64) -> i64{
	if n == 0 || n == 1{
		println!("Die Zahl ist 1 Fibo: {}", n);
		return n;
		}
	else { 
		rec(n)
	}
}

pub fn rec(mut n: i64) -> i64{
	println!("Berechne Fibo({})", n);
	n = (n-1) + (n-2);
	println!("Nach Berechnung({})", n);
	fibo_rec(n)
}
