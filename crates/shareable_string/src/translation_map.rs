use crate::{ShareableString, SharedStringStore};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::hash::RandomState;
use std::sync::Arc;

/// A thread-safe map for storing translations of `ShareableString`s.
#[derive(Debug, Clone)]
pub struct SharedStringTranslationMap {
    /// The interning store used to deduplicate all keys, languages, and translations.
    store: SharedStringStore,
    /// Language code used when the requested language has no translation entry.
    fallback_language: ShareableString,
    /// Map from translation key to a per-language translation table, shared across clones.
    data: Arc<RwLock<HashMap<ShareableString, HashMap<ShareableString, ShareableString>>>>,
}

impl SharedStringTranslationMap {
    /// Creates a new `SharedStringTranslationMap` with the given fallback language being "en".
    #[hotpath::measure]
    #[must_use]
    pub fn new() -> Self {
        let store = SharedStringStore::new();
        let fallback_language = store.launder("en");

        Self {
            store,
            fallback_language,
            data: Arc::new(RwLock::new(HashMap::default())),
        }
    }

    /// Creates a new `SharedStringTranslationMap` with the given `SharedStringStore`.
    #[hotpath::measure]
    pub fn new_with_data<L>(store: SharedStringStore, fallback_language: L) -> Self
    where
        L: Into<ShareableString> + AsRef<str>,
    {
        let fallback_language = store.launder(fallback_language);

        Self {
            store,
            fallback_language,
            data: Arc::new(RwLock::new(HashMap::default())),
        }
    }

    /// Returns the translation for the given key and language if it exists.
    /// Will use the fallback language if the specified language is not found.
    /// If parameters are provided, they will be used to replace placeholders in the translation.
    #[hotpath::measure]
    pub fn get_translation<K, L>(
        &self,
        key: K,
        language: L,
        params: Option<&HashMap<ShareableString, ShareableString>>,
    ) -> Option<ShareableString>
    where
        K: AsRef<str>,
        L: AsRef<str>,
    {
        let translation;
        {
            let read_lock = self.data.read();
            translation = read_lock.get(key.as_ref()).and_then(|translations| {
                translations
                    .get(language.as_ref())
                    .or_else(|| translations.get(self.fallback_language.as_ref()))
                    .cloned()
            })?;
        }
        if let Some(params) = params {
            let mut result = translation.as_str().to_string();
            for (param_key, param_value) in params {
                let placeholder = format!("%{{{}}}", param_key.as_str());
                result = result.replace(&placeholder, param_value.as_str());
            }
            Some(self.store.launder(result))
        } else {
            Some(translation)
        }
    }

    /// Returns the fallback language.
    #[must_use]
    pub const fn get_fallback_language(&self) -> &ShareableString {
        &self.fallback_language
    }

    /// Sets the translation for the given key and language.
    ///
    /// The key, language, and translation are automatically laundered into the map's store.
    #[hotpath::measure]
    pub fn set_translation<K, L, T>(&self, key: K, language: L, translation: T)
    where
        K: Into<ShareableString> + AsRef<str>,
        L: Into<ShareableString> + AsRef<str>,
        T: Into<ShareableString> + AsRef<str>,
    {
        let interned_key = self.store.launder(key);
        let interned_lang = self.store.launder(language);
        let interned_translation = self.store.launder(translation);
        let mut write_lock = self.data.write();
        write_lock
            .entry(interned_key)
            .or_default()
            .insert(interned_lang, interned_translation);
    }

    /// Sets all translations for a given key.
    ///
    /// The key and all languages/translations in the data map are automatically laundered into the map's store.
    #[hotpath::measure]
    pub fn set_translation_key<K, K2, V2>(&self, key: K, data: HashMap<K2, V2>)
    where
        K: Into<ShareableString> + AsRef<str>,
        K2: Into<ShareableString> + AsRef<str> + Clone,
        V2: Into<ShareableString> + AsRef<str> + Clone,
    {
        let interned_key = self.store.launder(key);
        let mut interned_data =
            HashMap::with_capacity_and_hasher(data.len(), RandomState::default());
        for (lang, translation) in data {
            let interned_lang = self.store.launder(lang.clone());
            let interned_translation = self.store.launder(translation.clone());
            interned_data.insert(interned_lang, interned_translation);
        }
        let mut write_lock = self.data.write();
        write_lock.insert(interned_key, interned_data);
    }

    /// Merges another `SharedStringTranslationMap` into the current map.
    ///
    /// For each key-value pair in the provided map's data, it sets the key and its
    /// respective translations into the current map. The operation launders all keys
    /// and translations into the current map's store.
    #[hotpath::measure]
    pub fn insert_translation_map(&mut self, translation_map: &Self) {
        let read_lock = translation_map.data.read();
        for (key, translations) in read_lock.iter() {
            self.set_translation_key(key.clone(), translations.clone());
        }
    }
}

impl Default for SharedStringTranslationMap {
    fn default() -> Self {
        Self::new()
    }
}

/// A message that can be translated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslateMessage {
    /// The translation key identifying which message to look up.
    message_key: ShareableString,
    /// Named parameters substituted into the translated string at runtime.
    message_params: HashMap<ShareableString, ShareableString>,
}

impl TranslateMessage {
    /// Creates a new `TranslateMessage` with the given message key and parameters.
    #[must_use]
    pub const fn new(
        message_key: ShareableString,
        message_params: HashMap<ShareableString, ShareableString>,
    ) -> Self {
        TranslateMessage {
            message_key,
            message_params,
        }
    }

    /// Returns a reference to the message key.
    #[must_use]
    pub const fn message_key(&self) -> &ShareableString {
        &self.message_key
    }

    /// Returns a reference to the message parameters.
    #[must_use]
    pub const fn message_params(&self) -> &HashMap<ShareableString, ShareableString> {
        &self.message_params
    }

    /// Translates the message using the given translation map and language.
    #[must_use]
    #[hotpath::measure]
    pub fn translate<L>(
        &self,
        translation_map: &SharedStringTranslationMap,
        language: L,
    ) -> Option<ShareableString>
    where
        L: AsRef<str>,
    {
        translation_map.get_translation(&self.message_key, language, Some(&self.message_params))
    }

    /// Launders the message key and parameters using the provided `SharedStringStore`.
    #[must_use]
    #[hotpath::measure]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        TranslateMessage {
            message_key: store.launder(&self.message_key),
            message_params: self
                .message_params
                .iter()
                .map(|(k, v)| (store.launder(k), store.launder(v)))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_map_auto_launder() {
        let store = SharedStringStore::new();
        let map = SharedStringTranslationMap::new_with_data(store, "en");

        // Create strings that are NOT in the map's store
        let key = "key";
        let lang = "en";
        let translation = "hello";

        // Assert they are NOT in the store initially
        assert!(!map.store.contains("key"));
        assert!(map.store.contains("en")); // Fallback language is already in the store
        assert!(!map.store.contains("hello"));

        map.set_translation(key, lang, translation);

        // Now they should be in the store because set_translation launders
        assert!(map.store.contains("key"));
        assert!(map.store.contains("en"));
        assert!(map.store.contains("hello"));

        // And the data in the map should be using the interned instances
        let interned_key = map.store.get("key");
        let interned_lang = map.store.get("en");
        let interned_translation = map.store.get("hello");

        let retrieved_translation = map.get_translation("key", "en", None).unwrap();

        assert!(Arc::ptr_eq(
            retrieved_translation.as_arc(),
            interned_translation.as_arc()
        ));

        // Verify the keys in the map are also interned
        let read_lock = map.data.read();
        let (k, v) = read_lock.get_key_value(&interned_key).unwrap();
        assert!(Arc::ptr_eq(k.as_arc(), interned_key.as_arc()));

        let (l, t) = v.get_key_value(&interned_lang).unwrap();
        assert!(Arc::ptr_eq(l.as_arc(), interned_lang.as_arc()));
        assert!(Arc::ptr_eq(t.as_arc(), interned_translation.as_arc()));
    }

    #[test]
    fn test_translation_with_params() {
        let map = SharedStringTranslationMap::new();

        map.set_translation("greeting", "en", "Hello, %{name}!");

        let mut params = HashMap::new();
        params.insert(map.store.launder("name"), map.store.launder("Junie"));

        // Retrieve with parameters
        let translation = map
            .get_translation("greeting", "en", Some(&params))
            .unwrap();
        assert_eq!(translation.as_str(), "Hello, Junie!");
    }
}
