use rand::Rng;

pub fn op(){
	let z :i32 = rand::thread_rng().gen_range(0..1);
	let a :i32 = i32::MAX - z;
	let b :i32 = 2;
	let c: i32 = 1;

	let mut sum: i32;

	sum = a + b;
	println!("a + b :{}", sum);
	sum = b + c;
	println!("b + c :{}", sum);
	sum = c + a;
	println!("c + a :{}", sum);
}