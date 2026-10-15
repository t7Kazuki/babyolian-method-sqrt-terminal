use std::io;
use f256::f256;

fn main() {
    let mut x: f256 = f256::from(1.0);
    let mut term;
	let mut state:char = 'y';
	while state == 'y' {
		println!("Value?");
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		let S: f256 = input.trim().parse().unwrap();

		println!("Num of Iterations?");
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		let mut iterations: i32 = input.trim().parse().unwrap();

		let max_it = iterations;
		
	    println!("\n");
	    while iterations != 0{
	    	x = (x+S/x)/f256::from(2);
	    	term = max_it - iterations;
	    	iterations = iterations - 1;
	    	println!("{term}: {x}");
	    }

	    println!("\n final: {x}");
	    x = f256::from(1.0);

	    println!("\nAnother?(y/n)");
	    let mut input = String::new();
	    io::stdin().read_line(&mut input).unwrap();
	    state = input.trim().parse().unwrap();
	}
    
}
