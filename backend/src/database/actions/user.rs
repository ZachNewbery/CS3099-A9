use crate::database::models::{DatabaseFederatedUser, DatabaseLocalUser, DatabaseUser};
use chrono::Local;
use diesel::prelude::*;
use diesel::MysqlConnection;
use either::Either;
use either::Either::{Left, Right};

pub(crate) fn get_user_detail(
    conn: &MysqlConnection,
    user: &DatabaseUser,
) -> Result<Either<DatabaseLocalUser, DatabaseFederatedUser>, diesel::result::Error> {
    let local: Option<DatabaseLocalUser> = DatabaseLocalUser::belonging_to(user)
        .first::<DatabaseLocalUser>(conn)
        .optional()?;

    let fed: Option<DatabaseFederatedUser> = DatabaseFederatedUser::belonging_to(user)
        .first::<DatabaseFederatedUser>(conn)
        .optional()?;

    if local.is_none() && fed.is_none() {
        return Err(diesel::NotFound);
    }

    Ok(local.map_or_else(|| Right(fed.unwrap()), |l| Left(l)))
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

pub(crate) fn get_local_user(
    conn: &MysqlConnection,
    id_: &str,
) -> Result<Option<(DatabaseUser, DatabaseLocalUser)>, diesel::result::Error> {
    use crate::database::schema::LocalUsers::dsl::*;
    use crate::database::schema::Users::dsl::*;

    Users
        .inner_join(LocalUsers)
        .filter(username.eq(id_))
        .select((Users::all_columns(), LocalUsers::all_columns()))
        .first::<(_, _)>(conn)
        .optional()
}
