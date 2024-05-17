use std::collections::HashMap;
use crate::metamodel::TypeModel;

type AppName = String;
type TypeName = String;

pub struct TypeModelProvider {
    app_models: HashMap<AppName, HashMap<TypeName, TypeModel>>,
}

impl TypeModelProvider {
    pub fn new(app_models: HashMap<String, HashMap<String, TypeModel>>) -> TypeModelProvider {
        TypeModelProvider { app_models }
    }

    pub fn get_type_model(&self, app_name: &str, entity_name: &str) -> Option<&TypeModel> {
        let app_map = self.app_models.get(app_name)?;
        let entity_model = app_map.get(entity_name)?;
        Some(entity_model)
    }
}
