table! {
    use diesel::sql_types::*;
    use crate::models::enums::*;

    conlangs (id) {
        id -> Int4,
        name -> Text,
        native_name -> Nullable<Text>,
        registry_code -> Nullable<Varchar>,
        creators -> Nullable<Array<Int4>>,
        links -> Nullable<Array<Text>>,
        start_year -> Nullable<Timestamp>,
        physical_mode -> Physical_mode,
        scripts -> Nullable<Array<Int4>>,
        groups -> Nullable<Array<Int4>>,
        purpose -> Purpose_sub,
        vocabulary_source -> Vocabulary_source,
        development -> Development_level,
        notes -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::enums::*;

    groups (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::enums::*;

    persons (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::enums::*;

    scripts (id) {
        id -> Int4,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(conlangs, groups, persons, scripts,);
