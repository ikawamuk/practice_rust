use std::io;

fn celsius_to_fahrenheit(cel_dgeree:f64)->f64 {
	(cel_dgeree * 1.8) + 32.0
}

fn fahrenheit_to_celsius(fah_dgeree:f64)->f64 {
	(fah_dgeree - 32.0) * 0.55
}

fn main(){
	loop {
		let mut input: String = String::new();
		while input.trim().is_empty() {
			println!("Please input a degree (ex: 25C, 77F). quit with 'q'");
			io::stdin()
				.read_line(&mut input)
				.expect("Faild to read");
		}
		let input = input.trim();
		if input == "q" {
			break ;
		}
		let (num_str, unit) = input.split_at(input.len() - 1);
		let degree: f64 = match num_str.parse() {
			Ok(num)=>num,
			Err(_)=>{
				println!("Enter a valid number");
				continue ;
			}
		};
		match unit.to_uppercase().as_str() {
			"C" => {
				let f = celsius_to_fahrenheit(degree);
				println!("{:.1}°C is {:.1}°F", degree, f);
			}
			"F" => {
				let c = fahrenheit_to_celsius(degree);
				println!("{:.1}°F is {:.1}°C", degree, c);
			}
			_ => println!("put dgree with unit 'C' or 'F'")
		}
	}
}
