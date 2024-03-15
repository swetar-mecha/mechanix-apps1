use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};

use crate::{server::provision_client::ProvisionManagerClient, settings::{Modules, WidgetConfigs}};
use async_trait::async_trait;
use custom_utils::{get_gif_from_path, get_image_from_path};
use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts}, gtk::{
        self,
        glib::clone,
        prelude::{ButtonExt, WidgetExt},
        Button,
    }, AsyncComponentSender
};
use tokio::sync::oneshot;

pub struct Settings {
    pub modules: Modules,
    pub widget_configs: WidgetConfigs,
}
pub struct CheckInternet {
    settings: Settings,
    task: Option<tokio::task::JoinHandle<()>>,
    cancel_flag: Arc<AtomicBool>,
}

#[derive(Debug)]
enum AppInput {
}

#[derive(Debug)]
pub enum InputMessage {
    ActiveScreen(String),
    NextScreen,
    ConnectionNotFound,
    ShowError(String),
    BackScreen
}

#[derive(Debug)]
pub enum CheckInternetOutput {
    BackPressed,
    LinkMachine,
    ConnectionNotFound,
    ShowError(String)
}

pub struct AppWidgets {}

#[async_trait(?Send)]
impl AsyncComponent for CheckInternet {
    type Init = Settings;
    type Input = InputMessage;
    type Output = CheckInternetOutput;
    type Root = gtk::Box;
    type Widgets = AppWidgets;
    type CommandOutput = ();

    fn init_root() -> Self::Root {
        gtk::Box::builder().build()
    }

    /// Initialize the UI and model.
    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let modules = init.modules.clone();
        let widget_configs = init.widget_configs.clone();

        let cancel_flag = Arc::new(AtomicBool::new(false));

        let model = CheckInternet {settings:init, task: None, cancel_flag:  cancel_flag.clone() } ;

        let main_content_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["app-container", "check-internet-text"])
            .build();

        let footer_content_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(true)
            .valign(gtk::Align::End)
            .css_classes(["footer-container"])
            .build();

        // get gif
        let gif_path = modules.pages_settings.check_internet.search_wifi.clone();
        let paintable = get_gif_from_path(gif_path);

        let image_from = gtk::Image::builder()
            .width_request(370)
            .height_request(370)
            .paintable(&paintable)
            .css_classes(["gif-img"])
            .build();

        let label1: gtk::Label = gtk::Label::builder()
            .label("Checking for internet connectivity...")
            .build();

        // let label2: gtk::Label = gtk::Label::builder().label("Please wait").build();

        main_content_box.append(&image_from);
        main_content_box.append(&label1);
        // main_content_box.append(&label2);

        let footer_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .build();

        let back_icon_img: gtk::Image = get_image_from_path(widget_configs.footer.back_icon, &[]);
        let back_button_box = gtk::Box::builder().hexpand(true).build();
        let back_button = Button::builder().build();
        back_button.set_child(Some(&back_icon_img));
        back_button.add_css_class("footer-container-button");

        back_button.connect_clicked(clone!(@strong sender => move |_| {
            let _ =  sender.input_sender().send(InputMessage::BackScreen);
            // let _ =  sender.output(CheckInternetOutput::BackPressed);
        }));

        let next_icon_img: gtk::Image = get_image_from_path(widget_configs.footer.next_icon, &[]);
        let next_button = Button::new();
        next_button.set_child(Some(&next_icon_img));
        next_button.add_css_class("footer-container-button");

        next_button.connect_clicked(clone!(@strong sender => move |_| {
          let _ =  sender.output(CheckInternetOutput::LinkMachine);
        }));

        back_button_box.append(&back_button);
        footer_box.append(&back_button_box);
        footer_box.append(&next_button);

        footer_content_box.append(&footer_box);
        main_content_box.append(&footer_content_box);

        root.append(&main_content_box);

        let widgets = AppWidgets {};

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) { 
        match message {
            InputMessage::ActiveScreen(text) => {
                println!("active screen: {:?}", text);

                let cancel_flag_clone = self.cancel_flag.clone();

                let sender: relm4::Sender<InputMessage> = sender.input_sender().clone();
                // self.task = init_services(sender).await;

                // let task = tokio::spawn(async move {
                //     let time_duration=Duration::from_millis(7000);
                //     let _ = tokio::time::sleep(time_duration).await;
                //     let _ = init_services(sender).await;
                //     println!("ASYNC CALL");
                // });

                println!("cancel_flag_clone og {:?} ", cancel_flag_clone.to_owned());
                println!("cancel_flag_clone check{:?} ", !cancel_flag_clone.load(Ordering::SeqCst));

                if !cancel_flag_clone.load(Ordering::SeqCst) {
                    let task = tokio::spawn(async move {
                        let time_duration=Duration::from_millis(7000);
                        let _ = tokio::time::sleep(time_duration).await;
                        let _ = init_services(sender).await;
                        println!("ASYNC CALL");
                    });
                }

                // self.task = Some(task);
                // self.cancel_rx = Some(cancel_tx);
            },
            InputMessage::BackScreen => {
                println!("REDIRECT TO BACK SCREEN");
                println!("BackScreen cancel_flag_clone {:?} ", self.cancel_flag.clone());

                // if let Some(task) = self.task.take() {
                //     task.abort_handle(); // Cancel pending API task
                //     // task.abort(); // Cancel pending API task
                // }
                self.cancel_flag.store(true, Ordering::SeqCst);
                let _ =  sender.output(CheckInternetOutput::BackPressed);
                // let _ =  sender.output(CheckInternetOutput::LinkMachine);

            },
            InputMessage::NextScreen => {
                let _ =  sender.output(CheckInternetOutput::LinkMachine);
            },
            InputMessage::ConnectionNotFound => {
                let _ =  sender.output(CheckInternetOutput::ConnectionNotFound);
            }
            InputMessage::ShowError(text) => {
                let _ =  sender.output(CheckInternetOutput::ShowError(text));
            }
        }
        
    }
}

async fn init_services(sender: relm4::Sender<InputMessage>) {
    println!("init services called...");

    // let time_duration=Duration::from_millis(7000);
    // // let time_duration=Duration::from_millis(400000);
    // let _ = tokio::time::sleep(time_duration).await;

    println!("INIT SERVICE IN PROGRESS....");



    match ProvisionManagerClient::new().await {
        Ok(mut client) => {
            match client.ping().await {
                Ok(response) => {
                    println!("ping response {:?} ", response);
                    if response.code == "success" {
                    let _ = sender.send(InputMessage::NextScreen);
                    }
                    else {
                    let _ = sender.send(InputMessage::ConnectionNotFound);
                    }
                },
                Err(error) => {
                    // ping error: status: Internal, message: "deadline has elapsed", details: [], metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Mon, 11 Mar 2024 07:31:12 GMT", "content-length": "0"} }

                    // ping error: status: Internal, message: "timeout while receiving message from channel", details: [], metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Mon, 11 Mar 2024 07:48:28 GMT", "content-length": "0"} }

                    

                    eprintln!("ping error: {}", error);
                    let _ = sender.send(InputMessage::ShowError("Try after some time!".to_owned()));
                }
            }
        },
        Err(error) => {

            // agent connection refused! - Handler error screen
            // Client error :: Error in ProvisioningServiceClient: tonic::transport::Error(Transport, hyper::Error(Connect, ConnectError("tcp connect error", Os { code: 111, kind: ConnectionRefused, message: "Connection refused" }))) 
            // let _ = sender.send(InputMessage::ConnectionNotFound);
            
            // Error connecting to service

            println!("Client error :: {} ", error);
            let _ = sender.send(InputMessage::ShowError("Machine Agent is not running".to_owned()));


        }
    };

}
