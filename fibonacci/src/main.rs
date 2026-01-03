use std::env;

fn main() {
	hello_world("Takeshi");
	fibonacci(10);
}

fn fibonacci(n: i32){
	let mut a: i32 = 1;
	let mut b: i32 = 1;
	let mut sum: i32;

	if n >= 1 {println!("{}", a);}
	if n >= 2 {println!("{}", b);}
	for _i in 2..n {
		sum = add(a, b);
		println!("{}",sum);
		a = b;
		b = sum;
	}
}

fn add(a: i32, b: i32) -> i32 {
	a + b
}

fn hello_world(name: &str){
	println!("Hello, {}!", name);
}

