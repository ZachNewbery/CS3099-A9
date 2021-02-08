use crate::database::models::{DatabaseFederatedUser, DatabaseLocalUser, DatabaseUser};
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
