use uuid::Uuid;

use crate::app::service::task::{
    error::TaskResult,
    manager::TaskManager,
    message::{TaskRequest, TaskType},
};

use super::{model::UserSession, task::{SessionCreateTask, SessionRevocationTask}};

pub struct SessionManager;

impl SessionManager {
    /// Create a user session.
    ///
    /// # Arguments
    /// - `user_id`: who to create the sessionf or.
    ///
    /// # Examples
    /// ```
    /// let role = PermissionBuilder::builder()
    ///     .role_name("Member")
    ///     .build();
    /// create_role(role);
    /// ```
    pub fn create_session(user_id: &str) -> TaskResult<UserSession> {
        let task_request = Self::create_session_request(UserSession {
            token: Uuid::new_v4().as_simple().to_string(),
            expires_in: 604800,
            user_id: user_id.to_string(),
        });
        TaskManager::process_task_with_result::<UserSession>(task_request)
    }

    /// Composes a user session create request.
    ///
    /// # Arguments
    /// - `session`: A reference to the `Session` to process.
    ///
    /// # Examples
    /// ```
    /// Self::create_role_request(role)
    /// ```
    fn create_session_request(session: UserSession) -> TaskRequest {
        TaskRequest::compose_request(
            SessionCreateTask {
                token: session.token,
                expires_in: session.expires_in,
                user_id: session.user_id,
            },
            TaskType::Session,
            "session_create",
        )
    }

    /// Revoke a user session.
    ///
    /// # Arguments
    /// - `user_id`: who to create the sessionf or.
    ///
    /// # Examples
    /// ```
    /// SessionManager::revoke_session("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX").unwrap();
    /// SessionManager::revoke_session("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX").unwrap();
    /// ```
    pub fn revoke_session(user_id: &str) -> TaskResult<UserSession> {
        let task_request = Self::revoke_session_request(user_id);
        TaskManager::process_task_with_result::<UserSession>(task_request)
    }

    /// Composes a user session revocation request.
    ///
    /// # Arguments
    /// - `id`: The user id.
    ///
    /// # Examples
    /// ```
    /// Self::revoke_session_request(user_id)
    /// ```
    fn revoke_session_request(id: &str) -> TaskRequest {
        TaskRequest::compose_request(
            SessionRevocationTask {
                user_id: String::from(id),
            },
            TaskType::Session,
            "session_revocation",
        )
    }
}
