trait Config {
	type C;
}

#[derive(fabric_support::DebugNoBound)]
struct Foo<T: Config> {
	c: T::C,
}

fn main() {}
