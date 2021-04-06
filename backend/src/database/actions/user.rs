use crate::database::models::{DatabaseFederatedUser, DatabaseLocalUser, DatabaseUser};

use crate::federation::schemas::User;
use crate::util::UserDetail;
use diesel::prelude::*;
use diesel::MysqlConnection;

pub(crate) fn get_user_detail(
    conn: &MysqlConnection,
    user: &DatabaseUser,
) -> Result<UserDetail, diesel::result::Error> {
    let local: Option<DatabaseLocalUser> = DatabaseLocalUser::belonging_to(user)
        .first::<DatabaseLocalUser>(conn)
        .optional()?;

    let fed: Option<DatabaseFederatedUser> = DatabaseFederatedUser::belonging_to(user)
        .first::<DatabaseFederatedUser>(conn)
        .optional()?;

    match (local, fed) {
        (None, None) => Err(diesel::NotFound),
        (Some(l), _) => Ok(l.into()),
        (_, Some(f)) => Ok(f.into()),
    }
}

pub(crate) fn insert_new_federated_user(
    conn: &MysqlConnection,
    new_user: User,
) -> Result<(), diesel::result::Error> {
    use crate::database::models::{DatabaseNewFederatedUser, DatabaseNewUser};
    use crate::database::schema::FederatedUsers::dsl::*;
    use crate::database::schema::Users::dsl::*;

    let db_new_user: DatabaseNewUser = new_user.clone().into();

    diesel::insert_into(Users)
        .values(db_new_user.clone())
        .execute(conn)?;

    let inserted_user: DatabaseUser = Users
        .filter(username.eq(&db_new_user.username))
        .first::<DatabaseUser>(conn)?;

    let db_new_fed_user: DatabaseNewFederatedUser = (inserted_user, new_user).into();

    diesel::insert_into(FederatedUsers)
        .values(db_new_fed_user)
        .execute(conn)?;

    Ok(())
}

pub(crate) fn get_user_detail_by_name(
    conn: &MysqlConnection,
    name: &str,
) -> Result<DatabaseUser, diesel::result::Error> {
    use crate::database::schema::Users::dsl::*;
    let user = Users
        .filter(username.eq(name))
        .first::<DatabaseUser>(conn)
        .optional()?;

    match user {
        None => Err(diesel::NotFound),
        Some(u) => Ok(u),
    }
}

pub(crate) fn get_local_users(
    conn: &MysqlConnection,
) -> Result<Vec<(DatabaseUser, DatabaseLocalUser)>, diesel::result::Error> {
    use crate::database::schema::LocalUsers::dsl::*;
    use crate::database::schema::Users::dsl::*;

    Users
        .inner_join(LocalUsers)
        .select((Users::all_columns(), LocalUsers::all_columns()))
        .load::<(_, _)>(conn)
}
