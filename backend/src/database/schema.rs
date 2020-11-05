table! {
    Communities (id) {
        id -> Unsigned<Bigint>,
        uid -> Varchar,
        title -> Varchar,
    }
}

table! {
    Posts (id) {
        id -> Unsigned<Bigint>,
        uuid -> Varchar,
        title -> Varchar,
        author -> Nullable<Unsigned<Bigint>>,
        contType -> Nullable<Varchar>,
        body -> Varchar,
        created -> Date,
        modified -> Nullable<Date>,
    }
}

table! {
    Users (id) {
        id -> Unsigned<Bigint>,
        username -> Nullable<Varchar>,
    }
}

joinable!(Posts -> Users (author));

allow_tables_to_appear_in_same_query!(Communities, Posts, Users,);
