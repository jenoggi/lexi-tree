use std::io;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use substring::Substring;

#[derive(Default, Clone)]
enum Word {
	#[default]
	None,
	Leaf(String),
	Node(Box<WordNode>)
}

#[derive(Clone)]
struct WordNode {
	data: [Word; 64]
}

fn read_strings_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn char_to_index(c: char) -> usize {
	match c {
		'-' => {return 63;}
		'0'..='9' => {return ((c as u8) - ('0' as u8) + 1).into();}
		'a'..='z' => {return ((c as u8) - ('a' as u8) + 11).into();}
		'A'..='Z' => {return ((c as u8) - ('A' as u8) + 37).into();}
		_ => {}
	}
	return 0;
}

fn index_to_char(u: usize) -> char {
	match u {
		63 => {return '-'}
		1..=10 => {return char::from_u32(('0' as u32) + (u as u32) - 1).unwrap(); }
		11..=36 => {return char::from_u32(('a' as u32) + (u as u32) - 11).unwrap(); }
		37..=62 => {return char::from_u32(('A' as u32) + (u as u32) - 37).unwrap(); }
		_ => {}
	}
	return ' ';
}

impl WordNode {
    fn default() -> WordNode {
        WordNode {
			data: [const { Word::None }; 64],
        }
    }
	
	fn add_word_to_tree(&mut self, s: &mut String) {
		//need to take care if last char in string!!
		//println!("{}",s);
		if s.is_empty() {
			// last character
			self.data[0] = Word::Leaf(String::new());
		} else {
			let i = char_to_index(s.chars().nth(0).unwrap());
			s.remove(0);
			let nn;
			match self.data[i] {
				Word::None => {nn = Word::Leaf(s.to_string());}
				Word::Leaf(ref mut s2) => {
					let mut w: WordNode = WordNode::default();
					w.add_word_to_tree(s2);
					w.add_word_to_tree(s);
					nn = Word::Node(Box::new(w));
				}
				Word::Node(ref mut bw) => {
					bw.add_word_to_tree(s);
					return;
				}
			}
			self.data[i] = nn;
		}
	}
	
	fn print_string(&self, s: String) {
		for w in 0..64 {
			match self.data[w] {
				Word::None => {}
				Word::Leaf(ref s2) => {
					println!("{}{}{}",s,index_to_char(w),s2);
				}
				Word::Node(ref bw) => {
					let mut ts = s.clone();
					ts.push(index_to_char(w));
					bw.print_string(ts);
				}
			}
		}
	}
	
	fn find_words(&self, search_for: &mut String, s: &mut String) {
		if  search_for.is_empty() {
			self.print_string(s.to_string());
		} else {
			let c = search_for.chars().nth(0).unwrap();
			s.push(c);
			let i = char_to_index(c);
			search_for.remove(0);
			
			match self.data[i] {
				Word::None => {}
				Word::Leaf(ref s2) => {
					if s2.chars().count() >= search_for.chars().count() {
						if s2.substring(0, search_for.chars().count()) == search_for {
							println!("{}{}",s,s2);
						}
					}
				}
				Word::Node(ref bw) => {
					bw.find_words(search_for, s);
				}
			}
		}
	}
} 

fn main() {
	let iv = read_strings_from_file("keywords.txt").unwrap();
	
	let mut word_root = WordNode::default();
	
	for mut s in iv {
		word_root.add_word_to_tree(&mut s);
	}
	
	word_root.print_string("".to_string());
	println!();
	
	loop {
		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
			Ok(..) => {
				let mut s2 = "".to_string();
				word_root.find_words(&mut input.trim().to_string(), &mut s2);
				println!("");
			}
			Err(error) => println!("error: {error}"),
		}
	}
}
