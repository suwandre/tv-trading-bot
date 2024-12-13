use serde::Serialize;

/// `ApiResponse` is a generic struct that represents the response that the server sends back to the client.
#[derive(Serialize)]
pub struct ApiResponse<T> {
    /// includes both the status code and the status message.
    pub status: &'static str,
    /// includes the message that the server wants to send back to the client.
    pub message: String,
    /// includes any optional data that the server wants to send back to the client.
    pub data: Option<T>
}