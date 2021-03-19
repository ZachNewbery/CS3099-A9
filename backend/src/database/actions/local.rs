use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::database::models::{DatabaseLocalUser, DatabaseUser};
use crate::internal::user::{EditProfile, NewLocalUser};

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

    LocalUsers
        .filter(id.eq(id_ck))
        .filter(session.eq(session_ck))
        .first::<DatabaseLocalUser>(conn)
        .optional()
}

// TODO: This is buggy for username xor email unique
pub(crate) fn get_local_user_by_username_email(
    conn: &MysqlConnection,
    username_: &str,
    email_: &str,
) -> Result<Option<DatabaseLocalUser>, diesel::result::Error> {
    use crate::database::schema::LocalUsers::dsl::*;
    use crate::database::schema::Users::dsl::*;

    Users
        .inner_join(LocalUsers)
        .filter(username.eq(username_).or(email.eq(email_)))
        .select(LocalUsers::all_columns())
        .first::<DatabaseLocalUser>(conn)
        .optional()
}

pub(crate) fn get_local_user_by_credentials(
    conn: &MysqlConnection,
    email_ck: &str,
    password_ck: &str,
) -> Result<Option<(DatabaseUser, DatabaseLocalUser)>, diesel::result::Error> {
    use crate::database::schema::LocalUsers::dsl::*;
    use crate::database::schema::Users::dsl::*;

    LocalUsers
        .filter(email.eq(email_ck))
        .filter(password.eq(password_ck))
        .inner_join(Users)
        .select((Users::all_columns(), LocalUsers::all_columns()))
        .first::<(_, _)>(conn)
        .optional()
}

pub(crate) fn insert_new_local_user(
    conn: &MysqlConnection,
    new_user: NewLocalUser,
) -> Result<(), diesel::result::Error> {
    use crate::database::models::{DatabaseNewLocalUser, DatabaseNewUser};
    use crate::database::schema::LocalUsers::dsl::*;
    use crate::database::schema::Users::dsl::*;

    let db_new_user: DatabaseNewUser = new_user.clone().into();

    diesel::insert_into(Users)
        .values(db_new_user.clone())
        .execute(conn)?;

    // Unfortunately MySQL does not support RETURN statements.
    // We will have to make a second query to fetch the new user id.
    let inserted_user: DatabaseUser = Users
        .filter(username.eq(&db_new_user.username))
        .first::<DatabaseUser>(conn)?;

    let db_new_local_user: DatabaseNewLocalUser = (inserted_user, new_user).into();

    diesel::insert_into(LocalUsers)
        .values(db_new_local_user)
        .execute(conn)?;

    Ok(())
}

pub(crate) fn update_local_user(
    conn: &MysqlConnection,
    user: DatabaseLocalUser,
    update_to: &EditProfile,
) -> Result<DatabaseLocalUser, diesel::result::Error> {
    use crate::database::schema::LocalUsers::dsl::*;

    let user_id = user.id;

    match &update_to.password {
        None => {}
        Some(t) => {
            diesel::update(&user)
                .set(password.eq(t.clone()))
                .execute(conn)?;
        }
    }

    match &update_to.bio {
        None => {}
        Some(t) => {
            diesel::update(&user).set(bio.eq(t.clone())).execute(conn)?;
        }
    }

    match &update_to.avatar {
        None => {}
        Some(t) => {
            diesel::update(&user)
                .set(avatar.eq(t.clone()))
                .execute(conn)?;
        }
    }

    LocalUsers
        .filter(id.eq(user_id))
        .first::<DatabaseLocalUser>(conn)
}
