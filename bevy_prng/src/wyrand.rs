use crate::{newtype::newtype_prng, SeedableEntropySource};

use bevy::prelude::{Reflect, ReflectFromReflect};
use rand_core::{RngCore, SeedableRng};

#[cfg(feature = "serialize")]
use bevy::prelude::{ReflectDeserialize, ReflectSerialize};

newtype_prng!(
    WyRand,
    ::wyrand::WyRand,
    "A newtyped [`wyrand::WyRand`] RNG",
    "wyrand"
);
