//! Registry of every built-in component registration, keyed by `(id, version)`.

include!(concat!(env!("OUT_DIR"), "/built_in_registry.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_components_contains() {
        let random = vec![("gain", 1u16)];
        for (id, version) in random {
            let registration = COMPONENTS
                .get(&(id, version))
                .unwrap_or_else(|| panic!("{id} version {version} should be registered"));
            let definition = registration.definition();
            assert_eq!(definition.id(), id);
            assert_eq!(definition.version(), version);
        }
    }

    #[test]
    fn component_keys_match_definition_ids() {
        for (&(id, version), &registration) in COMPONENTS.entries() {
            let definition = registration.definition();
            assert_eq!(definition.id(), id);
            assert_eq!(definition.version(), version);
        }
    }

    #[test]
    fn component_version_greater_than_zero() {
        for (&(id, version), &_definition) in COMPONENTS.entries() {
            assert!(version > 0, "version for {id} should be greater than zero");
        }
    }

    #[test]
    fn latest_returns_highest_version_per_id() {
        let keys = vec![("gain", 1u16)];
        assert_eq!(LATEST.len(), keys.len());

        for (id, version) in keys {
            let latest_registration = LATEST
                .get(id)
                .unwrap_or_else(|| panic!("latest definition for {id} should exist"));
            let latest_definition = latest_registration.definition();
            assert_eq!(latest_definition.version(), version);
        }
    }

    #[test]
    fn versions_lists_all_versions_per_id() {
        let versions = vec![("gain", vec![1u16])];
        assert_eq!(VERSIONS.len(), versions.len());

        for (id, expected_versions) in versions {
            let actual_versions = VERSIONS
                .get(id)
                .unwrap_or_else(|| panic!("versions for {id} should exist"));
            assert_eq!(actual_versions, &expected_versions);
        }
    }

    #[test]
    fn every_component_is_listed_in_versions() {
        for &(id, version) in COMPONENTS.keys() {
            let versions = VERSIONS
                .get(id)
                .unwrap_or_else(|| panic!("versions for {id} should exist"));
            assert!(
                versions.contains(&version),
                "{id} version {version} should be listed"
            );
        }
    }

    #[test]
    fn every_listed_version_has_a_component() {
        let listed_component_count: usize = VERSIONS.values().map(|versions| versions.len()).sum();
        assert_eq!(listed_component_count, COMPONENTS.len());

        for (&id, versions) in VERSIONS.entries() {
            for &version in *versions {
                assert!(
                    COMPONENTS.contains_key(&(id, version)),
                    "{id} version {version} should be registered"
                );
            }
        }
    }

    #[test]
    fn latest_matches_highest_listed_version() {
        assert_eq!(LATEST.len(), VERSIONS.len());

        for (&id, versions) in VERSIONS.entries() {
            let latest = *LATEST
                .get(id)
                .unwrap_or_else(|| panic!("latest definition for {id} should exist"));
            assert_eq!(Some(&latest.definition().version()), (*versions).last());
            assert!(
                COMPONENTS
                    .get(&(id, latest.definition().version()))
                    .is_some_and(|registration| std::ptr::eq(*registration, latest)),
                "latest definition for {id} should be registered"
            );
        }
    }

    #[test]
    fn registrations_instantiate_matching_components() {
        for (&(id, version), registration) in COMPONENTS.entries() {
            let instance = registration
                .instantiate()
                .unwrap_or_else(|_| panic!("{id} version {version} should instantiate"));
            assert_eq!(instance.definition().id(), id);
            assert_eq!(instance.definition().version(), version);
        }
    }

    #[test]
    fn instantiated_components_include_their_custom_functions() {
        let instance = COMPONENTS
            .get(&("gain", 1))
            .unwrap_or_else(|| panic!("gain version 1 should be registered"))
            .instantiate()
            .unwrap_or_else(|_| panic!("gain version 1 should instantiate"));

        let function = instance
            .engine()
            .functions()
            .get("gain")
            .unwrap_or_else(|| panic!("gain function should be registered"));
        assert_eq!(
            function.parameter_constraints(),
            &expression_engine::prelude::ArgumentCount::Exact { count: 2 }
        );
    }
}
