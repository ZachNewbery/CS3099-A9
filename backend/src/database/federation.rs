use crate::database::models::FederatedUser;
use crate::federation::schemas::NewPost;
use diesel::prelude::*;
use diesel::{Connection, MysqlConnection, RunQueryDsl};

pub(crate) fn create_federated_post(
    conn: &MysqlConnection,
    new_post: NewPost,
) -> Result<(), diesel::result::Error> {
    use crate::database::models::DBNewPost;
    use crate::database::schema::Posts;

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
    username_ck: &str,
    host_ck: &str,
) -> Result<Option<FederatedUser>, diesel::result::Error> {
    use crate::database::schema::FederatedUsers::dsl::*;
    use crate::database::schema::Users::dsl::*;

    Ok(Users
        .inner_join(FederatedUsers)
        .filter(username.eq(username_ck))
        .filter(host.eq(host_ck))
        .select(FederatedUsers::all_columns())
        .first::<FederatedUser>(conn)
        .optional()?)
}

pub(crate) fn insert_federated_user(
    conn: &MysqlConnection,
    id_ck: &str,
    host_ck: &str,
) -> Result<(), diesel::result::Error> {
    conn.transaction::<(), diesel::result::Error, _>(|| {
        use crate::database::models::{DBNewFedUser, DBNewUser, User};
        use crate::database::schema::FederatedUsers::dsl::*;
        use crate::database::schema::Users::dsl::*;

        let db_new_user = DBNewUser {
            username: id_ck.to_string(),
        };

        diesel::insert_into(Users)
            .values(db_new_user.clone())
            .execute(conn)?;

        let inserted_user: User = Users
            .filter(username.eq(&db_new_user.username))
            .first::<User>(conn)?;

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
