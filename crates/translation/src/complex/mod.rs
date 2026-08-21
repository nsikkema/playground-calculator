/// This module contains the expression engine translation map.
pub(crate) mod expression_engine;

use crate::complex::expression_engine::add_expression_engine_translation_map;
use shareable_string::SharedStringTranslationMap;

/// Adds complex translations to the provided `SharedStringTranslationMap`.
pub(crate) fn add_complex_translations(translation_map: &mut SharedStringTranslationMap) {
    add_expression_engine_translation_map(translation_map);
}
