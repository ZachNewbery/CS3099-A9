use crate::database::actions::user::get_user_detail;
use crate::database::models::{
    DatabaseCommunity, DatabaseFederatedUser, DatabaseLocalUser, DatabaseNewPost, DatabasePost,
    DatabaseUser,
};
use diesel::prelude::*;
use diesel::BelongingToDsl;
use either::Either;
use either::Either::{Left, Right};
use uuid::Uuid;

pub(crate) fn get_posts_of_community(
    conn: &MysqlConnection,
    community: &DatabaseCommunity,
) -> Result<Option<Vec<DatabasePost>>, diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;
    Posts
        .filter(communityId.eq(community.id))
        .load(conn)
        .optional()
}

pub(crate) fn add_federated_post(
    conn: &MysqlConnection,
    new_post: DatabaseNewPost,
) -> Result<(), diesel::result::Error> {
    todo!()
}

pub(crate) fn get_post(
    conn: &MysqlConnection,
    uuid_: &Uuid,
) -> Result<
    Option<(
        DatabasePost,
        DatabaseCommunity,
        DatabaseUser,
        Either<DatabaseLocalUser, DatabaseFederatedUser>,
        DatabasePost, // Parent
    )>,
    diesel::result::Error,
> {
    use crate::database::schema::Communities::dsl::*;
    use crate::database::schema::LocalUsers::dsl::*;
    use crate::database::schema::Posts::dsl::*;
    use crate::database::schema::Users::dsl::*;

    let (post, community, user) = match Posts
        .filter(uuid.eq(uuid_.to_string()))
        .inner_join(Users)
        .inner_join(Communities)
        .select((
            Posts::all_columns(),
            Communities::all_columns(),
            Users::all_columns(),
        ))
        .first::<(_, _, _)>(conn)
        .optional()?
    {
        None => return Ok(None),
        Some(t) => t,
    };

    let parent = get_parent_of(conn, &post)?;

    let user_detail = get_user_detail(conn, &user)?;

    Ok(Some((post, community, user, user_detail, parent)))
}

pub(crate) fn get_parent_of(
    conn: &MysqlConnection,
    post: &DatabasePost,
) -> Result<DatabasePost, diesel::result::Error> {
    DatabasePost::belonging_to(post).first::<DatabasePost>(conn)
}

pub(crate) fn get_children_posts_of(
    conn: &MysqlConnection,
    parent: &DatabasePost,
) -> Result<
    Option<
        Vec<(
            DatabasePost,
            DatabaseCommunity,
            DatabaseUser,
            Either<DatabaseLocalUser, DatabaseFederatedUser>,
        )>,
    >,
    diesel::result::Error,
> {
    use crate::database::schema::Communities::dsl::*;
    use crate::database::schema::LocalUsers::dsl::*;
    use crate::database::schema::Posts::dsl::*;
    use crate::database::schema::Users::dsl::*;

    let children: Vec<(DatabasePost, DatabaseCommunity, DatabaseUser)> =
        match DatabasePost::belonging_to(parent)
            .inner_join(Users)
            .inner_join(Communities)
            .select((
                Posts::all_columns(),
                Communities::all_columns(),
                Users::all_columns(),
            ))
            .load::<(_, _, _)>(conn)
            .optional()?
        {
            None => return Ok(None),
            Some(t) => t,
        };

    let children = children
        .into_iter()
        .map(|(p, c, u)| Ok((p, c, u, get_user_detail(conn, &u)?)))
        .collect::<Result<Vec<_>, diesel::result::Error>>()?;

    Ok(Some(children))
}
