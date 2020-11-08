#![allow(non_snake_case)]

pub mod models;
pub mod schema;

use self::models::*;
use crate::federation::schemas::NewPost;
use diesel::prelude::*;

#[allow(dead_code)]
pub(crate) fn create_federated_post(
    conn: &MysqlConnection,
    new_post: NewPost,
) -> Result<(), diesel::result::Error> {
    use schema::Posts;

    // TODO: Write database action to insert-or-get user, remove default
    if get_federated_user(&conn, &new_post.author.id, &new_post.author.host)?.is_none() {
        // Update both Users and FederatedUsers table.
        insert_federated_user(&conn, &new_post.author.id, &new_post.author.host)?;
    }

    let db_new_post = DBNewPost::from(new_post);

    conn.transaction::<(), diesel::result::Error, _>(|| {
        diesel::insert_into(Posts::table)
            .values(&db_new_post)
            .execute(conn)?;

        Ok(())
    })
}

pub(crate) fn get_federated_user(
    conn: &MysqlConnection,
    id_ck: &u64,
    host_ck: &str,
) -> Result<Option<FederatedUser>, diesel::result::Error> {
    use crate::database::schema::FederatedUsers::dsl::*;

    Ok(FederatedUsers
        .filter(userId.eq(id_ck).and(host.eq(host_ck)))
        .select(FederatedUsers::all_columns())
        .first::<FederatedUser>(conn)
        .optional()?)
}

pub(crate) fn insert_federated_user(
    conn: &MysqlConnection,
    id_ck: &u64,
    host_ck: &str,
) -> Result<(), diesel::result::Error> {
    conn.transaction::<(), diesel::result::Error, _>(|| {
        use crate::database::schema::FederatedUsers::dsl::*;
        use crate::database::schema::Users::dsl::*;

        // how do we get usernames from UserID struct?
        // TODO: replace username placeholder
        let db_new_user = DBNewUser {
            username: "placeholder".to_string(),
        };

        diesel::insert_into(Users)
            .values(db_new_user.clone())
            .execute(conn)?;

        let inserted_user: User = Users
            .filter(username.eq(&db_new_user.username))
            .first::<User>(conn)?;

        // TODO: Fix user id vs row id.
        let db_new_fed_user = DBNewFedUser {
            id: inserted_user.id,
            host: host_ck.to_string(),
        };

        diesel::insert_into(FederatedUsers)
            .values(db_new_fed_user)
            .execute(conn)?;

        Ok(())
    })
}

// #[allow(dead_code)]
// pub(crate) fn show_posts(_conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
//     use schema::Posts::dsl::*;
//
//     let result = Posts
//         .limit(5)
//         .load::<Post>(_conn)
//         .expect("Error Getting Posts.");
//
//     Ok(())
// }
