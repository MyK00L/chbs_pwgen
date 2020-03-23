
const WORDS_PER_DIC: usize = 2048;
const WORDS_IN_PW: usize = 4;
const MIN_WORD_LEN: usize = 5;

fn add_words(filename: &str, min_len: usize, n: usize, v: &mut Vec<String>) -> () {
	v.extend(std::fs::read_to_string(filename).unwrap().split('\n').map(|s| s.split(' ').collect::<Vec<&str> >()[0].to_string()).filter(|s| s.len()>=min_len).take(n));
}

fn r(m: usize) -> usize {
	rand::random::<usize>()%m
}

fn uppercase_first_letter_stackoverflow(s: &str) -> String {
	let mut c = s.chars();
	match c.next() {
		None => String::new(),
				 Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
	}
}

fn main() {
	let mut v = Vec::<String>::new();
	add_words("it_50k.txt", MIN_WORD_LEN, WORDS_PER_DIC, &mut v);
	add_words("en_50k.txt", MIN_WORD_LEN, WORDS_PER_DIC, &mut v);
	for _ in 0..WORDS_IN_PW {
		print!("{}",uppercase_first_letter_stackoverflow(&v[r(v.len())]));
	}
	print!("\n");
}

