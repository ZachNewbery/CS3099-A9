//! Database actions concerning Posts
use crate::database::actions::user::get_user_detail;
use crate::database::models::{
    DatabaseCommunity, DatabaseMarkdown, DatabaseNewPost, DatabasePost, DatabaseText, DatabaseUser,
};
use crate::federation::schemas::{ContentType, DatabaseContentType};
use std::collections::HashMap;

use diesel::prelude::*;
use diesel::BelongingToDsl;

use crate::util::UserDetail;
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

/// Returns all the top level posts stored in the Posts table
pub(crate) fn get_all_top_level_posts(
    conn: &MysqlConnection,
) -> Result<Option<Vec<DatabasePost>>, diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;
    Posts
        .filter(parentId.is_null()) // Only top level
        .load(conn)
        .optional()
}

/// Returns all posts in the Posts table
pub(crate) fn get_all_posts(
    conn: &MysqlConnection,
) -> Result<Option<Vec<DatabasePost>>, diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;
    Posts.load(conn).optional()
}

/// Returns all top level posts in a specified community in the Posts table
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

/// Obtains all the posts authored by a given user
pub(crate) fn get_posts_by_user(
    conn: &MysqlConnection,
    user: &DatabaseUser,
) -> Result<Option<Vec<DatabasePost>>, diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;
    Posts.filter(authorId.eq(user.id)).load(conn).optional()
}

/// Struct representing an unwrapped Post, containing all useful information
#[derive(Clone, Debug)]
pub struct PostInformation {
    /// Row belonging to the Post in the Posts table in the database
    pub post: DatabasePost,
    /// Content of the Post, stored as a Vector
    pub content: Vec<HashMap<ContentType, serde_json::Value>>,
    /// Community that the Post belongs to
    pub community: DatabaseCommunity,
    /// User who authored the Post
    pub user: DatabaseUser,
    /// Further details of the author
    pub user_details: UserDetail,
    /// Row belonging to the parent Post of the Post in the Posts table in the database
    pub parent: Option<DatabasePost>,
}

/// Retrieves a specified post from the Posts table given its UUID
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

/// Obtains the parent of a post given the post
pub(crate) fn get_parent_of(
    conn: &MysqlConnection,
    post: &DatabasePost,
) -> Result<Option<DatabasePost>, diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;
    if let Some(parent_id) = post.parent_id {
        Posts
            .filter(id.eq(parent_id))
            .first::<DatabasePost>(conn)
            .optional()
    } else {
        Ok(None)
    }
}

/// Obtains the children of a post given a post
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

/// Obtains an array of the content of a post given a post
pub(crate) fn get_content_of_post(
    conn: &MysqlConnection,
    post: &DatabasePost,
) -> Result<Vec<HashMap<ContentType, serde_json::Value>>, diesel::result::Error> {
    // We have to check through *every single* content type to pick up posts.
    let mut post_content: Vec<HashMap<ContentType, serde_json::Value>> = Vec::new();

    // Text
    {
        let t = DatabaseText::belonging_to(post)
            .first::<DatabaseText>(conn)
            .optional()?;
        if let Some(content) = t {
            let mut map = HashMap::new();
            map.insert(ContentType::Text, json!({ "text": content.content }));
            post_content.push(map)
        }
    }

    // Markdown
    {
        let m = DatabaseMarkdown::belonging_to(post)
            .first::<DatabaseMarkdown>(conn)
            .optional()?;
        if let Some(content) = m {
            let mut map = HashMap::new();
            map.insert(ContentType::Markdown, json!({ "text": content.content }));
            post_content.push(map)
        }
    }

    Ok(post_content)
}

/// Removes the content of a post from the database
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

/// Edits the title of a Post in the Posts table
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

/// Removes a post from the database
pub(crate) fn remove_post(
    conn: &MysqlConnection,
    post: DatabasePost,
) -> Result<(), diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;

    remove_post_contents(conn, &post)?;

    diesel::update(&post).set(deleted.eq(true)).execute(conn)?;

    Ok(())
}

/// Inserts a new post and content into the database
pub(crate) fn put_post(
    conn: &MysqlConnection,
    new_post: &DatabaseNewPost,
) -> Result<DatabasePost, diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;

    let uuid_ = new_post.uuid.clone();

    diesel::insert_into(Posts).values(new_post).execute(conn)?;

    Posts.filter(uuid.eq(uuid_)).first::<DatabasePost>(conn)
}

/// Inserts post content into the database
pub(crate) fn put_post_contents(
    conn: &MysqlConnection,
    post: &DatabasePost,
    contents: &[DatabaseContentType],
) -> Result<(), diesel::result::Error> {
    for content in contents {
        match content {
            DatabaseContentType::Text { text } => {
                use crate::database::schema::Text::dsl::*;
                diesel::insert_into(Text)
                    .values((content.eq(text), postId.eq(post.id)))
                    .execute(conn)?;
            }
            DatabaseContentType::Markdown { text } => {
                use crate::database::schema::Markdown::dsl::*;
                diesel::insert_into(Markdown)
                    .values((content.eq(text), postId.eq(post.id)))
                    .execute(conn)?;
            }
        }
    }

    Ok(())
}

/// Updates a post in the Posts table
pub(crate) fn touch_post(
    conn: &MysqlConnection,
    post: DatabasePost,
) -> Result<DatabasePost, diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;

    diesel::update(&post)
        .set(modified.eq(Utc::now().naive_utc()))
        .execute(conn)?;

    Posts.filter(id.eq(post.id)).first(conn)
}
