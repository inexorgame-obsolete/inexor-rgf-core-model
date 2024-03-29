use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::BehaviourTypeId;
use crate::NamespacedType;
use crate::RelationTypeId;

/// The behaviour of a relation type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RelationBehaviourTypeId {
    /// The relation type.
    pub relation_ty: RelationTypeId,

    /// The behaviour type.
    pub behaviour_ty: BehaviourTypeId,
}

impl RelationBehaviourTypeId {
    pub fn new(relation_ty: RelationTypeId, behaviour_ty: BehaviourTypeId) -> Self {
        RelationBehaviourTypeId { relation_ty, behaviour_ty }
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> Self {
        let namespaced_type = NamespacedType::new(namespace, type_name);
        RelationBehaviourTypeId::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}

impl From<NamespacedType> for RelationBehaviourTypeId {
    fn from(namespaced_type: NamespacedType) -> Self {
        RelationBehaviourTypeId::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}

impl From<&BehaviourTypeId> for RelationBehaviourTypeId {
    fn from(behaviour_ty: &BehaviourTypeId) -> Self {
        RelationBehaviourTypeId::new(NamespacedType::from(behaviour_ty).into(), behaviour_ty.clone())
    }
}

impl Display for RelationBehaviourTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", &self.relation_ty, &self.behaviour_ty)
    }
}

#[macro_export]
macro_rules! relation_behaviour_ty {
    (
        $relation_behaviour_type_id: ident,
        $relation_type_id: ident,
        $behaviour_type_id: ident
    ) => {
        lazy_static::lazy_static! {
            pub static ref $relation_behaviour_type_id: $crate::RelationBehaviourTypeId = $crate::RelationBehaviourTypeId::new($relation_type_id.clone(), $behaviour_type_id.clone());
        }
    };
}
