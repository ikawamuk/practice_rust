
pub fn var() {
	let x = 5;
	println!("The value of x is: {}", x);
	let x = x + 1;
	println!("The value of x is: {}", x);
	let mut x = 1;
	println!("The value of x is: {}", x);
	x = x + 1;
	println!("The value of x is: {}", x);
	x += 1;
	println!("The value of x is: {}", x);
	{
		let x = x * 2;
		println!("The value of x in the scope is: {}", x);
	}
	let x = x;
	println!("The value of x is: {}", x);
}

