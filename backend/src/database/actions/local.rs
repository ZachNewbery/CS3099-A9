use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::database::models::{DatabaseLocalUser, DatabasePost};
use crate::internal::{LocalNewPost, NewUser};

// FIXME: This is here for MVP purposes
pub(crate) fn create_local_post(
    conn: &MysqlConnection,
    new_post: LocalNewPost,
    local_user: DatabaseLocalUser,
) -> Result<(), diesel::result::Error> {
    use crate::database::models::DatabaseNewPost;
    use crate::database::schema::Posts::dsl::*;
    use chrono::{NaiveDateTime, Utc};

    let db_new_post = DatabaseNewPost {
        uuid: ::uuid::Uuid::new_v4().to_string(),
        title: new_post.title,
        body: new_post.body,
        author: local_user.user_id,
        content_type: 0,
        created: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        modified: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
    };

    diesel::insert_into(Posts)
        .values(db_new_post)
        .execute(conn)?;

    Ok(())
}

pub(crate) fn update_session(
    conn: &MysqlConnection,
    user: &DatabaseLocalUser,
    new_session: &str,
) -> Result<(), diesel::result::Error> {
    use crate::database::schema::LocalUsers::dsl::*;

    diesel::update(LocalUsers.filter(id.eq(user.id)))
        .set(session.eq(new_session))
        .execute(conn)?;
    Ok(())
}

pub(crate) fn validate_session(
    conn: &MysqlConnection,
    id_ck: u64,
    session_ck: &str,
) -> Result<Option<DatabaseLocalUser>, diesel::result::Error> {
    use crate::database::schema::LocalUsers::dsl::*;

    Ok(LocalUsers
        .filter(id.eq(id_ck))
        .filter(session.eq(session_ck))
        .first::<DatabaseLocalUser>(conn)
        .optional()?)
}

pub(crate) fn get_local_user(
    conn: &MysqlConnection,
    username_ck: &str,
    email_ck: &str,
) -> Result<Option<DatabaseLocalUser>, diesel::result::Error> {
    use crate::database::schema::LocalUsers::dsl::*;
    use crate::database::schema::Users::dsl::*;

    Ok(Users
        .inner_join(LocalUsers)
        .filter(username.eq(username_ck))
        .filter(email.eq(email_ck))
        .select(LocalUsers::all_columns())
        .first::<DatabaseLocalUser>(conn)
        .optional()?)
}

// FIXME: I cannot emphasize just how insecure this is. MUST fix before pushing to production
pub(crate) fn login_local_user(
    conn: &MysqlConnection,
    email_ck: &str,
    password_ck: &str,
) -> Result<Option<DatabaseLocalUser>, diesel::result::Error> {
    use crate::database::schema::LocalUsers::dsl::*;

    Ok(LocalUsers
        .filter(email.eq(email_ck))
        .filter(password.eq(password_ck))
        .first::<DatabaseLocalUser>(conn)
        .optional()?)
}

pub(crate) fn insert_new_local_user(
    conn: &MysqlConnection,
    new_user: NewUser,
) -> Result<(), diesel::result::Error> {
    conn.transaction::<(), diesel::result::Error, _>(|| {
        use crate::database::models::{DatabaseNewLocalUser, DatabaseNewUser, DatabaseUser};
        use crate::database::schema::LocalUsers::dsl::*;
        use crate::database::schema::Users::dsl::*;

        let db_new_user: DatabaseNewUser = new_user.clone().into();

        diesel::insert_into(Users)
            .values(db_new_user.clone())
            .execute(conn)?;

        // Unfortunately MySQL does not support RETURN statements.
        // We will have to make a second query to fetch the new user id.
        // TODO: Look into extracting function
        let inserted_user: DatabaseUser = Users
            .filter(username.eq(&db_new_user.username))
            .first::<DatabaseUser>(conn)?;

        let db_new_local_user: DatabaseNewLocalUser = (inserted_user, new_user).into();

        diesel::insert_into(LocalUsers)
            .values(db_new_local_user)
            .execute(conn)?;

        Ok(())
    })
}
