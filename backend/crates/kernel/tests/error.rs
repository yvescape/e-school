use kernel::{DomainError, FieldError};

#[test]
fn not_found_names_entity_and_id() {
    let err = DomainError::not_found("student", "42");
    assert_eq!(err.to_string(), "student 42 introuvable");
}

#[test]
fn validation_keeps_every_field_error() {
    let err = DomainError::Validation(vec![
        FieldError::new("email", "required", "L'email est obligatoire"),
        FieldError::new("name", "too_long", "Nom trop long"),
    ]);
    let DomainError::Validation(fields) = &err else {
        panic!("expected validation error");
    };
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].field, "email");
    assert_eq!(fields[1].code, "too_long");
}

#[test]
fn field_error_serializes_its_three_parts() {
    let json = serde_json::to_value(FieldError::new("a", "b", "c")).unwrap();
    assert_eq!(
        json,
        serde_json::json!({"field": "a", "code": "b", "message": "c"})
    );
}

#[test]
fn conflict_and_internal_carry_their_message() {
    assert_eq!(
        DomainError::Conflict("doublon".into()).to_string(),
        "conflit : doublon"
    );
    assert_eq!(
        DomainError::Internal("db".into()).to_string(),
        "erreur interne : db"
    );
}
