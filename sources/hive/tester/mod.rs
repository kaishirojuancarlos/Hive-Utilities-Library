use crate::hive::core::{Immutable, Mutable};

pub fn launch_test()
{
	DEF.set(*ABC.get());
	println!(":: {}", ABC.get());
	println!(":: {}", DEF.get());
}

pub const ABC: Immutable<usize> = Immutable::new(2290);
static DEF: Mutable<usize> = Mutable::new(1445);
