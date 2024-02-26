// @generated automatically by Diesel CLI.

diesel::table! {
    dog_facts (id) {
        id -> Int4,
        fact -> Varchar,
    }
}

diesel::table! {
    invitations (invitation_token) {
        invitation_token -> Uuid,
        email -> Text,
        used -> Bool,
        expires_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    dog_facts,
    invitations,
);
