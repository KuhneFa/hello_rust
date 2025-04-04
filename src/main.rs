mod ranges;
mod fibonacci;
mod sieb;

fn main() {
	//ranges::integer_ranges();
//	let result = fibonacci::fibo_rec(10);
//	println!("Ergebnis: {}", result);
	let maximum = 100_000;
	let primes = sieb::erathostenes(maximum);
	println!("Primzahlen zwischen 2 und {} gefunden: {}", maximum, primes.len());
	for prime in primes.iter().take(20){
		println!("{}", prime)	
	}
}
