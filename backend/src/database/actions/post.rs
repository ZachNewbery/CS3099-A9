use crate::database::actions::user::get_user_detail;
use crate::database::models::{
    DatabaseCommunity, DatabaseFederatedUser, DatabaseLocalUser, DatabaseMarkdown, DatabaseNewPost,
    DatabasePost, DatabaseText, DatabaseUser,
};
use crate::federation::schemas::ContentType;

use diesel::prelude::*;
use diesel::BelongingToDsl;
use either::Either;

use uuid::Uuid;

pub(crate) fn get_all_top_level_posts(
    conn: &MysqlConnection,
) -> Result<Option<Vec<DatabasePost>>, diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;
    Posts
        .filter(parentId.is_null()) // Only top level
        .load(conn)
        .optional()
}

pub(crate) fn get_all_posts(
    conn: &MysqlConnection,
) -> Result<Option<Vec<DatabasePost>>, diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;
    Posts.load(conn).optional()
}

pub(crate) fn get_top_level_posts_of_community(
    conn: &MysqlConnection,
    community: &DatabaseCommunity,
) -> Result<Option<Vec<DatabasePost>>, diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;
    Posts
        .filter(communityId.eq(community.id))
        .filter(parentId.is_null()) // Only top level
        .load(conn)
        .optional()
}

#[derive(Clone, Debug)]
pub struct PostInformation {
    pub post: DatabasePost,
    pub content: Vec<ContentType>,
    pub community: DatabaseCommunity,
    pub user: DatabaseUser,
    pub user_details: Either<DatabaseLocalUser, DatabaseFederatedUser>,
    pub parent: Option<DatabasePost>,
}

pub(crate) fn get_post(
    conn: &MysqlConnection,
    post_uuid: &Uuid,
) -> Result<Option<PostInformation>, diesel::result::Error> {
    use crate::database::schema::Communities::dsl::*;

    use crate::database::schema::Posts::dsl::*;
    use crate::database::schema::Users::dsl::*;

    let (post, community, user) = match Posts
        .filter(uuid.eq(post_uuid.to_string()))
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

    let user_details = get_user_detail(conn, &user)?;

    Ok(Some(PostInformation {
        post,
        content,
        community,
        user,
        user_details,
        parent,
    }))
}

pub(crate) fn get_parent_of(
    conn: &MysqlConnection,
    post: &DatabasePost,
) -> Result<Option<DatabasePost>, diesel::result::Error> {
    DatabasePost::belonging_to(post)
        .first::<DatabasePost>(conn)
        .optional()
}

pub(crate) fn get_children_posts_of(
    conn: &MysqlConnection,
    parent: &DatabasePost,
) -> Result<Option<Vec<PostInformation>>, diesel::result::Error> {
    use crate::database::schema::Communities::dsl::*;

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
            Ok(PostInformation {
                post: p.clone(),
                content: get_content_of_post(conn, &p)?,
                community: c,
                user: u.clone(),
                user_details: get_user_detail(conn, &u)?,
                parent: Some(parent.clone()),
            })
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
        post_content.append(
            &mut DatabaseMarkdown::belonging_to(post)
                .load::<DatabaseMarkdown>(conn)?
                .into_iter()
                .map(|m| ContentType::Markdown {
                    markdown: m.content,
                })
                .collect(),
        )
    }

    Ok(post_content)
}

pub(crate) fn remove_post_contents(
    conn: &MysqlConnection,
    post: &DatabasePost,
) -> Result<(), diesel::result::Error> {
    // We have to check through *every single* content type to delete posts.

    // Text
    {
        use crate::database::schema::Text::dsl::*;
        diesel::delete(Text.filter(postId.eq(post.id))).execute(conn)?;
    }

    // Markdown
    {
        use crate::database::schema::Markdown::dsl::*;
        diesel::delete(Markdown.filter(postId.eq(post.id))).execute(conn)?;
    }

    Ok(())
}

pub(crate) fn modify_post_title(
    conn: &MysqlConnection,
    post: DatabasePost,
    new_title: &str,
) -> Result<DatabasePost, diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;

    let id_ = post.id;

    diesel::update(&post)
        .set(title.eq(new_title.to_string()))
        .execute(conn)?;

    Posts.filter(id.eq(id_)).first::<DatabasePost>(conn)
}

pub(crate) fn remove_post(
    conn: &MysqlConnection,
    post: DatabasePost,
) -> Result<(), diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;

    remove_post_contents(conn, &post)?;

    diesel::update(&post).set(deleted.eq(true)).execute(conn)?;

    Ok(())
}

pub(crate) fn put_post(
    conn: &MysqlConnection,
    new_post: &DatabaseNewPost,
) -> Result<DatabasePost, diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;

    let uuid_ = new_post.uuid.clone();

    diesel::insert_into(Posts).values(new_post).execute(conn)?;

    Posts.filter(uuid.eq(uuid_)).first::<DatabasePost>(conn)
}

pub(crate) fn put_post_contents(
    conn: &MysqlConnection,
    post: &DatabasePost,
    contents: &[ContentType],
) -> Result<(), diesel::result::Error> {
    for content in contents {
        match content {
            ContentType::Text { text } => {
                use crate::database::schema::Text::dsl::*;
                diesel::insert_into(Text)
                    .values((content.eq(text), postId.eq(post.id)))
                    .execute(conn)?;
            }
            ContentType::Markdown { markdown } => {
                use crate::database::schema::Markdown::dsl::*;
                diesel::insert_into(Markdown)
                    .values((content.eq(markdown), postId.eq(post.id)))
                    .execute(conn)?;
            }
        }
    }
    Ok(())
}
