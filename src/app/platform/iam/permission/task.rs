use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::app::{
    database::postgres::PostgresDatabase,
    platform::iam::permission::model::Permission,
    service::task::{
        error::TaskError,
        message::{TaskRequest, TaskResponse, TaskStatus},
        Task, TaskHandler,
    },
};

#[derive(Serialize, Deserialize)]
pub struct PermissionTask<T: Serialize> {
    pub param: T,
}

pub struct PermissionTaskHandler;

#[async_trait]
impl TaskHandler for PermissionTaskHandler {
    async fn handle(pg: &PostgresDatabase, task_request: TaskRequest) -> TaskResponse {
        if task_request.task_action.eq("permission_create") {
            let payload = match TaskRequest::intepret_request_payload::<PermissionTask<Permission>>(
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
            return PermissionCreateTask::run(pg, task_request, payload.param).await;
        }

        if task_request.task_action.eq("permission_delete") {
            let payload = match TaskRequest::intepret_request_payload::<PermissionTask<String>>(
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
            return PermissionDeleteTask::run(pg, task_request, payload.param).await;
        }

        return TaskResponse::throw_failed_response(
            task_request,
            vec![TaskError::FailedToFindAction.to_string()],
        );
    }
}

/// Represents a task for creating a permission.
///
/// This struct does not hold any data itself and serves as a marker for implementing the `Task` trait,
/// specifically for creating a permission in a PostgreSQL database. The task takes a `Permission` object
/// as a parameter and returns a `TaskResult<bool>` indicating the success or failure of the operation.
///
/// # Examples
///
/// ```
/// #[async_trait]
/// impl Task<Permission, PostgresDatabase, bool> for CreatePermissionTask {
///     async fn run(pg: PostgresDatabase, param: Permission) -> TaskResult<bool> {
///         // Implementation goes here
///     }
/// }
/// ```
///
/// In this implementation, `run` is an asynchronous function that should contain the logic for creating
/// a new permission in the database. The result of this operation is encapsulated in `TaskResult<bool>`.
struct PermissionCreateTask;
#[async_trait]
impl Task<PostgresDatabase, TaskRequest, Permission> for PermissionCreateTask {
    async fn run(db: &PostgresDatabase, request: TaskRequest, param: Permission) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        let stmt = pool
            .prepare(
                "INSERT INTO iam_permissions (id, permission_name, permission_key) VALUES ($1, $2, $3)",
            )
            .await
            .unwrap();
        match pool
            .execute(
                &stmt,
                &[
                    &param.permission_id,
                    &param.permission_name,
                    &param.permission_key,
                ],
            )
            .await
        {
            Ok(_) => {
                return TaskResponse::compose_response(
                    request,
                    TaskStatus::Completed,
                    param,
                    Vec::default(),
                )
            }
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::PermissionDuplication.to_string()],
                )
            }
        }
    }
}

/// Represents a task for deleting a permission.
///
/// This struct does not hold any data itself and serves as a marker for implementing the `Task` trait,
/// specifically for deleting a permission in a PostgreSQL database. The task typically takes a `Permission` object
/// identifier as a parameter and returns a `TaskResult<bool>` indicating the success or failure of the deletion operation.
///
/// # Examples
///
/// ```
/// #[async_trait]
/// impl Task<PermissionIdentifier, PostgresDatabase, bool> for PermissionDeleteTask {
///     async fn run(pg: PostgresDatabase, param: PermissionIdentifier) -> TaskResult<bool> {
///         // Implementation goes here
///     }
/// }
/// ```
///
/// In this implementation, `run` is an asynchronous function that should contain the logic for deleting
/// an existing permission from the database. The result of this operation is encapsulated in `TaskResult<bool>`.
struct PermissionDeleteTask;
#[async_trait]
impl Task<PostgresDatabase, TaskRequest, String> for PermissionDeleteTask {
    async fn run(db: &PostgresDatabase, request: TaskRequest, param: String) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        let stmt = pool
            .prepare(
                "DELETE FROM iam_permissions
                WHERE id = $1
                   OR permission_name = $1
                   OR permission_key = $1",
            )
            .await
            .unwrap();
        match pool.execute(&stmt, &[&param]).await {
            Ok(v) => {
                if v != 0 {
                    return TaskResponse::compose_response(
                        request,
                        TaskStatus::Completed,
                        param,
                        Vec::default(),
                    );
                }
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::PermissionNotFound.to_string()],
                );
            }
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::TaskInternalError.to_string()],
                )
            }
        }
    }
}
