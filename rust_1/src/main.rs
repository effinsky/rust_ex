mod models;

use crate::models::teacher::Teacher;
use std::fmt::Debug;

fn main() {
	let extern_var = 666;
	// this creates a closure(#1) that returns another closure(#2) with arguments built-int,
	// and also uses external var
	let local_fn = |base: String, times: i32| -> Box<dyn Fn() -> String> {
		println!("printing external var: {}", extern_var);
		Box::new(move || -> String { base.repeat(times as usize) })
	};

	// this creates a closure(#2) from by calling the other closure(#1)
	let base_quadrupler = local_fn("nothingness".to_owned(), 4);
	println!("quadrupled base: {}", base_quadrupler());

	// this, in turn, creates another closure(#3) -- one which takes a func as an argument
	let another_local_fn = |local_fn: &dyn Fn() -> String| {
		local_fn();
	};

	// and this calls that closure and supplies the closure received from the original closure
	// as an arg to closure#3
	another_local_fn(&base_quadrupler);

	let jobs = vec!["pimp", "broser", "organ player"];
	let t1 = Teacher {
		name: "Carlito".to_owned(),
		jobs,
	};

	println!("my first teacher: {:?}", t1);

	let new_change = Change::default();
	let _ = new_change;

	let _ = String::from("nuthin"); // where is this shit going?
	generic_func(432);

	let list = vec![
		"nothingness".to_owned(),
		"somethingness".to_owned(),
		"everythingness".to_owned(),
	];
	let total_len = get_total_strings_len(list);
	println!("total length: {}", total_len);
}

fn generic_func<T>(x: T)
where
	T: Clone + Debug,
{
	println!("printing in generic func {:?}", x);
}

pub fn spellcheck<C: SpellChecker>(input: &str, spellchecker: C) -> String {
	let mut result = input.to_owned();
	for change in spellchecker.check(input) {
		apply_change(&mut result, change);
	}
	result
}

pub fn spellcheck2(input: &str, spellchecker: Box<dyn SpellChecker>) -> String {
	let mut result = input.to_owned();
	for change in spellchecker.check(input) {
		apply_change(&mut result, change);
	}
	result
}
pub trait SpellChecker {
	fn check(&self, input: &str) -> Vec<Change>;
}

pub struct Change;

impl Default for Change {
	fn default() -> Self {
		println!("instantiating change struct");
		Self {}
	}
}

fn apply_change(_: &mut String, _: Change) {}

fn get_total_strings_len(list: Vec<String>) -> i32 {
	list.into_iter()
		.map(|v| v.len())
		.fold(0, |acc, v| acc + v as i32)
}
