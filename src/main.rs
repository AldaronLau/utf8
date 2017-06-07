use std::env;
use std::u8;

fn main() {
	let mut out_string: String = "".to_string();

	for argument in env::args().skip(1) {
//		println!("converting: {}", argument);

		if let Ok(x) = u8::from_str_radix(argument.as_str(), 16) {
			if let Ok(y) = String::from_utf8(vec![x]) {
				out_string += y.as_str();
			} else {
				panic!("oh boy")
			};
		} else {
			panic!("Error parsing");
		}
	}

	println!("MSG: {}", out_string);
}
