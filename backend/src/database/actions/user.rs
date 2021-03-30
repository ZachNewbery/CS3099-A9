use crate::database::models::{DatabaseFederatedUser, DatabaseLocalUser, DatabaseUser};

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

pub(crate) fn get_user(
    conn: &MysqlConnection,
    uid: &u64,
) -> Result<Option<DatabaseUser>, diesel::result::Error> {
    use crate::database::schema::Users::dsl::*;
    Users
        .filter(id.eq(uid))
        .first::<DatabaseUser>(conn)
        .optional()
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
