use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use thiserror::Error;

use metamodel::TypeModel;
use rest_client::{RestClient, RestClientError};
use type_model_provider::TypeModelProvider;

use crate::entity_client::EntityClient;
use crate::instance_mapper::{InstanceMapper, InstanceMapperError};
use crate::mail_facade::MailFacade;

mod entity_client;
mod instance_mapper;
mod json_element;
mod rest_client;
mod element_value;
mod metamodel;
mod type_model_provider;
mod mail_facade;

uniffi::setup_scaffolding!();

#[derive(Debug, Clone)]
pub struct TypeRef {
    pub app: String,
    pub type_: String,
}

// Option 1:
// metamodel -> Rust struct -> Kotlin/Swift classes
// need to be able to covert from ParsedEntity -> Rust struct
// will generate a bit more code but we need to write the conversion only once
// might or might not work for WASM

// Option 2:
// metamodel -> Kotlin/Swift classes
// need to be able to covert from ParsedEntity -> Kotlin/Swift class
// will generate a bit less code but we need to write the conversion for every platform
// will work for WASM for sure

impl Display for TypeRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeRef({}, {})", self.app, self.type_)
    }
}

trait AuthHeadersProvider {
    fn auth_headers(&self) -> HashMap<String, String>;
}

enum LoginState {
    NotLoggedIn,
    LoggedIn { access_token: String },
}

struct SdkState {
    login_state: RwLock<LoginState>,
}

#[derive(uniffi::Object)]
pub struct Sdk {
    state: Arc<SdkState>,
    entity_client: Arc<EntityClient>,
}

fn init_type_model_provider() -> TypeModelProvider {
    let tutanota_type_model_str = include_str!("../test_data/tutanota_type_model.json");
    let tutanota_type_model =
        serde_json::from_str::<HashMap<String, TypeModel>>(&tutanota_type_model_str)
            .expect("Could not parse type model :(");
    let type_model_provider = TypeModelProvider::new(HashMap::from([(
        "tutanota".to_owned(),
        tutanota_type_model,
    )]));
    type_model_provider
}

#[uniffi::export]
impl Sdk {
    #[uniffi::constructor]
    pub fn new(base_url: String, rest_client: Arc<dyn RestClient>) -> Sdk {
        let type_model_provider = init_type_model_provider();
        // TODO validate parameters
        let instance_mapper = Arc::new(InstanceMapper::new(type_model_provider));
        let state = Arc::new(SdkState {
            login_state: RwLock::new(LoginState::NotLoggedIn),
        });
        Sdk {
            state: state.clone(),
            entity_client: Arc::new(EntityClient::new(
                rest_client,
                instance_mapper,
                &base_url,
                state,
            )),
        }
    }

    pub fn login(&self, access_token: &str) {
        let mut login_state = self.state.login_state.write().unwrap();
        if let LoginState::LoggedIn { .. } = *login_state {
            panic!("Already logged in!")
        }
        *login_state = LoginState::LoggedIn {
            access_token: access_token.to_owned(),
        }
    }


    pub fn mail_facade(&self) -> MailFacade {
        MailFacade::new(self.entity_client.clone())
    }
}

impl AuthHeadersProvider for SdkState {
    fn auth_headers(&self) -> HashMap<String, String> {
        let g = self.login_state.read().unwrap();
        match g.deref() {
            LoginState::NotLoggedIn => HashMap::new(),
            LoginState::LoggedIn { access_token } => {
                HashMap::from([("accessToken".to_owned(), access_token.as_str().to_owned())])
            }
        }
    }
}

#[derive(uniffi::Enum)]
pub enum ListLoadDirection {
    ASC,
    DESC,
}

#[derive(uniffi::Record, Debug, PartialEq, Clone)]
pub struct IdTuple {
    pub list_id: String,
    pub element_id: String,
}

impl IdTuple {
    pub fn new(list_id: String, element_id: String) -> Self {
        Self { list_id, element_id }
    }
}

#[derive(Error, Debug, uniffi::Error)]
pub enum ApiCallError {
    #[error("Rest client error, source: {source}")]
    RestClient {
        #[from]
        source: RestClientError,
    },
    #[error("ServerResponseError, status: {status}")]
    ServerResponseError { status: u32 },
    #[error("InternalSdkError: {error_message}")]
    InternalSdkError {
        error_message: String,
    },
}


impl From<InstanceMapperError> for ApiCallError {
    fn from(value: InstanceMapperError) -> Self {
        ApiCallError::InternalSdkError { error_message: value.to_string() }
    }
}