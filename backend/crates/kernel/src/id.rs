//! Identifiants typés en UUID v7 (triables par date de création).
//! Le crate qui déclare un identifiant doit dépendre de `serde`.

/// Déclare un identifiant fortement typé : `define_id!(StudentId);`
#[macro_export]
macro_rules! define_id {
    ($name:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            serde::Serialize,
            serde::Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name($crate::uuid::Uuid);

        impl $name {
            #[allow(clippy::new_without_default)]
            pub fn new() -> Self {
                Self($crate::uuid::Uuid::now_v7())
            }

            pub const fn from_uuid(value: $crate::uuid::Uuid) -> Self {
                Self(value)
            }

            pub const fn as_uuid(&self) -> &$crate::uuid::Uuid {
                &self.0
            }
        }

        $crate::__id_conversions!($name);
    };
}
