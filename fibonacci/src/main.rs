use std::io;

fn main() {
	hello_world("fibonacci");
	let input: i32 = loop {
	let mut input: String = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read");
		match input.trim().parse() {
			Ok(n)=> {break n;}
			Err(_)=>{
				println!("Input a number");
				continue ;
			}
		};
	};
	fibonacci(input);
}

fn fibonacci(n: i32){
	let mut a: i32 = 1;
	let mut b: i32 = 1;
	let mut sum: i32 = 1;

	if n == 0 {
		println!("0 is invalid");
		return ;
	}
	if n == 1 {
		println!("{}", a);
		return ;
	}
	if n == 2 {
		println!("{}", b);
		return ;
	}
	for _i in 2..n {
		sum = add(a, b);
		// println!("{}",sum);
		a = b;
		b = sum;
	}
	println!("result: {}", sum);
}

fn add(a: i32, b: i32) -> i32 {
	a + b
}

fn hello_world(name: &str){
	println!("Hello, {}!", name);
}

