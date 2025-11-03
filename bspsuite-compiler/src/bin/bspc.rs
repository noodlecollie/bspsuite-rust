use bspcore;

fn main()
{
	bspcore::run_from_shell_arguments(&std::env::args().collect());
}
