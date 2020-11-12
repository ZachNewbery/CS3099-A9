table! {
    Communities (id) {
        id -> Unsigned<Bigint>,
        uuid -> Text,
        description -> Text,
        title -> Text,
    }
}

table! {
    FederatedUsers (id) {
        id -> Unsigned<Bigint>,
        userId -> Unsigned<Bigint>,
        host -> Text,
    }
}

table! {
    LocalUsers (id) {
        id -> Unsigned<Bigint>,
        userId -> Unsigned<Bigint>,
        email -> Varchar,
        password -> Text,
        createdAt -> Timestamp,
    }
}

table! {
    Posts (id) {
        id -> Unsigned<Bigint>,
        uuid -> Text,
        title -> Text,
        author -> Unsigned<Bigint>,
        contentType -> Unsigned<Bigint>,
        body -> Text,
        created -> Timestamp,
        modified -> Timestamp,
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
