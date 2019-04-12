//Module to handle all channel related operations
use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        entry::Entry, 
        cas::content::Address,
        hash::HashString
    }
};

use std::collections::HashMap;
use itertools::Itertools;

use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters
    }
};

pub fn create_query_points(query_points: Vec<HashMap<String, String>>, context: &Address, privacy: &app_definitions::Privacy, 
                            query_type: &String, expression: &Address) -> ZomeApiResult<String>{
    let mut addressed_params: Vec<HashMap<String, String>> = query_points.to_vec();
    hdk::api::link_entries(&context, &expression, "expression")?;
    for (i, query_param) in query_points.iter().enumerate(){
        match query_param["type"].as_ref(){
            "Channel" => {
                let entry = Entry::App("channel".into(), app_definitions::Channel{name: query_param["value"].to_string(), 
                                                    parent: context.clone(), privacy: privacy.clone(), channel_type: app_definitions::ChannelType::Tag}.into()).into();
                let address = hdk::entry_address(&entry)?;
                let dht_entry = hdk::get_entry(&address)?;

                match dht_entry {
                    Some(_value) => {
                        //No checks to see if there is a link on context need to made - chanels can only be created in the None block here and thus must have been linked to the context
                        hdk::api::link_entries(&address, expression, "expression")?;
                        hdk::api::link_entries(&address, expression, query_param["value"].to_string())?;
                        hdk::api::link_entries(&expression, &address, "channel")?;
                        addressed_params[i].insert("address".to_string(), address.to_string()); 
                    }, 
                    None => {
                        let address = hdk::utils::commit_and_link(&entry, context, "channel")?;
                        hdk::api::link_entries(&address, expression, "expression")?;
                        hdk::api::link_entries(&address, expression, query_param["value"].to_string())?;
                        hdk::api::link_entries(&expression, &address, "channel")?;
                        addressed_params[i].insert("address".to_string(), address.to_string());
                    }
                };
            },
            "Type" => {
                let entry = Entry::App("channel".into(), app_definitions::Channel{name: query_param["value"].to_string(), 
                                                    parent: context.clone(), privacy: privacy.clone(), channel_type: app_definitions::ChannelType::Type}.into()).into();
                let address = hdk::entry_address(&entry)?;
                let dht_entry = hdk::get_entry(&address)?;

                match dht_entry {
                    Some(_value) => {
                        hdk::api::link_entries(&address, expression, "expression")?;
                        hdk::api::link_entries(&address, expression, query_param["value"].to_string())?;
                        hdk::api::link_entries(expression, &address, "type")?;
                        addressed_params[i].insert("address".to_string(), address.to_string());
                    },
                    None => {
                        let address = hdk::utils::commit_and_link(&entry, context, "channel")?;
                        hdk::api::link_entries(&address, expression, "expression")?;
                        hdk::api::link_entries(&address, expression, query_param["value"].to_string())?;
                        hdk::api::link_entries(expression, &address, "type")?;
                        addressed_params[i].insert("address".to_string(), address.to_string());
                    }
                };
            },
            "Time:Y" => {
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: context.clone(), time_type: app_definitions::TimeType::Year}.into()).into();
                let address = hdk::entry_address(&entry)?;
                let dht_entry = hdk::get_entry(&address)?;

                match dht_entry {
                    Some(_value) => {
                        hdk::api::link_entries(&address, expression, "expression")?;
                        hdk::api::link_entries(&address, expression, query_param["value"].to_string())?;
                        hdk::api::link_entries(expression, &address, "time")?;
                        hdk::api::link_entries(expression, &address, "year")?;
                        addressed_params[i].insert("address".to_string(), address.to_string());
                    },
                    None => {
                        let address = hdk::utils::commit_and_link(&entry, context, "time")?;
                        hdk::api::link_entries(context, &address, "year")?;
                        hdk::api::link_entries(&address, expression, "expression")?;
                        hdk::api::link_entries(&address, expression, query_param["value"].to_string())?;
                        hdk::api::link_entries(&expression, &address, "time")?;
                        hdk::api::link_entries(&expression, &address, "year")?;
                        addressed_params[i].insert("address".to_string(), address.to_string());
                    }
                };
            },
            "Time:M" => {
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: context.clone(), time_type: app_definitions::TimeType::Month}.into()).into();
                let address = hdk::entry_address(&entry)?;
                let dht_entry = hdk::get_entry(&address)?;

                match dht_entry {
                    Some(_value) => {
                        hdk::api::link_entries(&address, expression, "expression")?;
                        hdk::api::link_entries(&address, expression, query_param["value"].to_string())?;
                        hdk::api::link_entries(&expression, &address, "time")?;
                        hdk::api::link_entries(&expression, &address, "month")?;
                        addressed_params[i].insert("address".to_string(), address.to_string());
                    },
                    None => {
                        let address = hdk::utils::commit_and_link(&entry, context, "time")?;
                        hdk::api::link_entries(context, &address, "month")?;
                        hdk::api::link_entries(&address, expression, "expression")?;
                        hdk::api::link_entries(&address, expression, query_param["value"].to_string())?;
                        hdk::api::link_entries(&expression, &address, "time")?;
                        hdk::api::link_entries(&expression, &address, "month")?;
                        addressed_params[i].insert("address".to_string(), address.to_string());
                    }
                };
            },
            "Time:D" => {
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: context.clone(), time_type: app_definitions::TimeType::Day}.into()).into();
                let address = hdk::entry_address(&entry)?;
                let dht_entry = hdk::get_entry(&address)?;

                match dht_entry {
                    Some(_value) => {
                        hdk::api::link_entries(&address, expression, "expression")?;
                        hdk::api::link_entries(&address, expression, query_param["value"].to_string())?;
                        hdk::api::link_entries(&expression, &address, "time")?;
                        hdk::api::link_entries(&expression, &address, "day")?;
                        addressed_params[i].insert("address".to_string(), address.to_string());
                    },
                    None => {
                        let address = hdk::utils::commit_and_link(&entry, context, "time")?;
                        hdk::api::link_entries(context, &address, "day")?;
                        hdk::api::link_entries(&address, expression, "expression")?;
                        hdk::api::link_entries(&address, expression, query_param["value"].to_string())?;
                        hdk::api::link_entries(&expression, &address, "time")?;
                        hdk::api::link_entries(&expression, &address, "day")?;
                        addressed_params[i].insert("address".to_string(), address.to_string());
                    }
                };
            },
            "Time:H" => {
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: context.clone(), time_type: app_definitions::TimeType::Hour}.into()).into();
                let address = hdk::entry_address(&entry)?;
                let dht_entry = hdk::get_entry(&address)?;

                match dht_entry {
                    Some(_value) => {
                        hdk::api::link_entries(&address, expression, "expression")?;
                        hdk::api::link_entries(&address, expression, query_param["value"].to_string())?;
                        hdk::api::link_entries(&expression, &address, "time")?;
                        hdk::api::link_entries(&expression, &address, "hour")?;
                        addressed_params[i].insert("address".to_string(), address.to_string());
                    },
                    None => {
                        let address = hdk::utils::commit_and_link(&entry, context, "time")?;
                        hdk::api::link_entries(&context, &address, "hour")?;
                        hdk::api::link_entries(&address, expression, "expression")?;
                        hdk::api::link_entries(&address, expression, query_param["value"].to_string())?;
                        hdk::api::link_entries(&expression, &address, "time")?;
                        hdk::api::link_entries(&expression, &address, "hour")?;
                        addressed_params[i].insert("address".to_string(), address.to_string());
                    }
                };
            },
            "user" => {
                let entry = Entry::App("username".into(), app_definitions::UserName{username: query_param["value"].to_string()}.into()).into();
                let address = hdk::entry_address(&entry)?;
                let dht_entry = hdk::get_entry(&address)?;

                match dht_entry{
                    Some(_value) => {
                        hdk::api::link_entries(&address, &expression, "expression")?;
                        hdk::api::link_entries(&expression, &address, "user")?;
                        addressed_params[i].insert("address".to_string(), address.to_string());
                    },
                    None => {
                        return Err(ZomeApiError::from("No user with given username string".to_string()))
                    }
                };
            },
            _ => {
                return Err(ZomeApiError::from("That contextual link type does not exist".to_string()))
            }
        };
    };

    if query_type == "contextual" {
        create_contextual_links(&addressed_params, expression)?;
    };
    Ok("ok".to_string())
}

//Only expression posts are being indexed using contextual_links
//any other information must be found using "normal" link querying - users can be found by getting owners of expression posts
pub fn create_contextual_links(query_points: &Vec<HashMap<String, String>>, expression: &Address) -> ZomeApiResult<String>{
    let mut link_combinations = vec![]; //Vector for link combinations on expression

    for (i, _) in query_points.iter().enumerate(){
        let combinations = query_points.iter().combinations(i);
        for c in combinations.into_iter(){
            link_combinations.push(c);
        };
    };
    link_combinations.push(query_points.iter().collect());
    link_combinations = link_combinations[1..link_combinations.len()].to_vec();

    for link in link_combinations{ //Create link combinations for expression indexing
        let start = link[0];
        let link_strings: Vec<String> = link.iter().map(|link_value| format!("{}<{}>", link_value["value"].to_lowercase(), link_value["type"].to_lowercase(),) ).collect();
        let link_string = link_strings.join(":");
        hdk::api::link_entries(&HashString::from(start["address"].clone()), expression, link_string)?;
    };
    Ok("ok".to_string())
}