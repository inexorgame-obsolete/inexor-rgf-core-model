use std::convert::TryFrom;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::EntityInstance;
use crate::ReactiveFlowInstance;
use crate::RelationInstance;

#[derive(Debug)]
pub struct FlowInstanceCreationError;

/// A flow instance is a container for entity instances and relation instances.
///
/// A flow instance is strictly associated with a wrapper entity instance. The properties
/// of the wrapper entity instance are the properties of the flow.
///
/// Additionally, flows can be nested -  from the perspective of the outer flow
/// the inner flow acts like an entity instance. The wrapper entity instance of
/// the inner flow is the interface which can be accessed by the outer flow.
///
/// Entity instances and relation instances can be shared with multiple flows.
///
/// It's even possible to connect entity instances from different flows with relation
/// instances.
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlowInstance {
    /// The id of the flow corresponds to the id of the wrapper entity instance
    ///
    /// This means the vector of entity instances must contain an instance with
    /// the id of the flow.
    pub id: Uuid,

    /// The entity type of the flow instance.
    #[serde(alias = "type")]
    pub type_name: String,

    /// The name of the flow instance.
    #[serde(default = "String::new")]
    pub name: String,

    /// Textual description of the flow instance.
    #[serde(default = "String::new")]
    pub description: String,

    /// The entity instances which are contained in this flow instance.
    ///
    /// It can't have a default because the wrapper entity instance must be
    /// present in the list of entities.
    #[serde(alias = "entities")]
    pub entity_instances: Vec<EntityInstance>,

    /// The relation instances which are contained in this flow instance.
    ///
    /// By default, no relation instances are contained in this flow instance.
    #[serde(default = "Vec::new", alias = "relations")]
    pub relation_instances: Vec<RelationInstance>,
}

impl FlowInstance {
    /// Constructs a new flow instance from the wrapper entity instance.
    pub fn from_instance_with_name<S: Into<String>>(wrapper_entity_instance: EntityInstance, name: S) -> FlowInstance {
        FlowInstance {
            id: wrapper_entity_instance.id,
            type_name: wrapper_entity_instance.type_name.clone(),
            name: name.into(),
            description: String::new(),
            entity_instances: vec![wrapper_entity_instance],
            relation_instances: Vec::new(),
        }
    }
}

impl From<EntityInstance> for FlowInstance {
    fn from(wrapper_entity_instance: EntityInstance) -> FlowInstance {
        FlowInstance {
            id: wrapper_entity_instance.id,
            type_name: wrapper_entity_instance.type_name.clone(),
            name: String::new(),
            description: String::new(),
            entity_instances: vec![wrapper_entity_instance],
            relation_instances: Vec::new(),
        }
    }
}

impl TryFrom<ReactiveFlowInstance> for FlowInstance {
    type Error = FlowInstanceCreationError;

    fn try_from(reactive_flow: ReactiveFlowInstance) -> Result<Self, FlowInstanceCreationError> {
        let wrapper = reactive_flow.get_entity(reactive_flow.id);
        if wrapper.is_none() {
            return Err(FlowInstanceCreationError);
        }
        let wrapper = wrapper.unwrap();
        let entity_instance: EntityInstance = wrapper.clone().into();
        let mut flow_instance = FlowInstance::from(entity_instance);
        flow_instance.description = wrapper.description.clone();
        reactive_flow.entity_instances.read().unwrap().iter().for_each(|(_, entity)| {
            if entity.id != reactive_flow.id {
                flow_instance.entity_instances.push(entity.clone().into());
            }
        });
        reactive_flow.relation_instances.read().unwrap().iter().for_each(|(_, relation_instance)| {
            flow_instance.relation_instances.push(relation_instance.clone().into());
        });
        Ok(flow_instance)
    }
}

impl TryFrom<Arc<ReactiveFlowInstance>> for FlowInstance {
    type Error = FlowInstanceCreationError;

    fn try_from(reactive_flow: Arc<ReactiveFlowInstance>) -> Result<Self, FlowInstanceCreationError> {
        let wrapper = reactive_flow.get_entity(reactive_flow.id);
        if wrapper.is_none() {
            return Err(FlowInstanceCreationError);
        }
        let wrapper = wrapper.unwrap();
        let entity_instance: EntityInstance = wrapper.clone().into();
        let mut flow_instance = FlowInstance::from(entity_instance);
        flow_instance.description = wrapper.description.clone();
        reactive_flow.entity_instances.read().unwrap().iter().for_each(|(_, entity)| {
            if entity.id != reactive_flow.id {
                flow_instance.entity_instances.push(entity.clone().into());
            }
        });
        reactive_flow.relation_instances.read().unwrap().iter().for_each(|(_, relation_instance)| {
            flow_instance.relation_instances.push(relation_instance.clone().into());
        });
        Ok(flow_instance)
    }
}
