table! {
    Communities (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        description -> Text,
        title -> Text,
    }
}

table! {
    CommunitiesUsers (id) {
        id -> Unsigned<Bigint>,
        communityId -> Unsigned<Bigint>,
        userId -> Unsigned<Bigint>,
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
        session -> Varchar,
        bio -> Nullable<Text>,
        avatar -> Nullable<Text>,
    }
}

table! {
    Markdown (id) {
        id -> Unsigned<Bigint>,
        content -> Text,
        postId -> Unsigned<Bigint>,
    }
}

table! {
    Posts (id) {
        id -> Unsigned<Bigint>,
        uuid -> Varchar,
        title -> Nullable<Text>,
        authorId -> Unsigned<Bigint>,
        created -> Timestamp,
        modified -> Timestamp,
        parentId -> Nullable<Unsigned<Bigint>>,
        communityId -> Unsigned<Bigint>,
        deleted -> Bool,
    }
}

table! {
    Text (id) {
        id -> Unsigned<Bigint>,
        content -> Text,
        postId -> Unsigned<Bigint>,
    }
}

table! {
    Users (id) {
        id -> Unsigned<Bigint>,
        username -> Varchar,
    }
}

joinable!(CommunitiesUsers -> Communities (communityId));
joinable!(CommunitiesUsers -> Users (userId));
joinable!(FederatedUsers -> Users (userId));
joinable!(LocalUsers -> Users (userId));
joinable!(Markdown -> Posts (postId));
joinable!(Posts -> Communities (communityId));
joinable!(Posts -> Users (authorId));
joinable!(Text -> Posts (postId));

allow_tables_to_appear_in_same_query!(
    Communities,
    CommunitiesUsers,
    FederatedUsers,
    LocalUsers,
    Markdown,
    Posts,
    Text,
    Users,
);
