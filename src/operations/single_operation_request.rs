use crate::models::Request;

pub struct OperationId(pub String);

#[derive(Default, Clone)]
pub struct NoOperationId;

#[derive(Default, Clone)]
pub struct SingleOperationRequest<I> {
    operation_id: I,
}

impl SingleOperationRequest<NoOperationId> {
    pub fn new() -> Self {
        SingleOperationRequest::default()
    }

    pub fn set_operation_id(self, operation_id: String) -> SingleOperationRequest<OperationId> {
        SingleOperationRequest {
            operation_id: OperationId(operation_id),
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
