
use base64::{display::Base64Display, Engine};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(
    Debug,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    Eq,
    PartialEq,
    Hash,
    ToSchema,
)]
#[schema(value_type = String)]
pub struct UuidBase64Url(
    #[serde(serialize_with = "uuid_as_string_base_64_url", deserialize_with = "uuid_from_string_base_64_url")]
    uuid::Uuid
);

impl UuidBase64Url {
    pub fn new(id: uuid::Uuid) -> Self {
        Self(id)
    }

    pub fn new_random_id() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }

    pub fn for_debugging_only_zero() -> Self {
        Self(uuid::Uuid::nil())
    }
}

impl From<uuid::Uuid> for UuidBase64Url {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for UuidBase64Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayer = Base64Display::new(
            self.0.as_bytes(),
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
        );
        write!(f, "{}", displayer)
    }
}

pub fn uuid_as_string_base_64_url<
    S: Serializer,
>(
    value: &uuid::Uuid,
    s: S,
) -> Result<S::Ok, S::Error> {
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(value)
        .serialize(s)
}

pub fn uuid_from_string_base_64_url<
    'de,
    D: Deserializer<'de>,
>(
    d: D,
) -> Result<Uuid, D::Error> {
    let text  = <&'de str>::deserialize(d)?;
    let mut data_slice = [0u8; 16];
    let _ = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode_slice(text, &mut data_slice)
        .map_err(<D::Error as serde::de::Error>::custom)?;
    Ok(uuid::Uuid::from_bytes(data_slice))
}
