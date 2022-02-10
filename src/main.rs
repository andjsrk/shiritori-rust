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
	let mut last_word_last_char: Option<char> = None;
	let mut used_words: Vec<String> = vec![];
	loop {
		input.clear();
		print!("단어 입력 > ");
		std::io::stdout().flush().unwrap(); // print!() does not flush stdout implicitly | https://github.com/rust-lang/rust/issues/23818
		let readline_result = std::io::stdin().read_line(&mut input);
		input = String::from(input.trim());
		if readline_result.is_ok() {
			println!("입력 받기 실패");
			continue;
		}
		if input.chars().count() < 2 {
			println!("단어의 길이는 2자 이상이어야 합니다.");
			continue;
		}
		if input == "exit()" {
			break;
		}
		println!("입력한 단어: {}", input);
		let first_char: char = input.chars().nth(0).unwrap();
		if last_word_last_char.is_some() && first_char != last_word_last_char.unwrap() {
			println!("{}는 {}로 시작하지 않습니다.", input, last_word_last_char.unwrap());
			continue;
		}
		let found_words = dict.get(&first_char);
		if !found_words.is_some() || !found_words.unwrap().contains(&&*input) {
			println!("{}? 그런 건 없다.", input);
			continue;
		}
		if used_words.contains(&input) {
			println!("{}은(는) 이미 사용된 단어입니다.", input);
			continue;
		}
		let last_char = input.chars().last().unwrap();
		let continuable_words = dict.get(&last_char);
		if continuable_words.is_none() {
			println!("와 한방단어");
			break;
		}
		used_words.push(input.clone());
		last_word_last_char = Some(last_char);
	}
}
