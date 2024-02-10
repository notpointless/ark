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

        return TaskResponse::throw_failed_response(
            task_request,
            vec![TaskError::FailedToFindAction.to_string()],
        );
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserCreateTask {
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

        let mut transaction = pool.transaction().await.unwrap();
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
pub struct UserReadTask {
    pub identifier: String,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserReadTask> for UserReadTask {
    async fn run(db: &PostgresDatabase, request: TaskRequest, param: UserReadTask) -> TaskResponse {
        // very messy what we should do propogate/push onto call back the error but the way i built the system... yeah that won't work.

        let mut pool = db.pool.get().await.unwrap();

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
                // retriever user from postgres database here..1
                // UserManager::get_id_from_oauth_id();

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
                    LEFT JOIN iam_user_oauth o ON u.id = o.user_id
                    WHERE u.id = $1
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
                                SecurityToken::deserialize_and_decode(row.get::<_, Option<&str>>(10)),
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