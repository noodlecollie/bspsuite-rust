use std::env;

fn main()
{
	let args: Vec<String> = env::args().collect();

	// TODO: Call into bspcore
	for arg in args
	{
		println!("{arg}");
	}
}
