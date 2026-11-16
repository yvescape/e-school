/// Conversions texte des identifiants (usage interne de `define_id!`).
#[doc(hidden)]
#[macro_export]
macro_rules! __id_conversions {
    ($name:ident) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl ::std::convert::From<$name> for String {
            fn from(id: $name) -> Self {
                id.0.to_string()
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::uuid::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $crate::uuid::Uuid::parse_str(s).map(Self)
            }
        }
    };
}
