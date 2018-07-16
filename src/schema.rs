table! {
    countries (short_name) {
        full_name -> Varchar,
        continent -> Varchar,
        short_name -> Varchar,
    }
}

table! {
    products (id) {
        id -> Varchar,
        name -> Varchar,
        price -> Float8,
        description -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    countries,
    products,
);
