table! {
    Communities (id) {
        id -> Unsigned<Bigint>,
        uuid -> Varchar,
        description -> Varchar,
        title -> Varchar,
    }
}

table! {
    FederatedUsers (id) {
        id -> Unsigned<Bigint>,
        userId -> Unsigned<Bigint>,
        host -> Varchar,
    }
}

table! {
    LocalUsers (id) {
        id -> Unsigned<Bigint>,
        userId -> Unsigned<Bigint>,
        password -> Varchar,
    }
}

table! {
    Posts (id) {
        id -> Unsigned<Bigint>,
        uuid -> Varchar,
        title -> Varchar,
        author -> Unsigned<Bigint>,
        contentType -> Unsigned<Bigint>,
        body -> Varchar,
        created -> Datetime,
        modified -> Datetime,
    }
}

table! {
    Users (id) {
        id -> Unsigned<Bigint>,
        username -> Varchar,
    }
}

joinable!(FederatedUsers -> Users (userId));
joinable!(LocalUsers -> Users (userId));
joinable!(Posts -> Users (author));

allow_tables_to_appear_in_same_query!(Communities, FederatedUsers, LocalUsers, Posts, Users,);
