table! {
    Communities (id) {
        id -> Unsigned<Bigint>,
        uuid -> Varchar,
        descr -> Varchar,
        title -> Varchar,
    }
}

table! {
    Posts (id) {
        id -> Bigint,
        uuid -> Varchar,
        title -> Varchar,
        author -> Bigint,
        contType -> Varchar,
        body -> Varchar,
        created -> Date,
        modified -> Nullable<Date>,
    }
}

table! {
    Users (id) {
        id -> Unsigned<Bigint>,
        username -> Varchar,
    }
}

joinable!(Posts -> Users (author));

allow_tables_to_appear_in_same_query!(Communities, Posts, Users,);
