use crate::models::Request;

/// Represents a query parameter for the ID of an operation.
pub struct OperationId(pub String);

/// Represents the absence of a query parameter for the ID of an operation.
#[derive(Default, Clone)]
pub struct NoOperationId;

/// Represents a request to fetch details of a single operation from the Horizon API.
///
/// `SingleOperationRequest` is a struct tailored to querying details of a specific operation
/// on the Horizon API. This struct is designed to be used in conjunction with the
/// [`HorizonClient::get_single_operation`](crate::horizon_client::HorizonClient::get_single_operation) method.
///
/// # Fields
/// Required:
/// * `operation_id` - The ID of the operation.
///
/// ## Usage
/// Instances of `SingleOperationRequest` are created and configured using setter methods for each
/// parameter.
/// ```
/// # use stellar_rs::operations::prelude::SingleOperationRequest;
/// # use stellar_rs::models::Request;
/// let request = SingleOperationRequest::new()
///    .set_operation_id("123456");
/// // Use with HorizonClient::get_single_operation
/// ```
///
#[derive(Default, Clone)]
pub struct SingleOperationRequest<I> {
    /// The ID of the operation.
    operation_id: I,
}

impl SingleOperationRequest<NoOperationId> {
    /// Creates a new `SingleOperationRequest` with default parameters.
    pub fn new() -> Self {
        SingleOperationRequest::default()
    }

    /// Sets the operation ID for the request.
    ///
    /// # Arguments
    /// * `operation_id` - A `String` specifying the operation ID.
    ///
    pub fn set_operation_id(self, operation_id: impl Into<String>) -> SingleOperationRequest<OperationId> {
        SingleOperationRequest {
            operation_id: OperationId(operation_id.into()),
        }
    }
}

impl Request for SingleOperationRequest<OperationId> {
    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        query.push_str(&format!("{}", self.operation_id.0));

        query.trim_end_matches('&').to_string()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}/{}",
            base_url,
            super::OPERATIONS_PATH,
            self.get_query_parameters()
        )
    }
}
