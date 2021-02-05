table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    Comments (id) {
        id -> Unsigned<Bigint>,
        post -> Unsigned<Bigint>,
        parent -> Nullable<Unsigned<Bigint>>,
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
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    Communities (id) {
        id -> Unsigned<Bigint>,
        name -> Text,
        description -> Text,
        title -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    CommunitiesUsers (id) {
        id -> Unsigned<Bigint>,
        communityId -> Unsigned<Bigint>,
        userId -> Unsigned<Bigint>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    FederatedUsers (id) {
        id -> Unsigned<Bigint>,
        userId -> Unsigned<Bigint>,
        host -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    LocalUsers (id) {
        id -> Unsigned<Bigint>,
        userId -> Unsigned<Bigint>,
        email -> Varchar,
        password -> Text,
        createdAt -> Timestamp,
        session -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    Markdown (id) {
        id -> Unsigned<Bigint>,
        content -> Text,
        postId -> Unsigned<Bigint>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    Posts (id) {
        id -> Unsigned<Bigint>,
        uuid -> Text,
        title -> Text,
        authorId -> Unsigned<Bigint>,
        contentType -> Enum,
        created -> Timestamp,
        modified -> Timestamp,
        parentId -> Nullable<Unsigned<Bigint>>,
        communityId -> Unsigned<Bigint>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    Text (id) {
        id -> Unsigned<Bigint>,
        content -> Text,
        postId -> Unsigned<Bigint>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    Users (id) {
        id -> Unsigned<Bigint>,
        username -> Varchar,
    }
}

joinable!(Comments -> Posts (post));
joinable!(Comments -> Users (author));
joinable!(CommunitiesUsers -> Communities (communityId));
joinable!(CommunitiesUsers -> Users (userId));
joinable!(FederatedUsers -> Users (userId));
joinable!(LocalUsers -> Users (userId));
joinable!(Markdown -> Posts (postId));
joinable!(Posts -> Communities (communityId));
joinable!(Posts -> Users (authorId));
joinable!(Text -> Posts (postId));

allow_tables_to_appear_in_same_query!(
    Comments,
    Communities,
    CommunitiesUsers,
    FederatedUsers,
    LocalUsers,
    Markdown,
    Posts,
    Text,
    Users,
);
