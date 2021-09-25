use std::collections::HashMap;
use std::io::Write;

struct GameState {
	input: String,
	starting_char: Option<char>,
	used_words: Vec<String>
}

fn on_turn(dict: &HashMap<char, Vec<&str>>, state: GameState) {
	match state.input.clone() {
		mut input @ _ => (
			input.clear(),
			print!("단어 입력 > "),
			std::io::stdout().flush().unwrap(),
			match std::io::stdin().read_line(&mut input) {
				Ok(_) => (
					input = String::from(input.trim()),
					if input.chars().count() < 2 {
						(
							println!("단어의 길이는 2자 이상이어야 합니다."),
							on_turn(dict, state)
						).1
					} else {
						if input != "exit()" {
							(
								println!("입력한 단어: {}", input),
								match input.chars().nth(0).unwrap() {
									first_char @ _ =>
										if state.starting_char.is_some() && first_char != state.starting_char.unwrap() {
											(
												println!("{}는 {}로 시작하지 않습니다.", first_char, state.starting_char.unwrap()),
												on_turn(dict, state)
											).1
										} else {
											match dict.get(&first_char) {
												Some(found_words) =>
													if found_words.contains(&input.as_str()) {
														match input.chars().last().unwrap() {
															last_char @ _ => 
																match dict.get(&last_char) {
																	Some(_) =>
																		if state.used_words.contains(&input) {
																			(
																				println!("{}은(는) 이미 사용된 단어입니다.", input),
																				on_turn(dict, state)
																			).1
																		} else {
																			on_turn(dict, GameState { input: input.clone(), starting_char: Some(last_char), used_words: [ state.used_words, vec![ input.clone() ] ].concat() })
																		},
																	None => println!("와 한방단어")
																}
														}
													} else {
														(
															println!("{}? 그런 건 없다.", input),
															on_turn(dict, state)
														).1
													},
												None => (
													println!("{}? 그런 건 없다.", input),
													on_turn(dict, state)
												).1
											}
										}
								}
							).1
						}
					}
				).1,
				Err(_) => ()
			}
		).3
	}
}

fn main() {
	match
		[
			('가', vec![ "가나다", "가위" ]),
			('다', vec![ "다다", "다리미" ]),
			('위', vec![ "위기" ]),
		]
			.iter()
			.cloned()
			.collect::<HashMap<char, Vec<&str>>>()
	{
		dict @ _ =>
			match String::new() {
				input @ _ =>
					on_turn(&dict, GameState { input, starting_char: None, used_words: vec![] })
			}
	}
}
