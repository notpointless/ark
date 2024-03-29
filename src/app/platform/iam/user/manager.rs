use core::panic;
use std::any::TypeId;

use crate::app::{
    platform::iam::user::task::{UserUpdateAsBooleanTask, UserUpdateAsIntegerTask},
    service::{
        cache::{
            error::CacheResult,
            manager::CacheManager,
            message::{CacheLocation, CacheRequest, CacheStatus},
        },
        task::{
            error::TaskResult,
            manager::TaskManager,
            message::{TaskRequest, TaskStatus, TaskType},
        },
    },
};

use super::{
    cache::{UserAddToCache, UserReadFromCache},
    model::{User, UserSecurity},
    task::{
        UserAddPermission, UserAddRole, UserCreateSecurityToken, UserCreateTask, UserDeletePermission, UserDeleteRole, UserExchangeOAuthIdForId, UserPreloadCache, UserReadTask, UserUpdateTask
    },
};

pub struct UserManager;

impl UserManager {
    /// Create a user
    ///
    /// # Arguments
    /// - `user`: information about the user to create.
    ///
    /// # Examples
    /// ```
    /// create_user(user);
    /// ```
    pub fn create_user(user: User) -> TaskResult<TaskStatus> {
        let task_request = Self::create_user_request(user.clone());
        TaskManager::process_task(task_request)
        /*
        match TaskManager::process_task(task_request) {
            Ok(v) => {
                UserCacheManager::create_user_cache(user).unwrap();
                return Ok(v)
            },
            Err(er) => return Err(er),
        }
        */
    }

    /// Create a user request
    ///
    /// # Arguments
    /// - `user`: request for a user to be created .
    ///
    /// # Examples
    /// ```
    /// create_user_request(user);
    /// ```
    fn create_user_request(user: User) -> TaskRequest {
        TaskRequest::compose_request(UserCreateTask { user }, TaskType::User, "user_create")
    }

    /// Retrieve information about a specific user by their uuid
    ///
    /// # Arguments
    /// - `identifier`: the uuid of the user.
    ///
    /// # Examples
    /// ```
    /// get_user("2f4afce2-ec56-429a-96b1-480c0b20943a");
    /// ```
    pub fn get_user(identifier: &str) -> TaskResult<User> {
        let task_request = Self::get_user_request(&String::from(identifier));
        TaskManager::process_task_with_result::<User>(task_request)
    }

    /// get user request
    ///
    /// # Arguments
    /// - `identifier`: create a request for a user to be retrieved.
    ///
    /// # Examples
    /// ```
    /// get_user_request("2f4afce2-ec56-429a-96b1-480c0b20943a");
    /// ```
    fn get_user_request(identifier: &str) -> TaskRequest {
        TaskRequest::compose_request(
            UserReadTask {
                identifier: String::from(identifier),
            },
            TaskType::User,
            "user_read",
        )
    }

    /// Update a specific field for the specified user.
    ///
    /// # Arguments
    /// - `search_by`: the user identifier, id, username or email.
    /// - `update_for`: the field to update.
    /// - `value`: the desired value for the field.
    ///
    /// # Examples
    /// ```
    /// update_user("chomnr", "email", "newchomnr@gmail.com");
    /// update_user("2f4afce2-ec56-429a-96b1-480c0b20943a", "email", "newchomnr@gmail.com");
    /// update_user("chomnr@gmail.com", "email", "newchomnr@gmail.com");
    /// ``
    pub fn update_user(search_by: &str, update_for: &str, value: &str) -> TaskResult<TaskStatus> {
        let mut cache_request =
            Self::update_user_task_request::<String>(search_by, update_for, value);
        if update_for.eq_ignore_ascii_case("verified") {
            cache_request = Self::update_user_task_request::<bool>(search_by, update_for, value);
        }

        if update_for.eq_ignore_ascii_case("created_at")
            || update_for.eq_ignore_ascii_case("updated_at")
        {
            cache_request = Self::update_user_task_request::<i64>(search_by, update_for, value);
        }
        TaskManager::process_task(cache_request)
    }

    /// Read user from cache request.
    ///
    /// # Arguments
    /// - `search_by`: the user identifier, id, username or email.
    /// - `update_for`: the field to update.
    /// - `value`: the desired value for the field.
    ///
    /// # Examples
    /// ```
    /// update_user_task_request("chomnr", "email", "newchomnr@gmail.com");
    /// update_user_task_request("2f4afce2-ec56-429a-96b1-480c0b20943a", "email", "newchomnr@gmail.com");
    /// update_user_task_request("chomnr@gmail.com", "email", "newchomnr@gmail.com");
    /// ``
    fn update_user_task_request<T: 'static>(
        search_by: &str,
        update_for: &str,
        value: &str,
    ) -> TaskRequest {
        if TypeId::of::<T>() == TypeId::of::<str>() || TypeId::of::<T>() == TypeId::of::<String>() {
            return TaskRequest::compose_request(
                UserUpdateTask {
                    search_by: String::from(search_by),
                    update_for: String::from(update_for),
                    value: String::from(value),
                },
                TaskType::User,
                "user_update",
            );
        }

        if TypeId::of::<T>() == TypeId::of::<bool>() {
            return TaskRequest::compose_request(
                UserUpdateAsBooleanTask {
                    search_by: String::from(search_by),
                    update_for: String::from(update_for),
                    value: value
                        .parse::<bool>()
                        .expect("[ARC] update_user_task_request value is not of a bool type."),
                },
                TaskType::User,
                "user_update_as_boolean",
            );
        }

        if TypeId::of::<T>() == TypeId::of::<i64>() {
            return TaskRequest::compose_request(
                UserUpdateAsIntegerTask {
                    search_by: String::from(search_by),
                    update_for: String::from(update_for),
                    value: value
                        .parse::<i64>()
                        .expect("[ARC] update_user_task_request value is not of a integer type."),
                },
                TaskType::User,
                "user_update_as_integer",
            );
        }
        panic!("[ARC] update_user_task_request unsupported conversion type")
    }

    /// Creates a security token for user.
    ///
    /// # Note
    /// Security stamp automatically gets updated
    /// 
    /// # Arguments
    /// - `search_for`: the user identifier, id, username or email.
    /// - `action`: the action the token will be used for ex: email_reset, password_reset etc;.
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// preload_user_cache_request();
    /// ```
    pub fn create_security_token(search_by: &str, action: &str) -> TaskResult<UserSecurity> {
        let task_request = Self::create_security_token_request(search_by, action);
        TaskManager::process_task_with_result::<UserSecurity>(task_request)
    }

    /// Composes a security token request.
    ///
    /// # Examples
    /// ```
    /// let task_response = preload_permission_request();
    /// ```
    fn create_security_token_request(search_by: &str, action: &str) -> TaskRequest {
        TaskRequest::compose_request(
            UserCreateSecurityToken {
                search_by: String::from(search_by),
                action: String::from(action),
            },
            TaskType::User,
            "user_create_security_token",
        )
    }

    /// Exchange an oauth_id for a user_id.
    /// 
    /// # Arguments
    /// - `oauth_id`: the oauth id of the user.
    /// - `oauth_provider`: the provider.
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// exchange_oauth_for_id("oauth_id_here", "discord");
    /// ```
    pub fn exchange_oauth_for_id(oauth_id: &str, oauth_provider: &str) -> TaskResult<String> {
        let task_request = Self::exchange_oauth_for_id_request(oauth_id, oauth_provider);
        TaskManager::process_task_with_result::<String>(task_request)
    }

    /// Composes a exchange oauth for id request.
    ///
    /// # Arguments
    /// - `oauth_id`: the oauth id of the user.
    /// - `oauth_provider`: the provider.
    /// 
    /// # Examples
    /// ```
    /// let task_response = preload_permission_request();
    /// ```
    fn exchange_oauth_for_id_request(oauth_id: &str, oauth_provider: &str) -> TaskRequest {
        TaskRequest::compose_request(
            UserExchangeOAuthIdForId {
                oauth_id: String::from(oauth_id),
                provider: String::from(oauth_provider),
            },
            TaskType::User,
            "user_exchange_oauthid_for_id",
        )
    }

    /// Add permission to user.
    /// 
    /// # Arguments
    /// - `user_identifier`: the oauth id of the user.
    /// - `permission_identifier`: the permission identifier.
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// add_permission_to_user("oauth_id_here", "discord");
    /// ```
    pub fn add_permission_to_user(user_identifier: &str, permission_identifier: &str) -> TaskResult<TaskStatus> {
        let task_request = Self::add_permission_to_user_request(user_identifier, permission_identifier);
        TaskManager::process_task(task_request)
    }
    
    /// Composes a add user permission to a user request.
    ///
    /// # Arguments
    /// - `oauth_id`: The id of the user.
    /// - `oauth_provider`: the provider.
    /// 
    /// # Examples
    /// ```
    /// add_permission_to_user_request("user_identifier", "permission_identifier");
    /// ```
    fn add_permission_to_user_request(user_identifier: &str, permission_identifier: &str) -> TaskRequest {
        TaskRequest::compose_request(
            UserAddPermission {
                target_user_id: String::from(user_identifier),
                permission_identifier: String::from(permission_identifier),
            },
            TaskType::User,
            "user_add_permission",
        )
    }

    /// Add permission to user.
    /// 
    /// # Arguments
    /// - `user_identifier`: the oauth id of the user.
    /// - `permission_identifier`: the permission identifier.
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// delete_permission_from_user("oauth_id_here", "discord");
    /// ```
    pub fn delete_permission_from_user(user_identifier: &str, permission_identifier: &str) -> TaskResult<TaskStatus> {
        let task_request = Self::delete_permission_from_user_request(user_identifier, permission_identifier);
        TaskManager::process_task(task_request)
    }
    
    /// Composes a delete permission from user request.
    ///
    /// # Arguments
    /// - `user_identifier`: the oauth id of the user.
    /// - `permission_identifier`: the permission identifier.
    /// 
    /// # Examples
    /// ```
    /// delete_permission_from_user_request("user_identifier", "permission_identifier");
    /// ```
    fn delete_permission_from_user_request(user_identifier: &str, permission_identifier: &str) -> TaskRequest {
        TaskRequest::compose_request(
            UserDeletePermission {
                target_user_id: String::from(user_identifier),
                permission_identifier: String::from(permission_identifier),
            },
            TaskType::User,
            "user_delete_permission",
        )
    }

    /// Add role to user.
    /// 
    /// # Arguments
    /// - `user_identifier`: the oauth id of the user.
    /// - `role_identifier`: the permission identifier.
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// add_role_to_user("user_uuid", "role_identiifer");
    /// ```
    pub fn add_role_to_user(user_identifier: &str, role_identifier: &str) -> TaskResult<TaskStatus> {
        let task_request = Self::add_role_from_user_request(user_identifier, role_identifier);
        TaskManager::process_task(task_request)
    }
    
    /// Composes a add role from user request.
    ///
    /// # Arguments
    /// - `user_identifier`: the oauth id of the user.
    /// - `role_identifier`: the permission identifier.
    /// 
    /// # Examples
    /// ```
    /// add_role_from_user_request("user_identifier", "permission_identifier");
    /// ```
    fn add_role_from_user_request(user_identifier: &str, role_identifier: &str) -> TaskRequest {
        TaskRequest::compose_request(
            UserAddRole {
                target_user_id: String::from(user_identifier),
                role_identifier: String::from(role_identifier),
            },
            TaskType::User,
            "user_add_role",
        )
    }

    /// Delete role from user.
    /// 
    /// # Arguments
    /// - `user_identifier`: the oauth id of the user.
    /// - `role_identifier`: the permission identifier.
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// delete_role_from_user("user_uuid", "role_identiifer");
    /// ```
    pub fn delete_role_from_user(user_identifier: &str, role_identifier: &str) -> TaskResult<TaskStatus> {
        let task_request = Self::delete_role_from_user_request(user_identifier, role_identifier);
        TaskManager::process_task(task_request)
    }
    
    /// Composes a delete role from user request.
    ///
    /// # Arguments
    /// - `user_identifier`: the oauth id of the user.
    /// - `role_identifier`: the permission identifier.
    /// 
    /// # Examples
    /// ```
    /// delete_permission_from_user_request("user_identifier", "permission_identifier");
    /// ```
    fn delete_role_from_user_request(user_identifier: &str, role_identifier: &str) -> TaskRequest {
        TaskRequest::compose_request(
            UserDeleteRole {
                target_user_id: String::from(user_identifier),
                role_identifier: String::from(role_identifier),
            },
            TaskType::User,
            "user_delete_role",
        )
    }

    /// Preload user cache.
    ///
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// preload_user_cache_request();
    /// ```
    pub fn preload_user_cache() -> TaskResult<TaskStatus> {
        let task_request = Self::preload_user_cache_request();
        TaskManager::process_task(task_request)
    }

    /// Composes a user preload cache request.
    ///
    /// # Examples
    /// ```
    /// let task_response = preload_permission_request();
    /// ```
    fn preload_user_cache_request() -> TaskRequest {
        TaskRequest::compose_request(UserPreloadCache {}, TaskType::User, "user_preload_cache")
    }
}

pub(super) struct UserCacheManager;

impl UserCacheManager {
    /// Add user to cache.
    ///
    /// # Arguments
    /// - `user`: Add the specified user to a cache..
    ///
    /// # Examples
    /// ```
    /// add_user_to_cache(user);
    /// ```
    pub fn add_user_to_cache(user: User) -> CacheResult<CacheStatus> {
        let cache_request = Self::create_user_cache_request(user);
        CacheManager::process_cache(cache_request)
    }

    /// Add user to cache request.
    ///
    /// # Arguments
    /// - `user`: Add the specified user to a cache.
    ///
    /// # Examples
    /// ```
    /// create_user_cache_request(user);
    /// ```
    fn create_user_cache_request(user: User) -> CacheRequest {
        CacheRequest::compose_request(
            UserAddToCache { user },
            CacheLocation::User,
            "user_add_to_cache",
        )
    }

    /// Read user from cache.
    ///
    /// # Arguments
    /// - `identifier`: Reads the specified user from the cache via uuid.
    ///
    /// # Examples
    /// ```
    /// read_user_from_cache("2f4afce2-ec56-429a-96b1-480c0b20943a");
    /// ``
    pub fn read_user_from_cache(identifier: &str) -> CacheResult<User> {
        let cache_request = Self::read_user_cache_request(identifier);
        CacheManager::process_cache_with_result::<User>(cache_request)
    }

    /// Read user from cache request.
    ///
    /// # Arguments
    /// - `identifier`: Reads the specified user from the cache via uuid.
    ///
    /// # Examples
    /// ```
    /// read_user_cache_request("2f4afce2-ec56-429a-96b1-480c0b20943a");
    /// ``
    fn read_user_cache_request(identifier: &str) -> CacheRequest {
        CacheRequest::compose_request(
            UserReadFromCache {
                identifier: String::from(identifier),
            },
            CacheLocation::User,
            "user_read_from_cache",
        )
    }
}
