pub fn erathostenes(maximum:usize)-> Vec<usize>{
	let mut primes = vec![true; maximum + 1];
	primes[0] = false;
	primes[1] = false;
	
	let limit = (maximum as f64).sqrt() as usize;
	
	for num in 2..=limit {
		if primes[num]{
			let mut multiple = num * num;
			while multiple <= maximum {
				multiple += num;
			}
		}	
	}
	primes.iter()
		.enumerate()
		.filter(|(_, is_prime) | is_prime)
		.map(|(index, _) | index)
		.collect()
}
