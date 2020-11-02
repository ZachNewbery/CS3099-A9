table! {
    Communities (id) {
        id -> Unsigned<Bigint>,
        uuid -> Varchar,
        title -> Varchar,
    }
}

table! {
    Posts (id) {
        id -> Unsigned<Bigint>,
        uuid -> Varchar,
        title -> Varchar,
    }
}

table! {
    Users (id) {
        id -> Unsigned<Bigint>,
        username -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(Communities, Posts, Users,);
