use async_trait::async_trait;
use custom_utils::get_image_from_path;
use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts},
    gtk::{
        self, gdk::Display, glib::clone, pango, prelude::{ButtonExt, WidgetExt}, Button, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION
    },
    AsyncComponentSender,
};
use tokio::{sync::oneshot, time::interval};
use std::{thread::Builder, time::{Duration, Instant}};

use crate::{handlers::provision::handler::LinkMachineHandler,settings::{Modules, WidgetConfigs}};

// use crate::{services::provisionHandler::ProvisionHandler, settings::WidgetConfigs};

pub struct Settings {
    pub modules: Modules,
    pub widget_configs: WidgetConfigs,
}

pub struct LinkMachine {
    settings: Settings,
    connect_code: String,
    progress: f64,
    timer: i32,
    provision_status: bool,
    current_time: i32,
}

#[derive(Debug)]
enum AppInput {
    Increment,
    Decrement,
}

#[derive(Debug)]
pub enum LinkMachineOutput {
    BackPressed,
    NextPressed,
    ShowError
}

#[derive(Debug)]
pub enum InputMessage {
    ActiveScreen(String),
    CodeChanged(String),
    UpdateTimer(f64),
    GenerateCodeError(String),
    ProvisionSuccess,
    ShowError(String),
}

pub struct AppWidgets {
    connect_code_label: gtk::Label,
    progress_bar: gtk::ProgressBar,
    // spinner: gtk::Spinner,
    // timer_label: gtk::Label,
}

const TIMER: i32 = 10;

#[async_trait(?Send)]
impl AsyncComponent for LinkMachine {
    type Init = Settings;
    type Input = InputMessage;
    type Output = LinkMachineOutput;
    type Root = gtk::Box;
    type Widgets = AppWidgets;
    type CommandOutput = ();

    fn init_root() -> Self::Root {
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("../assets/css/style.css"));
        gtk::style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        gtk::Box::builder().build()
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        println!("link machine page init...");

        let modules = init.modules.clone();
        let widget_configs = init.widget_configs.clone();

        let main_content_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["app-container"])
            .build();

        let footer_content_box: gtk::Box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(true)
            .valign(gtk::Align::End)
            .css_classes(["footer-container"])
            .build();

        let header_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            // .css_classes(["link-machine-header"])
            .css_classes(["start-screen-header-box"])
            .build();

        let app_icon_path: Option<String> = modules.pages_settings.start_screen.app_icon.clone();

        let app_icon: gtk::Image = get_image_from_path(
            app_icon_path,
            &["app-icon"],
        );

        let header_label = gtk::Label::builder()
            .label("Link your Machine")
            .halign(gtk::Align::Start)
            .build();

        header_label.style_context().add_class("start-screen-header");

        header_box.append(&app_icon);
        header_box.append(&header_label);

        main_content_box.append(&header_box);

        let header_p = gtk::Label::builder()
            .label("Use this below code to connect this machine to your Mecha account")
            // .css_classes(["link-machine-header-label"])
            .css_classes(["start-screen-header-label"])
            .halign(gtk::Align::Start)
            .build();

        main_content_box.append(&header_p);


        let info_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["start-screen-steps-container"])
        .build();

        // check-code
        let main_code_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["link-machine-border-box"])
            .build();

        let code_label_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .halign(gtk::Align::Start)
            .build();

        // spinner
        let spinner = gtk::Spinner::builder()
            .css_classes(["blue"])
            .height_request(30)
            .width_request(30)
            .build();
        spinner.set_spinning(false);
      
        let connect_code_label = gtk::Label::builder()
            .label("") // ABCD 1234
            .css_classes(["link-machine-code"])
            .build();


        // progress bar
        let progress_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        // .css_classes(["link-machine-progress-box"])
        .build();

        // progressbar
        let progress_bar = gtk::ProgressBar::builder()
        .fraction(1.0)
        .hexpand_set(true)
        .hexpand(true)
        .build();
        progress_bar.style_context().add_class("custom-progress-bar");
        // progress_bar.set_visible(true);

        progress_box.append(&progress_bar);

        code_label_box.append(&connect_code_label);
        main_code_box.append(&code_label_box);
        // main_code_box.append(&spinner);

        info_box.append(&main_code_box);
        info_box.append(&progress_box);
        // main_content_box.append(&main_code_box);

        let main_steps_box: gtk::Box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["link-machine-steps-container"])
            .build();

        let linking_step1_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["link-machine-steps-box"])
            .hexpand(true)
            .build();

        let step1_label_box = gtk::Box::builder()
            .css_classes(["circle-border-box"])
            .valign(gtk::Align::Start)
            .build();

        let step1_label = gtk::Label::builder()
            .label("1")
            .width_request(25)
            .height_request(25)
            .build();
        step1_label_box.append(&step1_label);

        let step1_text = gtk::Label::builder()
            // .label("Sign up on mecha.so")
            .label("Create a new account on Mecha, if not signed up earlier.")
            .css_classes(["link-machine-steps-text"])
            .wrap(true)
            .wrap_mode(pango::WrapMode::Word) 
            .hexpand(true)
            .halign(gtk::Align::Start)
            .build();

        linking_step1_box.append(&step1_label_box);
        linking_step1_box.append(&step1_text);

        main_steps_box.append(&linking_step1_box);

        //
        let linking_step2_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["link-machine-steps-box"])
            .hexpand(true)
            .build();

        let step2_label_box = gtk::Box::builder()
            .css_classes(["circle-border-box"])
            .valign(gtk::Align::Start)
            .build();

        let step2_label = gtk::Label::builder()
            .label("2")
            .width_request(25)
            .height_request(25)
            .build();
        step2_label_box.append(&step2_label);

        let step2_text = gtk::Label::builder()
            // .label("Use the Console app or developer CLI to add a new \nmachine")
            .label("Navigate to Machines > Add Machine")
            .css_classes(["link-machine-steps-text"])
            .build();

        linking_step2_box.append(&step2_label_box);
        linking_step2_box.append(&step2_text);

        main_steps_box.append(&linking_step2_box);

        //
        let linking_step3_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["link-machine-steps-box"])
            .hexpand(true)
            .build();

        let step3_label_box = gtk::Box::builder()
            .css_classes(["circle-border-box"])
            .valign(gtk::Align::Start)
            .build();

        let step3_label = gtk::Label::builder()
            .label("3")
            .width_request(25)
            .height_request(25)
            .build();
        step3_label_box.append(&step3_label);

        let step3_text = gtk::Label::builder()
            // .label("Use this code when asked to enter the provisioning code")
            .label("Enter the code shown above when asked")
            .css_classes(["link-machine-steps-text"])
            .build();

        linking_step3_box.append(&step3_label_box);
        linking_step3_box.append(&step3_text);

        main_steps_box.append(&linking_step3_box);

        // let toast = gtk::InfoBar::new();
        // toast.set_message_type(gtk::MessageType::Error);
        // toast.add_button("HELLO", gtk::ResponseType::None);
        // // info_box.append(toast);

        // main_content_box.append(&main_steps_box);
        info_box.append(&main_steps_box);
        main_content_box.append(&info_box);

        // footer_box
        let footer_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .valign(gtk::Align::End)
            .build();

        let back_icon_img: gtk::Image = get_image_from_path(widget_configs.footer.back_icon, &[]);
        let back_button_box = gtk::Box::builder().hexpand(true).build();
        let back_button = gtk::Button::builder().build();
        back_button.set_child(Some(&back_icon_img));
        back_button.add_css_class("footer-container-button");

        back_button.connect_clicked(clone!(@strong sender => move |_| {
          let _ =  sender.output(LinkMachineOutput::BackPressed);
        }));

        // TEMP : REMVOE THIS LATER - NOT IN UI
        let next_icon_img: gtk::Image = get_image_from_path(widget_configs.footer.next_icon, &[]);
        let next_button = Button::new();
        next_button.set_child(Some(&next_icon_img));
        next_button.add_css_class("footer-container-button");

        next_button.connect_clicked(clone!(@strong sender => move |_| {
          let _ =  sender.output(LinkMachineOutput::NextPressed);
        }));

        back_button_box.append(&back_button);
        footer_box.append(&back_button_box);

        // TEMP : REMVOE THIS LATER
        footer_box.append(&next_button);

        footer_content_box.append(&footer_box);
        main_content_box.append(&footer_content_box);

        root.append(&main_content_box);

        let model = LinkMachine {
            settings: init,
            connect_code: "".to_string(),
            timer: TIMER,
            provision_status: false,
            progress: 0.0,
            current_time: 0,
        };

        let widgets = AppWidgets {
            connect_code_label, 
            progress_bar, 
        };

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        // println!("Inside update {:?}", message);
        let seconds = Duration::from_secs(10);
        let start = Instant::now();

        match message {
            InputMessage::ActiveScreen(text) => {
                let sender: relm4::Sender<InputMessage> = sender.input_sender().clone();
                let result = init_services(sender).await;
            },
            InputMessage::ProvisionSuccess => {
                println!("ProvisionSuccess -> move to NEXT..");
             let _ =  sender.output(LinkMachineOutput::NextPressed);
            },
            InputMessage::CodeChanged(code) => {
                println!("inside InputMessage code change");
                self.connect_code = code.clone();


                let mut total_time = 1.0;
                let mut fraction_value = 0.01;
                let mut g_code_interval = interval(Duration::from_secs(1)); 


                // loop {

                //     g_code_interval.tick().await;

                //     total_time =  total_time.to_owned() - fraction_value.to_owned();
                //     // self.progress = total_time.clone();
                //     // println!("total_time {:?} ", total_time.to_owned());

                //     if total_time == 0.0 {
                //         total_time = 1.0;
                //     }
                // }
                
                // // for _ in 0..60 {
                // //     // tokio::spawn(future)
                // //     fraction_value = total_time-0.01;
                // //     println!("fraction_value {:?} ", fraction_value.to_owned());
                // //     self.progress = fraction_value.clone();

                // //     if fraction_value == 0.0 {
                // //         fraction_value = total_time.clone();
                // //     }
        
                // // }


            },
            InputMessage::UpdateTimer(value) => {
                self.progress = value.clone();
            }
            InputMessage::GenerateCodeError(error) => {
                println!("Generate code error: {:?} ", error);
                println!("SHOW TOAST!");
            },
            InputMessage::ShowError(text) => {
                println!("Error to be shown:: {:?} ", text);
                let _ =  sender.output(LinkMachineOutput::ShowError);
            }
            
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: AsyncComponentSender<Self>) {
        println!("update_view {:?} ", self.connect_code);
        println!("progress {:?} ", self.progress.to_owned());
        widgets.connect_code_label.set_label(&self.connect_code);

        widgets.progress_bar.set_fraction(self.progress);

        // widgets.spinner.set_spinning(true);

        // let result = update_progress(sender);
        // let mut remaining_time = 1.0;
  
        // let mut g_code_interval = interval(Duration::from_secs(1)); 


    }
  
}

async fn init_services(sender: relm4::Sender<InputMessage>) {
    println!("init services called..."); 

    let sender_clone_1 = sender.clone();
    let mut link_machine_handler = LinkMachineHandler::new();
   
    let _ = relm4::spawn(async move {
        let _ = link_machine_handler.run(sender_clone_1).await;
    });
}

async fn update_progress(sender: relm4::Sender<InputMessage>) {
    // let mut time_interval = time::interval(Duration::from_secs(1)); // Tick every second within the 60 seconds

    // let total_time = Duration::from_secs_f64(1.0); // Total time is 1.0 seconds
    // let mut remaining_time = 1.0;

    // _= time_interval.tick() => {
    //     remaining_time -= 0.01;
    //     println!("Time fraction remaining: {:.2}", remaining_time);
    //     // sender
    //     if remaining_time <= 0.0 {
    //         remaining_time = 1.0; // Reset remaining time to 1.0
    //     }
    // }
}
