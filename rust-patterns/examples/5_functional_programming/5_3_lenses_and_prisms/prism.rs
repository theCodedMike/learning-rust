/// ```rust
/// pub trait Deserializer<'de>: Sized {
///     type Error: Error;
///
///     fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
///     where
///         V: Visitor<'de>;
///
///     fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
///     where
///         V: Visitor<'de>;
///
///     // remainder ommitted
/// }
/// ```

/// ```rust
/// pub trait Visitor<'de>: Sized {
///     type Value;
///
///     fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
///     where
///         E: Error;
///
///     fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
///     where
///         E: Error;
///
///     fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
///     where
///         E: Error;
///
///     // remainder omitted
/// }
/// ```