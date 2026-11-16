use kernel::define_id;
use std::str::FromStr;

define_id!(SampleId);

#[test]
fn new_ids_are_unique_and_time_ordered() {
    let first = SampleId::new();
    let second = SampleId::new();
    assert_ne!(first, second);
    assert!(first < second, "UUID v7 must sort by creation time");
}

#[test]
fn id_uses_uuid_version_7() {
    assert_eq!(SampleId::new().as_uuid().get_version_num(), 7);
}

#[test]
fn id_roundtrips_through_string() {
    let id = SampleId::new();
    let parsed = SampleId::from_str(&id.to_string()).unwrap();
    assert_eq!(id, parsed);
}

#[test]
fn invalid_string_is_rejected() {
    assert!(SampleId::from_str("pas-un-uuid").is_err());
}

#[test]
fn id_serializes_as_plain_string() {
    let id = SampleId::new();
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, format!("\"{id}\""));
    let back: SampleId = serde_json::from_str(&json).unwrap();
    assert_eq!(back, id);
}
