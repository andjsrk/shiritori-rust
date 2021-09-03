use std::collections::HashMap;
use std::io::Write;

fn main() {
	let dict = [
		('가', vec![ "가나다", "가위" ]),
		('다', vec![ "다다", "다리미" ]),
		('위', vec![ "위기" ]),
	]
		.iter()
		.cloned()
		.collect::<HashMap<char, Vec<&str>>>();
	let mut input = String::new();
	let mut starting_char: Option<char> = None;
	let mut input_words: Vec<String> = vec![];
	loop {
		input.clear();
		print!("단어 입력 > ");
		std::io::stdout().flush().unwrap(); // print!() does not flush stdout implicitly | https://github.com/rust-lang/rust/issues/23818
		let readline_result = std::io::stdin().read_line(&mut input);
		input = String::from(input.trim());
		if readline_result.is_ok() {
			if input.chars().count() < 2 {
				println!("단어의 길이는 2자 이상이어야 합니다.");
				continue;
			} else {
				if input == "exit()" {
					break;
				} else {
					println!("입력한 단어: {}", input);
					let first_char: char = input.chars().nth(0).unwrap();
					if starting_char.is_some() && first_char != starting_char.unwrap() {
						println!("{}는 {}로 시작하지 않습니다.", first_char, starting_char.unwrap());
						continue;
					}
					let found_words = dict.get(&first_char);
					if found_words.is_some() {
						if found_words.unwrap().contains(&&*input) {
							let last_char = input.chars().last().unwrap();
							let continuable_words = dict.get(&last_char);
							if continuable_words.is_none() {
								println!("와 한방단어");
								break;
							} else {
								if input_words.contains(&input) {
									println!("{}은(는) 이미 사용된 단어입니다.", input);
									continue;
								} else {
									input_words.push(input.clone());
									starting_char = Some(last_char);
								}
							}
						} else {
							println!("{}? 그런 건 없다.", input);
							continue;
						}
					} else {
						println!("{}? 그런 건 없다.", input);
						continue;
					}
				}
			}
		}
	}
}
