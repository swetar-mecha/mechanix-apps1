use std::{io::Error, time::Duration};
use anyhow::{bail, Result};
use relm4::Sender;
use tokio::{select, sync::mpsc, time};

use crate::{pages::configure_machine::InputMessage as Message, server::{identity_client::{self, GetMachineIdRequest, IdentityClient}, settings_client::{GetSettingsRequest, SettingsClient}}};

// pub struct MachineInfoHandler {
//     machine_id: Option<String>,
    
// }

#[derive(Debug)]
pub struct MachineInfoHandler {
    machine_id: Option<String>,
    name: Option<String>,
    icon: Option<String>,
}

#[derive(Debug)] 
pub struct  MachineInfoError{
    error_message: String,
    error_code: String
}

impl MachineInfoHandler {

    pub fn new() -> Self {
        Self {
            machine_id: None,
            name: None,
            icon: None,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {

        println!("Machine info handler run..");

        // get machine id
        let identity_client_response = IdentityClient::new().await;
        let mut identity_client: IdentityClient = match identity_client_response {
            Ok(result) => {
                println!("identity_client_response : {:?} ", result);
                result.into()
            },
            Err(error) => {
                println!("identity_client_response error: {:?} ", error);
                todo!()
            },
            
        };


        match identity_client.getting_machine_id().await {
            Ok(response) => {
                // identity_response : GetMachineIdResponse { machine_id: "c6c8f5c44262-utjf-228" } 

                println!("identity_response : {:?} ", response);
                self.machine_id = Some(response.machine_id);
            },
            Err(error) => {
                println!("identity_response err : {:?} ", error);
            }
        };

        // get name & icon
        let settings_client_response = SettingsClient::new().await;
        let mut settings_client: SettingsClient = match settings_client_response {
            Ok(result) => {
                println!("settings_client_response : {:?} ", result);
                result.into()
            },
            Err(error) => {
                println!("settings_client_response error: {:?} ", error);
                todo!()
            },
        };

        let name_request_obj = GetSettingsRequest {
            key: "identity.machine.name".to_owned(),
        };

        let icon_request_obj = GetSettingsRequest {
            key: "identity.machine.icon".to_owned(),
        };

        match settings_client.get_settings_data(name_request_obj.key).await {
            Ok(response) => {
                println!("name_request : {:?} ", response);
                self.name = Some(response.value);
            },
            Err(error) => {
                println!("name_request err : {:?} ", error);
            }
        };

        match settings_client.get_settings_data(icon_request_obj.key).await {
            Ok(response) => {
                println!("icon_request : {:?} ", response);
                self.icon = Some(response.value);
            },
            Err(error) => {
                println!("icon_request err : {:?} ", error);
            }
        };

    }


}

