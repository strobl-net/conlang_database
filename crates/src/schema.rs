table! {
    conlangs (id) {
        id -> Int4,
        name -> Text,
        native_name -> Nullable<Text>,
        registry_code -> Nullable<Varchar>,
        creators -> Nullable<Array<Int4>>,
        links -> Nullable<Array<Text>>,
        start_year -> Nullable<Timestamp>,
        physical_mode -> Physicalmode,
        scripts -> Nullable<Array<Int4>>,
        groups -> Nullable<Array<Int4>>,
        purpose -> Purposesub,
        vocabulary_source -> Vocabularysource,
        development -> Developmentlevel,
        notes -> Nullable<Text>,
    }
}

table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    persons (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    scripts (id) {
        id -> Int4,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    conlangs,
    groups,
    persons,
    scripts,
);
