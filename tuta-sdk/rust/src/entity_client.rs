use std::sync::Arc;

use crate::{ApiCallError, AuthHeadersProvider, IdTuple, RestClient, TypeRef};
use crate::element_value::{ElementValue, ParsedEntity};
use crate::instance_mapper::InstanceMapper;
use crate::json_element::RawEntity;
use crate::rest_client::{HttpMethod, RestClientOptions};

pub struct EntityClient {
    rest_client: Arc<dyn RestClient>,
    base_url: String,
    auth_headers_provider: Arc<dyn AuthHeadersProvider + Send + Sync>,
    instance_mapper: Arc<InstanceMapper>
}

impl EntityClient {
    pub(crate) fn new(
        rest_client: Arc<dyn RestClient>,
        instance_mapper: Arc<InstanceMapper>,
        base_url: &str,
        auth_headers_provider: Arc<dyn AuthHeadersProvider + Send + Sync>,
    ) -> Self {
        EntityClient {
            rest_client,
            instance_mapper,
            base_url: base_url.to_owned(),
            auth_headers_provider,
        }
    }
}

impl EntityClient {
    pub async fn load_list_element(
        &self,
        type_ref: &TypeRef,
        id: &IdTuple,
    ) -> Result<ParsedEntity, ApiCallError> {
        let options = RestClientOptions {
            body: None,
            headers: self.auth_headers_provider.auth_headers(),
        };
        let url = format!(
            "{}/rest/{}/{}/{}/{}",
            self.base_url, type_ref.app, type_ref.type_, id.list_id, id.element_id
        );
        let response = self
            .rest_client
            .request_binary(url, HttpMethod::GET, options)
            .await?;
        match response.status {
            200..=299 => {
                // Ok
            }
            _ => return Err(ApiCallError::ServerResponseError { status: response.status })
        }
        let response_bytes = response.body.expect("no body");
        let response_entity = serde_json::from_slice::<RawEntity>(response_bytes.as_slice()).unwrap();
        let parsed_entity = self.instance_mapper.parse(type_ref, response_entity)?;
        Ok(parsed_entity)
    }

    pub async fn load_element(
        &self,
        type_ref: &TypeRef,
        id: &str,
    ) -> Result<ParsedEntity, ApiCallError> {
        let options = RestClientOptions {
            body: None,
            headers: self.auth_headers_provider.auth_headers(),
        };
        let url = format!(
            "{}/rest/{}/{}/{}",
            self.base_url, type_ref.app, type_ref.type_, id
        );
        let response = self
            .rest_client
            .request_binary(url, HttpMethod::GET, options)
            .await?;
        match response.status {
            200..=299 => {
                // Ok
            }
            _ => return Err(ApiCallError::ServerResponseError { status: response.status })
        }
        let response_bytes = response.body.expect("no body");
        let response_entity = serde_json::from_slice(response_bytes.as_slice()).unwrap();
        let parsed_entity = self.instance_mapper.parse(type_ref, response_entity)?;
        Ok(parsed_entity)
    }
    //
    // pub async fn load_all(
    //     &self,
    //     type_ref: &TypeRef,
    //     list_id: String,
    //     start: Option<String>,
    // ) -> Vec<RawEntity> {
    //     unimplemented!()
    // }
    //
    // pub async fn load_range(
    //     &self,
    //     type_ref: &TypeRef,
    //     list_id: &str,
    //     start_id: &str,
    //     count: &str,
    //     list_load_direction: ListLoadDirection,
    // ) -> Vec<RawEntity> {
    //     unimplemented!()
    // }
    //
    // pub async fn setup_element(&self, type_ref: &TypeRef, entity: RawEntity) -> Vec<String> {
    //     unimplemented!()
    // }
    //
    // pub async fn setup_list_element(
    //     &self,
    //     type_ref: &TypeRef,
    //     list_id: &str,
    //     entity: RawEntity,
    // ) -> Vec<String> {
    //     unimplemented!()
    // }
    //
    pub async fn update(&self, type_ref: &TypeRef, entity: ParsedEntity) -> Result<(), ApiCallError> {
        let id = match &entity.get("_id").unwrap() {
            ElementValue::IdTupleId(ref id_tuple) => {
                format!("{}/{}", &id_tuple.list_id, &id_tuple.element_id)
            }
            _ => panic!("id is not string or array"),
        };
        let raw_entity = self.instance_mapper.serialize(type_ref, entity)?;
        let body = serde_json::to_vec(&raw_entity).unwrap();
        let options = RestClientOptions {
            body: Some(body),
            headers: self.auth_headers_provider.auth_headers(),
        };
        // FIXME we should look at type model whether it is ET or LET
        let url = format!(
            "{}/rest/{}/{}/{}",
            self.base_url, type_ref.app, type_ref.type_, id
        );
        self.rest_client
            .request_binary(url, HttpMethod::PUT, options)
            .await?;
        Ok(())
    }
    //
    // pub async fn erase_element(&self, type_ref: &TypeRef, id: &str) {
    //     unimplemented!()
    // }
    //
    // pub async fn erase_list_element(&self, type_ref: &TypeRef, id: IdTuple) {}
}
