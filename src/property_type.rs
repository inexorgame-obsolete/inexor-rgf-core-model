use async_graphql::*;
use serde::{Deserialize, Serialize};

use crate::{DataType, SocketType};
use crate::extension::Extension;

/// Definition of a property. The definition contains
/// the name of the property, the data type and the socket
/// type.
#[derive(Serialize, Deserialize, Clone, Debug, SimpleObject)]
pub struct PropertyType {

    /// The name of the property
    pub name: String,

    /// The data type of the property
    pub data_type: DataType,

    /// Specifies which type of socket
    #[serde(default = "SocketType::none")]
    pub socket_type: SocketType,

    /// Property specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,
}

impl PropertyType {
    pub fn new<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            data_type,
            socket_type: SocketType::None,
            extensions: Vec::new(),
        }
    }

    pub fn new_with_socket<S: Into<String>>(name: S, data_type: DataType, socket_type: SocketType) -> PropertyType {
        PropertyType {
            name: name.into(),
            data_type,
            socket_type,
            extensions: Vec::new(),
        }
    }

    pub fn input<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            data_type,
            socket_type: SocketType::Input,
            extensions: Vec::new(),
        }
    }

    pub fn output<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            data_type,
            socket_type: SocketType::Output,
            extensions: Vec::new(),
        }
    }
}