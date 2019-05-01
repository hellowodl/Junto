use hdk::{
    holochain_core_types::{
        cas::content::Address, 
        hash::HashString,
        json::{
            JsonString,
            default_to_json
        },
        error::HolochainError
    }
};

use std::collections::HashMap;
use serde::Serialize;

use super::app_definitions;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CreateUserInformation{
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: String,
    pub bio: String
}

//Basic struct to be used to describe a function and its parameters to the handle_hooks & handle_contextual_links functions
pub struct FunctionDescriptor{  
    pub name: &'static str,
    pub parameters: FunctionParameters,
}

#[derive(Serialize, Deserialize)]
pub struct UserDens{
    pub private_den: Option<GetLinksLoadElement<app_definitions::Channel>>,
    pub shared_den: Option<GetLinksLoadElement<app_definitions::Channel>>,
    pub public_den: Option<GetLinksLoadElement<app_definitions::Channel>>
}

#[derive(Serialize, Deserialize)]
pub struct UserPack{
    pub pack: Option<GetLinksLoadElement<app_definitions::Group>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackMembers{
    pub members: Vec<GetLinksLoadElement<app_definitions::UserName>>
}

pub enum QueryTarget{
    ExpressionPost,
    User
}

pub enum QueryOptions {
    FilterPopular,
    FilterNew,
    FilterOld
}

pub type GetLinksLoadResult<T> = Vec<GetLinksLoadElement<T>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetLinksLoadElement<T> {
	pub address: HashString,
	pub entry: T
}

impl From<UserDens> for JsonString {
    fn from(result: UserDens) -> JsonString {
        JsonString::from(json!({
            "private_den": match result.private_den{ 
                Some(den) => default_to_json(den),
                None => default_to_json("{}")
            },
            "shared_den": match result.shared_den{
                Some(den) => default_to_json(den),
                None => default_to_json("{}")
            },
            "public_den": match result.public_den{
                Some(den) => default_to_json(den),
                None => default_to_json("{}")
            }
        }))
    }
}

impl From<PackMembers> for JsonString {
    fn from(result: PackMembers) -> JsonString {
        JsonString::from(json!(default_to_json(result)))
    }
}

impl From<UserPack> for JsonString {
    fn from(result: UserPack) -> JsonString {
        match result.pack{
            Some(group) => JsonString::from(group),
            None => JsonString::from_json("{}")
        }
    }
}

impl<T: Into<JsonString>> From<GetLinksLoadElement<T>> for JsonString  where T: Serialize{
    fn from(result: GetLinksLoadElement<T>) -> JsonString {
        let entry = serde_json::to_string(&result.entry);
        let entry_string: String;
        match entry {
            Ok(entry) => entry_string = entry,
            Err(e) => return JsonString::from(HolochainError::SerializationError(e.to_string()))
        };
        let address = serde_json::to_string(&result.address);
        let address_string: String;
        match address{
            Ok(address) => address_string = address,
            Err(e) => return JsonString::from(HolochainError::SerializationError(e.to_string()))
        }

        json!(&format!("{{\"address\": {}, \"entry\": {}}}", address_string, entry_string)).into()
    }
}

impl<T> PartialEq for GetLinksLoadElement<T>{
    fn eq(self: &Self, other: &GetLinksLoadElement<T>) -> bool {
        self.address == other.address
    }
}

//Parameters for each function in holochain application
pub enum FunctionParameters{
    GlobalTimeToExpression{
        tag: &'static str, 
        direction: &'static str, 
        expression_address: Address,
    },
    LocalTimeToExpression{
        tag: &'static str, 
        direction: &'static str, 
        expression_address: Address,
        context: Address,
    },
    CreatePack{
        username_address: Address,
        first_name: String
    },
    CreateDen{
        username_address: Address,
        first_name: String
    },
    LinkExpression{
        tag: &'static str, 
        direction: &'static str, 
        parent_expression: Address, 
        child_expression: Address
    },
    CreateChannels{
        channels: Vec<String>,
        parent: Address,
        privacy: app_definitions::Privacy
    },
    CreateQueryPoints{
        query_points: Vec<HashMap<String, String>>, 
        context: Address, 
        privacy: app_definitions::Privacy,
        query_type: String,
        expression: Address
    }
}
