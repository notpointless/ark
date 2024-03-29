// todo: make a user_update_cache so I don't have to retrieve all the results every time
// will reduce lines of code by at least 200

use std::time::{SystemTime, UNIX_EPOCH};

use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::app::{
    database::postgres::PostgresDatabase,
    platform::iam::{
        permission::{cache::PermissionCache, model::Permission},
        role::{cache::RoleCache, model::Role},
    },
    service::{
        cache::{error::CacheError, notify_cache_hit, notify_cache_miss, LocalizedCache},
        task::{
            error::TaskError,
            message::{TaskRequest, TaskResponse, TaskStatus},
            Task, TaskHandler,
        },
    },
};

use super::{
    manager::UserCacheManager,
    model::{SecurityToken, User, UserSecurity},
};

pub struct UserTaskHandler;

#[async_trait]
impl TaskHandler<PostgresDatabase> for UserTaskHandler {
    async fn handle(pg: &PostgresDatabase, task_request: TaskRequest) -> TaskResponse {
        if task_request.task_action.eq("user_create") {
            let payload =
                match TaskRequest::intepret_request_payload::<UserCreateTask>(&task_request) {
                    Ok(p) => p,
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            task_request,
                            vec![TaskError::FailedToInterpretPayload.to_string()],
                        )
                    }
                };
            return UserCreateTask::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("user_read") {
            let payload = match TaskRequest::intepret_request_payload::<UserReadTask>(&task_request)
            {
                Ok(p) => p,
                Err(_) => {
                    return TaskResponse::throw_failed_response(
                        task_request,
                        vec![TaskError::FailedToInterpretPayload.to_string()],
                    )
                }
            };
            return UserReadTask::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("user_update") {
            let payload =
                match TaskRequest::intepret_request_payload::<UserUpdateTask>(&task_request) {
                    Ok(p) => p,
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            task_request,
                            vec![TaskError::FailedToInterpretPayload.to_string()],
                        )
                    }
                };
            return UserUpdateTask::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("user_update_as_boolean") {
            let payload = match TaskRequest::intepret_request_payload::<UserUpdateAsBooleanTask>(
                &task_request,
            ) {
                Ok(p) => p,
                Err(_) => {
                    return TaskResponse::throw_failed_response(
                        task_request,
                        vec![TaskError::FailedToInterpretPayload.to_string()],
                    )
                }
            };
            return UserUpdateAsBooleanTask::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("user_update_as_integer") {
            let payload = match TaskRequest::intepret_request_payload::<UserUpdateAsIntegerTask>(
                &task_request,
            ) {
                Ok(p) => p,
                Err(_) => {
                    return TaskResponse::throw_failed_response(
                        task_request,
                        vec![TaskError::FailedToInterpretPayload.to_string()],
                    )
                }
            };
            return UserUpdateAsIntegerTask::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("user_create_security_token") {
            let payload = match TaskRequest::intepret_request_payload::<UserCreateSecurityToken>(
                &task_request,
            ) {
                Ok(p) => p,
                Err(_) => {
                    return TaskResponse::throw_failed_response(
                        task_request,
                        vec![TaskError::FailedToInterpretPayload.to_string()],
                    )
                }
            };
            return UserCreateSecurityToken::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("user_exchange_oauthid_for_id") {
            let payload = match TaskRequest::intepret_request_payload::<UserExchangeOAuthIdForId>(
                &task_request,
            ) {
                Ok(p) => p,
                Err(_) => {
                    return TaskResponse::throw_failed_response(
                        task_request,
                        vec![TaskError::FailedToInterpretPayload.to_string()],
                    )
                }
            };
            return UserExchangeOAuthIdForId::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("user_add_role") {
            let payload =
                match TaskRequest::intepret_request_payload::<UserAddRole>(&task_request) {
                    Ok(p) => p,
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            task_request,
                            vec![TaskError::FailedToInterpretPayload.to_string()],
                        )
                    }
                };
            return UserAddRole::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("user_delete_role") {
            let payload =
                match TaskRequest::intepret_request_payload::<UserDeleteRole>(&task_request) {
                    Ok(p) => p,
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            task_request,
                            vec![TaskError::FailedToInterpretPayload.to_string()],
                        )
                    }
                };
            return UserDeleteRole::run(pg, task_request, payload).await;
        }


        if task_request.task_action.eq("user_add_permission") {
            let payload =
                match TaskRequest::intepret_request_payload::<UserAddPermission>(&task_request) {
                    Ok(p) => p,
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            task_request,
                            vec![TaskError::FailedToInterpretPayload.to_string()],
                        )
                    }
                };
            return UserAddPermission::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("user_delete_permission") {
            let payload =
                match TaskRequest::intepret_request_payload::<UserDeletePermission>(&task_request) {
                    Ok(p) => p,
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            task_request,
                            vec![TaskError::FailedToInterpretPayload.to_string()],
                        )
                    }
                };
            return UserDeletePermission::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("user_preload_cache") {
            let payload =
                match TaskRequest::intepret_request_payload::<UserPreloadCache>(&task_request) {
                    Ok(p) => p,
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            task_request,
                            vec![TaskError::FailedToInterpretPayload.to_string()],
                        )
                    }
                };
            return UserPreloadCache::run(pg, task_request, payload).await;
        }

        return TaskResponse::throw_failed_response(
            task_request,
            vec![TaskError::FailedToFindAction.to_string()],
        );
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct UserCreateTask {
    pub user: User,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserCreateTask> for UserCreateTask {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: UserCreateTask,
    ) -> TaskResponse {
        // because of how the users create their account (through oauth)
        // this operation should never fail.
        let mut pool = db.pool.get().await.unwrap();
        // dont include this as part of the transaction because if it fails the transaction fails.

        let transaction = pool.transaction().await.unwrap();
        transaction.execute(
            "INSERT INTO iam_users (id, verified, created_at, updated_at) VALUES ($1, $2, $3, $4)",
            &[&param.user.info.user_id, &param.user.info.verified, &param.user.info.created_at, &param.user.info.updated_at]
        ).await.unwrap();

        match transaction.execute(
            "INSERT INTO iam_user_oauth (user_id, oauth_id, oauth_provider) VALUES ($1, $2, $3)",
            &[&param.user.info.user_id, &param.user.auth.oauth_id, &param.user.auth.oauth_provider]
        ).await {
            Ok(_) => {},
            Err(_) => return TaskResponse::throw_failed_response(
                request,
                vec![TaskError::UserAlreadyExists.to_string()],
            ),
        }
        if !param.user.access.role.is_empty() {
            for role_identifier in &param.user.access.role {
                let role: Option<Role> = match RoleCache::get(role_identifier) {
                    Ok(v) => Some(v),
                    Err(_) => None,
                };
                if role != None {
                    transaction
                        .execute(
                            "INSERT INTO iam_user_role (user_id, role_id) VALUES ($1, $2)",
                            &[&param.user.info.user_id, &role.unwrap().role_id],
                        )
                        .await
                        .unwrap();
                }
            }
        }
        if !param.user.access.permission.is_empty() {
            for permission_identifier in &param.user.access.permission {
                let permission: Option<Permission> =
                    match PermissionCache::get(permission_identifier) {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    };
                if permission != None {
                    transaction.execute(
                    "INSERT INTO iam_user_permission (user_id, permission_id) VALUES ($1, $2)",
                    &[&param.user.info.user_id, &permission.unwrap().permission_id],
                ).await.unwrap();
                }
            }
        }
        match transaction.commit().await {
            Ok(_) => {
                UserCacheManager::add_user_to_cache(param.user.clone()).unwrap();
                return TaskResponse::compose_response(
                    request,
                    TaskStatus::Completed,
                    param,
                    Vec::default(),
                );
            }
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserAlreadyExists.to_string()],
                )
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct UserReadTask {
    pub identifier: String,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserReadTask> for UserReadTask {
    async fn run(db: &PostgresDatabase, request: TaskRequest, param: UserReadTask) -> TaskResponse {
        // very messy what we should do propogate/push onto call back the error but the way i built the system... yeah that won't work.

        let pool = db.pool.get().await.unwrap();

        match UserCacheManager::read_user_from_cache(&param.identifier) {
            Ok(user) => {
                notify_cache_hit("UserRead", "UserCache", &request.task_id);
                return TaskResponse::compose_response(
                    request,
                    TaskStatus::Completed,
                    user,
                    Vec::default(),
                );
            }
            Err(er) => {
                if er == CacheError::IdentifierMustBeAUuid {
                    return TaskResponse::throw_failed_response(
                        request,
                        vec![TaskError::FailedToCompleteTask.to_string()],
                    );
                }

                let fallback_stmt = pool
                    .prepare(
                        "SELECT 
                        u.id, 
                        u.username, 
                        u.email, 
                        u.verified, 
                        u.created_at, 
                        u.updated_at, 
                        array_agg(DISTINCT ur.role_id) FILTER (WHERE ur.role_id IS NOT NULL) AS roles, 
                        array_agg(DISTINCT up.permission_id) FILTER (WHERE up.permission_id IS NOT NULL) AS permissions,
                        o.oauth_id, 
                        o.oauth_provider,
                        u.security_token, 
                        u.security_stamp
                    FROM iam_users u
                    LEFT JOIN iam_user_role ur ON u.id = ur.user_id
                    LEFT JOIN iam_user_permission up ON u.id = up.user_id
                    LEFT JOIN iam_user_oauth o ON u.id = o.user_id OR o.oauth_id = $1
                    WHERE u.id = $1 OR o.oauth_id = $1
                    GROUP BY u.id, o.oauth_id, o.oauth_provider;",
                    )
                    .await
                    .unwrap();
                let fallback_query = pool.query_one(&fallback_stmt, &[&param.identifier]).await;
                match fallback_query {
                    Ok(row) => {
                        let user = User::new(
                            row.get(0),
                            row.get(1),
                            row.get(2),
                            row.get::<_, bool>(3),
                            row.get::<_, i64>(4),
                            row.get::<_, i64>(5),
                            row.get::<_, String>(8),
                            row.get::<_, String>(9),
                            row.get::<_, Option<Vec<String>>>(6).unwrap_or_default(),
                            row.get::<_, Option<Vec<String>>>(7).unwrap_or_default(),
                            UserSecurity::new(
                                SecurityToken::decode_then_deserialize(
                                    row.get::<_, Option<String>>(10),
                                ),
                                row.get(11),
                            ),
                        );
                        UserCacheManager::add_user_to_cache(user.clone()).unwrap();
                        notify_cache_miss("UserRead", "UserCache", &request.task_id);
                        return TaskResponse::compose_response(
                            request,
                            TaskStatus::Completed,
                            user.clone(),
                            Vec::default(),
                        );
                    }
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            request,
                            vec![TaskError::UserNotFound.to_string()],
                        );
                    }
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct UserUpdateTask {
    pub search_by: String,
    pub update_for: String,
    pub value: String,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserUpdateTask> for UserUpdateTask {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: UserUpdateTask,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        let stmt = match pool
            .prepare(
                format!(
                    "UPDATE iam_users
                SET {} = $1,
                updated_at = $2
                WHERE id = $3
                   OR username = $3
                   OR email = $3
                RETURNING *;",
                    param.update_for
                )
                .as_str(),
            )
            .await
        {
            Ok(v) => v,
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserFieldNotFound.to_string()],
                )
            }
        };
        match pool
            .query_one(
                &stmt,
                &[
                    &param.value,
                    &(SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as i64),
                    &param.search_by,
                ],
            )
            .await
        {
            Ok(row) => {
                if row.len() != 0 {
                    // test
                    match UserCacheManager::read_user_from_cache(&row.get::<_, String>(0)) {
                        Ok(mut user) => {
                            if param.update_for.eq_ignore_ascii_case("id") {
                                return TaskResponse::throw_failed_response(
                                    request,
                                    vec![TaskError::UserCannotUpdateId.to_string()],
                                );
                            }
                            if param.update_for.eq_ignore_ascii_case("username") {
                                user.info.username = Some(param.clone().value);
                            } else if param.update_for.eq_ignore_ascii_case("email") {
                                user.info.email = Some(param.clone().value);
                            }
                            UserCacheManager::add_user_to_cache(user).unwrap();
                        }
                        Err(_) => {
                            /* if not found in cache then it will just update the database. */
                        }
                    }
                    return TaskResponse::compose_response(
                        request,
                        TaskStatus::Completed,
                        param,
                        Vec::default(),
                    );
                }
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserNotFound.to_string()],
                );
            }
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserNotFound.to_string()],
                )
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct UserUpdateAsBooleanTask {
    pub search_by: String,
    pub update_for: String,
    pub value: bool,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserUpdateAsBooleanTask> for UserUpdateAsBooleanTask {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: UserUpdateAsBooleanTask,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        let stmt = match pool
            .prepare(
                format!(
                    "UPDATE iam_users
                SET {} = $1,
                updated_at = $2
                WHERE id = $3
                   OR username = $3
                   OR email = $3
                RETURNING *;",
                    param.update_for
                )
                .as_str(),
            )
            .await
        {
            Ok(v) => v,
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserFieldNotFound.to_string()],
                )
            }
        };
        match pool
            .query_one(
                &stmt,
                &[
                    &param.value,
                    &(SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as i64),
                    &param.search_by,
                ],
            )
            .await
        {
            Ok(row) => {
                if row.len() != 0 {
                    match UserCacheManager::read_user_from_cache(&row.get::<_, String>(0)) {
                        Ok(mut user) => {
                            if param.update_for.eq_ignore_ascii_case("verified") {
                                user.info.verified = param.clone().value;
                            }
                            UserCacheManager::add_user_to_cache(user).unwrap();
                        }
                        Err(_) => {
                            /* if not found in cache then it will just update the database. */
                        }
                    }
                    return TaskResponse::compose_response(
                        request,
                        TaskStatus::Completed,
                        param,
                        Vec::default(),
                    );
                }
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserNotFound.to_string()],
                );
            }
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserUniqueConstraint.to_string()],
                )
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct UserUpdateAsIntegerTask {
    pub search_by: String,
    pub update_for: String,
    pub value: i64,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserUpdateAsIntegerTask> for UserUpdateAsIntegerTask {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: UserUpdateAsIntegerTask,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        let stmt = match pool
            .prepare(
                format!(
                    "UPDATE iam_users
                SET {} = $1,
                updated_at = $2
                WHERE id = $3
                   OR username = $3
                   OR email = $3
                RETURNING *;",
                    param.update_for
                )
                .as_str(),
            )
            .await
        {
            Ok(v) => v,
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserFieldNotFound.to_string()],
                )
            }
        };
        match pool
            .query_one(
                &stmt,
                &[
                    &param.value,
                    &(SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as i64),
                    &param.search_by,
                ],
            )
            .await
        {
            Ok(row) => {
                if row.len() != 0 {
                    match UserCacheManager::read_user_from_cache(&row.get::<_, String>(0)) {
                        Ok(mut user) => {
                            if param.update_for.eq_ignore_ascii_case("created_at") {
                                user.info.created_at = param.clone().value;
                            }
                            if param.update_for.eq_ignore_ascii_case("updated_at") {
                                user.info.updated_at = param.clone().value;
                            }
                            UserCacheManager::add_user_to_cache(user).unwrap();
                        }
                        Err(_) => {
                            /* if not found in cache then it will just update the database. */
                        }
                    }
                    return TaskResponse::compose_response(
                        request,
                        TaskStatus::Completed,
                        param,
                        Vec::default(),
                    );
                }
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserNotFound.to_string()],
                );
            }
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserUniqueConstraint.to_string()],
                )
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct UserCreateSecurityToken {
    pub search_by: String,
    pub action: String,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserCreateSecurityToken> for UserCreateSecurityToken {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: UserCreateSecurityToken,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        let user_security = UserSecurity::create(&param.action);
        // updating security_stam
        let stmt_1 = pool
            .prepare(
                format!(
                    "UPDATE iam_users
                SET security_stamp = $1
                WHERE id = $2
                   OR username = $2
                   OR email = $2
                   RETURNING *;"
                )
                .as_str(),
            )
            .await
            .unwrap();
        pool.execute(
            &stmt_1,
            &[&user_security.clone().stamp.unwrap(), &param.search_by],
        )
        .await
        .unwrap();
        // updating security_token
        let stmt_2 = pool
            .prepare(
                format!(
                    "UPDATE iam_users
                SET security_token = $1
                WHERE id = $2
                   OR username = $2
                   OR email = $2
                   RETURNING *;"
                )
                .as_str(),
            )
            .await
            .unwrap();
        match pool
            .query_one(
                &stmt_2,
                &[
                    &user_security.clone().token.unwrap().serialize_then_hex(),
                    &param.search_by,
                ],
            )
            .await
        {
            Ok(row) => {
                if row.len() != 0 {
                    match UserCacheManager::read_user_from_cache(&row.get::<_, String>(0)) {
                        Ok(mut user) => {
                            user.security = user_security.clone();
                            UserCacheManager::add_user_to_cache(user).unwrap();
                        }
                        Err(_) => {
                            /* if not found in cache then it will just update the database. */
                        }
                    }
                    return TaskResponse::compose_response(
                        request,
                        TaskStatus::Completed,
                        user_security,
                        Vec::default(),
                    );
                }
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserNotFound.to_string()],
                );
            }
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserFailedToCreateSecurityToken.to_string()],
                )
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct UserExchangeOAuthIdForId {
    pub oauth_id: String,
    pub provider: String,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserExchangeOAuthIdForId> for UserExchangeOAuthIdForId {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: UserExchangeOAuthIdForId,
    ) -> TaskResponse {
        // retrieves directly from database.
        let pool = db.pool.get().await.unwrap();
        let stmt = pool
            .prepare(
                "SELECT user_id FROM iam_user_oauth WHERE oauth_id = $1 AND oauth_provider = $2",
            )
            .await
            .unwrap();
        match pool
            .query_one(&stmt, &[&param.oauth_id, &param.provider])
            .await
        {
            Ok(row) => {
                return TaskResponse::compose_response(
                    request,
                    TaskStatus::Completed,
                    row.get::<_, String>(0),
                    Vec::default(),
                );
            }
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserOAuthIdNotFound.to_string()],
                )
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct UserAddPermission {
    pub target_user_id: String,
    pub permission_identifier: String,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserAddPermission> for UserAddPermission {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: UserAddPermission,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        let stmt = pool
            .prepare("INSERT INTO iam_user_permission (user_id, permission_id) VALUES ($1, $2)")
            .await
            .unwrap();

        match PermissionCache::get(&param.permission_identifier) {
            Ok(permission) => match UserCacheManager::read_user_from_cache(&param.target_user_id) {
                Ok(mut cached_user) => match pool
                    .execute(
                        &stmt,
                        &[&param.target_user_id, &permission.permission_id],
                    )
                    .await
                {
                    Ok(_) => {
                        cached_user.access.permission.push(permission.permission_id);
                        UserCacheManager::add_user_to_cache(cached_user).unwrap();
                        // user exists in the cache so we need to update
                        return TaskResponse::compose_response(
                            request,
                            TaskStatus::Completed,
                            String::default(),
                            Vec::default(),
                        );
                    }
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            request,
                            vec![TaskError::UserPermissionAlreadyExists.to_string()],
                        );
                    }
                },
                Err(_) => {
                    // we don't need to directly check if a user exists here
                    // because of the foreign keys, we cannot insert an empty
                    // user because of that so if it throws out an error
                    // that means the user does not exist.
                    match pool
                        .execute(
                            &stmt,
                            &[&param.target_user_id, &param.permission_identifier],
                        )
                        .await
                    {
                        Ok(_) => {
                            return TaskResponse::compose_response(
                                request,
                                TaskStatus::Completed,
                                String::default(),
                                Vec::default(),
                            );
                        }
                        Err(_) => {
                            return TaskResponse::throw_failed_response(
                                request,
                                vec![TaskError::UserNotFound.to_string()],
                            );
                        }
                    }
                }
            },
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::PermissionNotFound.to_string()],
                )
            }
        }
    }
}


#[derive(Serialize, Deserialize)]
pub(super) struct UserDeletePermission {
    pub target_user_id: String,
    pub permission_identifier: String,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserDeletePermission> for UserDeletePermission {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: UserDeletePermission,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        let stmt = pool
            .prepare("DELETE FROM iam_user_permission WHERE user_id = $1 and permission_id = $2")
            .await
            .unwrap();
        match PermissionCache::get(&param.permission_identifier) {
            Ok(permission) => match UserCacheManager::read_user_from_cache(&param.target_user_id) {
                Ok(mut cached_user) => match pool
                    .execute(
                        &stmt,
                        &[&param.target_user_id, &permission.permission_id],
                    )
                    .await
                {
                    Ok(_) => {
                        cached_user.access.permission.retain(|perm| !perm.eq(&permission.permission_id) );
                        UserCacheManager::add_user_to_cache(cached_user).unwrap();
                        // user exists in the cache so we need to update
                        return TaskResponse::compose_response(
                            request,
                            TaskStatus::Completed,
                            String::default(),
                            Vec::default(),
                        );
                    }
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            request,
                            vec![TaskError::UserPermissionAlreadyExists.to_string()],
                        );
                    }
                },
                Err(_) => {
                    // we don't need to directly check if a user exists here
                    // because of the foreign keys, we cannot insert an empty
                    // user because of that so if it throws out an error
                    // that means the user does not exist.
                    match pool
                        .execute(
                            &stmt,
                            &[&param.target_user_id, &param.permission_identifier],
                        )
                        .await
                    {
                        Ok(_) => {
                            return TaskResponse::compose_response(
                                request,
                                TaskStatus::Completed,
                                String::default(),
                                Vec::default(),
                            );
                        }
                        Err(_) => {
                            return TaskResponse::throw_failed_response(
                                request,
                                vec![TaskError::UserNotFound.to_string()],
                            );
                        }
                    }
                }
            },
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::PermissionNotFound.to_string()],
                )
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct UserAddRole {
    pub target_user_id: String,
    pub role_identifier: String,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserAddRole> for UserAddRole {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: UserAddRole,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        let stmt = pool
            .prepare("INSERT INTO iam_user_role (user_id, role_id) VALUES ($1, $2)")
            .await
            .unwrap();

        match RoleCache::get(&param.role_identifier) {
            Ok(role) => match UserCacheManager::read_user_from_cache(&param.target_user_id) {
                Ok(mut cached_user) => match pool
                    .execute(
                        &stmt,
                        &[&param.target_user_id, &role.role_id],
                    )
                    .await
                {
                    Ok(_) => {
                        cached_user.access.role.push(role.role_id);
                        UserCacheManager::add_user_to_cache(cached_user).unwrap();
                        // user exists in the cache so we need to update
                        return TaskResponse::compose_response(
                            request,
                            TaskStatus::Completed,
                            String::default(),
                            Vec::default(),
                        );
                    }
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            request,
                            vec![TaskError::UserRoleAlreadyExists.to_string()],
                        );
                    }
                },
                Err(_) => {
                    match pool
                        .execute(
                            &stmt,
                            &[&param.target_user_id, &role.role_id],
                        )
                        .await
                    {
                        Ok(_) => {
                            return TaskResponse::compose_response(
                                request,
                                TaskStatus::Completed,
                                String::default(),
                                Vec::default(),
                            );
                        }
                        Err(_) => {
                            return TaskResponse::throw_failed_response(
                                request,
                                vec![TaskError::UserNotFound.to_string()],
                            );
                        }
                    }
                }
            },
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::RoleNotFound.to_string()],
                )
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct UserDeleteRole {
    pub target_user_id: String,
    pub role_identifier: String,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserDeleteRole> for UserDeleteRole {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: UserDeleteRole,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        let stmt = pool
            .prepare("DELETE FROM iam_user_role WHERE user_id = $1 and role_id = $2")
            .await
            .unwrap();

        match RoleCache::get(&param.role_identifier) {
            Ok(role) => match UserCacheManager::read_user_from_cache(&param.target_user_id) {
                Ok(mut cached_user) => match pool
                    .execute(
                        &stmt,
                        &[&param.target_user_id, &param.role_identifier],
                    )
                    .await
                {
                    Ok(_) => {
                        cached_user.access.role.retain(|perm| !perm.eq(&role.role_id) );
                        UserCacheManager::add_user_to_cache(cached_user).unwrap();
                        // user exists in the cache so we need to update
                        return TaskResponse::compose_response(
                            request,
                            TaskStatus::Completed,
                            String::default(),
                            Vec::default(),
                        );
                    }
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            request,
                            vec![TaskError::UserRoleAlreadyExists.to_string()],
                        );
                    }
                },
                Err(_) => {
                    match pool
                        .execute(
                            &stmt,
                            &[&param.target_user_id, &param.role_identifier],
                        )
                        .await
                    {
                        Ok(_) => {
                            return TaskResponse::compose_response(
                                request,
                                TaskStatus::Completed,
                                String::default(),
                                Vec::default(),
                            );
                        }
                        Err(_) => {
                            return TaskResponse::throw_failed_response(
                                request,
                                vec![TaskError::UserNotFound.to_string()],
                            );
                        }
                    }
                }
            },
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::RoleNotFound.to_string()],
                )
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct UserPreloadCache;

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserPreloadCache> for UserPreloadCache {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        _param: UserPreloadCache,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        let stmt = pool.prepare(
            "SELECT 
            u.id, 
            u.username, 
            u.email, 
            u.verified, 
            u.created_at, 
            u.updated_at, 
            array_agg(DISTINCT ur.role_id) FILTER (WHERE ur.role_id IS NOT NULL) AS roles, 
            array_agg(DISTINCT up.permission_id) FILTER (WHERE up.permission_id IS NOT NULL) AS permissions,
            o.oauth_id, 
            o.oauth_provider,
            u.security_token, 
            u.security_stamp
        FROM iam_users u
        LEFT JOIN iam_user_role ur ON u.id = ur.user_id
        LEFT JOIN iam_user_permission up ON u.id = up.user_id
        LEFT JOIN iam_user_oauth o ON u.id = o.user_id
        WHERE updated_at >= EXTRACT(EPOCH FROM NOW()) - 604800
        GROUP BY u.id, o.oauth_id, o.oauth_provider;",
        ).await.unwrap();

        match pool.query(&stmt, &[]).await {
            Ok(rows) => {
                let mut amt_items = 0;
                for row in rows {
                    amt_items += 1;
                    let user = User::new(
                        row.get(0),
                        row.get(1),
                        row.get(2),
                        row.get::<_, bool>(3),
                        row.get::<_, i64>(4),
                        row.get::<_, i64>(5),
                        row.get::<_, String>(8),
                        row.get::<_, String>(9),
                        row.get::<_, Option<Vec<String>>>(6).unwrap_or_default(),
                        row.get::<_, Option<Vec<String>>>(7).unwrap_or_default(),
                        UserSecurity::new(
                            SecurityToken::decode_then_deserialize(
                                row.get::<_, Option<String>>(10),
                            ),
                            row.get(11),
                        ),
                    );
                    UserCacheManager::add_user_to_cache(user.clone()).unwrap();
                }
                println!("[ARK] cached {} user(s) cache.", amt_items);
                return TaskResponse::compose_response(
                    request,
                    TaskStatus::Completed,
                    String::default(),
                    Vec::default(),
                );
            }
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserFailedToPreload.to_string()],
                )
            }
        }
    }
}
