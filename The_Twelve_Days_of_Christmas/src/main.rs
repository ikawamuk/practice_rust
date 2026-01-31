fn main() {
	for i in 1..=12 {
		on_the(i);
		my_true_love();
		gift(i);
		if i == 12 {return;}
		println!("");
	}
}

fn on_the(day: usize) {
	const DAY_STRS: [&str; 12] = ["first","second","third","fourth","fifth","sixth",
								"seventh","eighth","ninth","tenth","eleventh","twelfth"];
	let day = DAY_STRS[day - 1];
	println!("On the {} day of  Christmas,", day);
}

fn my_true_love(){
	println!("My true love sent to me");
}

fn gift(day: usize){
	const GIFTS: [&str; 12] = [
		"a partridge in a pear tree.",
		"Two turtle doves,",
		"Three French hens,",
		"Four calling birds,",
		"Five gold rings,",
		"Six geese a-laying,",
		"Seven swans a-swimming,",
		"Eight maids a-milking,",
		"Nine ladies dancing,",
		"Ten lords a-leaping,",
		"Eleven pipers piping,",
		"Twelve drummers drumming,"
	];
	for i in (0..day).rev() {
		if i == 0 {
			if day != 1 {print!("And ");}
			else {
				let mut str: String = GIFTS[i].to_string();
				str[0..1].make_ascii_uppercase();
				println!("{}", str);
				continue;
			}
		}
		println!("{}", GIFTS[i]);
	}
}