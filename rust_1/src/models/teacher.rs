#[derive(Debug)]
pub struct Teacher<'a> {
	pub name: String,
	pub jobs: Vec<&'a str>,
}
