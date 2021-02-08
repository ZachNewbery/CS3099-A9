use crate::database::actions::user::get_user_detail;
use crate::database::models::{
    DatabaseCommunity, DatabaseFederatedUser, DatabaseLocalUser, DatabaseMarkdown, DatabaseNewPost,
    DatabasePost, DatabaseText, DatabaseUser,
};
use crate::database::schema::Text::dsl::Text;
use crate::federation::schemas::ContentType;
use actix_web::error::ReadlinesError::ContentTypeError;
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
        Vec<ContentType>,
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

    let content = get_content_of_post(conn, &post)?;

    let parent = get_parent_of(conn, &post)?;

    let user_detail = get_user_detail(conn, &user)?;

    Ok(Some((post, content, community, user, user_detail, parent)))
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
            Vec<ContentType>,
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
        .map(|(p, c, u)| {
            Ok((
                p,
                get_content_of_post(conn, &p)?,
                c,
                u,
                get_user_detail(conn, &u)?,
            ))
        })
        .collect::<Result<Vec<_>, diesel::result::Error>>()?;

    Ok(Some(children))
}

pub(crate) fn get_content_of_post(
    conn: &MysqlConnection,
    post: &DatabasePost,
) -> Result<Vec<ContentType>, diesel::result::Error> {
    // We have to check through *every single* content type to pick up posts.
    let mut post_content: Vec<ContentType> = Vec::new();

    // Text
    {
        use crate::database::schema::Text::dsl::*;
        post_content.append(
            &mut DatabaseText::belonging_to(post)
                .load::<DatabaseText>(conn)?
                .into_iter()
                .map(|t| ContentType::Text { text: t.content })
                .collect(),
        )
    }

    // Markdown
    {
        use crate::database::schema::Markdown::dsl::*;
        post_content.append(
            &mut DatabaseMarkdown::belonging_to(post)
                .load::<DatabaseMarkdown>(conn)?
                .into_iter()
                .map(|m| ContentType::Markdown { text: m.content })
                .collect(),
        )
    }

    Ok(post_content)

    // match post.content_type {
    //     DatabaseContentType::Text => {
    //         use crate::database::schema::Text::*;
    //         let text: DatabaseText = DatabaseText::belonging_to(post)
    //             .first::<DatabaseText>(conn)?;
    //
    //         Ok(ContentType::Text {
    //             text: text.content
    //         })
    //     }
    //     DatabaseContentType::Markdown => {
    //         use crate::database::schema::Markdown::*;
    //         let text: DatabaseMarkdown = DatabaseMarkdown::belonging_to(post)
    //             .first::<DatabaseMarkdown>(conn)?;
    //
    //         Ok(ContentType::Text {
    //             text: text.content
    //         })
    //     }
    // }
}
