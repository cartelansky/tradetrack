
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 85,
                        patch: 0,
                        pre: vec![semver::Identifier::AlphaNumeric("nightly".to_owned()), ],
                        build: vec![],
                    },
                    host: "x86_64-pc-windows-msvc".to_owned(),
                    short_version_string: "rustc 1.85.0-nightly (45d11e51b 2025-01-01)".to_owned(),
                    commit_hash: Some("45d11e51bb66c2deb63a006fe3953c4b6fbc50c2".to_owned()),
                    commit_date: Some("2025-01-01".to_owned()),
                    build_date: None,
                    channel: Channel::Nightly,
                }
            }
            