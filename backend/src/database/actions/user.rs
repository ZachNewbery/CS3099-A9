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

    let fed: Option<DatabaseFederatedUser> = get_federated_user(conn, user)?;

    match (local, fed) {
        (None, None) => Err(diesel::NotFound),
        (Some(l), _) => Ok(l.into()),
        (_, Some(f)) => Ok(f.into()),
    }
}

pub(crate) fn get_federated_user(
    conn: &MysqlConnection,
    user: &DatabaseUser,
) -> Result<Option<DatabaseFederatedUser>, diesel::result::Error> {
    DatabaseFederatedUser::belonging_to(user)
        .first::<DatabaseFederatedUser>(conn)
        .optional()
}

pub(crate) fn insert_new_federated_user(
    conn: &MysqlConnection,
    new_user: &User,
) -> Result<DatabaseUser, diesel::result::Error> {
    use crate::database::models::{DatabaseNewFederatedUser, DatabaseNewUser};
    use crate::database::schema::FederatedUsers::dsl::*;
    use crate::database::schema::Users::dsl::*;

    // Format the input
    let mut formatted_user = new_user.clone();
    formatted_user.host = formatted_user.host.replace("https://", "").replace("/", "");

    let db_new_user: DatabaseNewUser = formatted_user.clone().into();

    diesel::insert_into(Users)
        .values(db_new_user.clone())
        .execute(conn)?;

    let inserted_user: DatabaseUser = Users
        .filter(username.eq(&db_new_user.username))
        .first::<DatabaseUser>(conn)?;

    let db_new_fed_user: DatabaseNewFederatedUser = (inserted_user.clone(), formatted_user).into();

    diesel::insert_into(FederatedUsers)
        .values(db_new_fed_user)
        .execute(conn)?;

    Ok(inserted_user)
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

pub(crate) fn get_name_from_local_user(
    conn: &MysqlConnection,
    lu: DatabaseLocalUser,
) -> Result<DatabaseUser, diesel::result::Error> {
    use crate::database::schema::Users::dsl::*;

    Users.filter(id.eq(lu.user_id)).first::<DatabaseUser>(conn)
}
