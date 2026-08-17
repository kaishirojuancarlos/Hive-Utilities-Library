use crate::hive::core::Immutable;

pub(crate) enum DevelopmentState
{
	Alpha,
	Beta,
	Release
}

pub(crate) const DEVELOPMENT_STAGE: Immutable<DevelopmentState> = Immutable::new(DevelopmentState::Alpha);
