use core::fmt;
use std::env;

use axum::{extract::FromRef, routing::get, Extension, Router};
use tokio::net::TcpListener;
use tower_cookies::{CookieManagerLayer, Key};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

static ADDRESS: &str = "0.0.0.0";
static PORT: usize = 3000;
static MODE: ServerMode = ServerMode::Development;

/// Represents a server configuration.
///
/// This struct holds the necessary configuration details for setting up and running a server.
/// It includes information about the server's address, operational mode, and tracing status.
///
/// # Fields
///
/// * `address` - The network address of the server, represented as a `String`.
/// * `mode` - The operational mode of the server, indicated by the `ServerMode` enum.
//  * `router` - The axum router.
///
/// # Example
///
/// ```
/// let server = ArcServer {
///     address: "127.0.0.1:8080".to_string(),
///     mode: ServerMode::Development,
///     tracing: true,
/// };
/// // The server is now configured to run on localhost port 8080 in development mode with tracing enabled.
/// ```
pub struct ArcServer {
    address: String,
    port: usize,
    mode: ServerMode,
    router: Router,
}

impl Default for ArcServer {
    fn default() -> Self {
        Self {
            address: ADDRESS.to_string(),
            port: PORT,
            mode: MODE,
            router: Router::new()
                .route("/", get(|| async { "Hello, World!" }))
                .layer(CookieManagerLayer::new())
                .layer(Extension(ArcState::default())),
        }
    }
}

impl ArcServer {
    /// Executes server operations based on the current server mode.
    ///
    /// This function checks the server mode (`self.mode`) and executes the corresponding
    /// server operation. There are three modes: Production, Development, and Maintenance.
    /// Each mode triggers a different behavior.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let arc = ArcServer::default();
    /// arc.run().await; // starts the server in self.mode mode
    /// ```
    pub async fn run(self) {
        let tcp = TcpListener::bind(&self.get_addr()).await.unwrap();
        println!("[ARC] mode: {}", self.mode.to_string());
        match self.mode {
            ServerMode::Production => {}
            ServerMode::Development => {
                Self::enable_tracing();
            }
            ServerMode::Maintenance => {
                Self::enable_tracing();
            }
        }
        println!(
            "[ARC] router initialized, now listening on port {}.",
            &self.port
        );
        axum::serve(tcp, self.router).await.unwrap();
    }

    /// Retrieves the full network address of the server.
    ///
    /// This function combines the server's address and port into a single `String`
    /// representation, formatted as "address:port". It's useful for quickly obtaining
    /// the complete address endpoint of the server.
    ///
    /// # Returns
    ///
    /// A `String` representing the server's full address.
    ///
    /// # Example
    ///
    /// ```
    /// let server = ArcServer { address: "127.0.0.1".to_string(), port: 8080, ... };
    /// let address = server.get_addr();
    /// assert_eq!(address, "127.0.0.1:8080");
    /// ```
    fn get_addr(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }

    /// Initializes tracing functionality for the application.
    ///
    /// This function sets up the tracing subscriber with environment-based filtering and
    /// a standard format layer. It's intended to be called during the server's startup
    /// phase to enable logging and diagnostic tracing.
    ///
    /// By default, it uses the environment's filter configuration or falls back to a
    /// debug level for the `with_axum_htmx_askama` crate.
    ///
    /// # Example
    ///
    /// ```
    /// ArcServer::enable_tracing();
    /// // Tracing is now enabled and configured.
    /// ```
    fn enable_tracing() {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
        println!("[ARC] tracer initialized.");
    }
}

/// Defines the operational modes for a server.
///
/// Variants:
/// - `Production`: Mode indicating the server is in a live, production environment (value 0).
/// - `Development`: Mode for development and testing purposes (value 1).
/// - `Maintenance`: Indicates the server is in maintenance mode, possibly for updates or repairs (value 2).
#[derive(Clone, Copy, PartialEq)]
enum ServerMode {
    Production,
    Development,
    Maintenance,
}

impl fmt::Display for ServerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerMode::Production => write!(f, "Production"),
            ServerMode::Development => write!(f, "Development"),
            ServerMode::Maintenance => write!(f, "Maintenance"),
        }
    }
}

#[derive(Clone)]
pub struct ArcState {
    key: Key,
}

impl FromRef<ArcState> for Key {
    fn from_ref(state: &ArcState) -> Self {
        state.key.clone()
    }
}

impl ArcState {
    fn default() -> Self {
        Self {
            key: ArcState::get_key(),
        }
    }

    pub fn get_key() -> Key {
        Key::from(
            env::var("COOKIE_ENCRYPTION_KEY")
                .expect("COOKIE_ENCRYPTION_KEY")
                .into_bytes()
                .as_slice(),
        )
    }
}