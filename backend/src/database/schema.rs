table! {
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
    Communities (id) {
        id -> Unsigned<Bigint>,
        name -> Text,
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
        uuid -> Text,
        title -> Text,
        authorId -> Unsigned<Bigint>,
        created -> Timestamp,
        modified -> Timestamp,
        parentId -> Nullable<Unsigned<Bigint>>,
        communityId -> Unsigned<Bigint>,
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
