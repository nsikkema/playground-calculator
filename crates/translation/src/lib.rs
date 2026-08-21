//! This crate provides a translation map for the whole program.

/// Add complex translations to the provided `SharedStringTranslationMap`.
pub(crate) mod complex;
/// Add simple translations to the provided `SharedStringTranslationMap`.
pub(crate) mod simple;

use crate::complex::add_complex_translations;
use crate::simple::add_simple_translations;
use shareable_string::{SharedStringStore, SharedStringTranslationMap};

/// Generates a `SharedStringTranslationMap` with both simple and complex translations added.
#[must_use]
pub fn generate_translation_map(store: &SharedStringStore) -> SharedStringTranslationMap {
    let fallback_language = store.launder("en");
    let mut translation_map =
        SharedStringTranslationMap::new_with_data(store.clone(), fallback_language);

    add_simple_translations(&mut translation_map);
    add_complex_translations(&mut translation_map);

    translation_map
}
