use gtk::prelude::*;
use relm4::{component::{AsyncComponent, AsyncComponentParts}, gtk::{self, glib::clone, pango, prelude::{ButtonExt, WidgetExt}, Button}, AsyncComponentSender, ComponentParts, ComponentSender, RelmApp, SimpleComponent};
use tonic::async_trait;
use crate::settings::{Modules, WidgetConfigs};
use custom_utils::get_image_from_path;

pub struct Settings {
    pub modules: Modules,
    pub widget_configs: WidgetConfigs,
}

pub struct MachineInfo {
    settings: Settings
}

#[derive(Debug)]
enum AppInput {
    Increment,
    Decrement,
}

#[derive(Debug)]
pub enum InputMessage {
    ActiveScreen(String),
}


#[derive(Debug)]
pub enum DevicePageOutput {
    BackPressed,
    NextPressed
}

pub struct AppWidgets {
}

#[async_trait(?Send)]
impl AsyncComponent for MachineInfo {

    type Init = Settings;
    type Input = InputMessage;
    type Output = DevicePageOutput;
    type Root = gtk::Box;
    type Widgets = AppWidgets;
    type CommandOutput = ();

    fn init_root() -> Self::Root {

        gtk::Box::builder()
        .build()

    }

    /// Initialize the UI and model.
    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let modules = init.modules.clone();
        let widget_configs = init.widget_configs.clone();

        let model = MachineInfo { settings: init };

        let main_content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical) 
        .css_classes(["app-container"])
        .halign(gtk::Align::Fill)
        .build();

        let footer_content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .vexpand(true)
        .valign(gtk::Align::End)
        .css_classes(["footer-container"])
        .build();

        let user_profile_icon: gtk::Image = get_image_from_path(
            modules.pages_settings.device_info.user_profile_img.clone(),
            &["device-info-icon"],
        );
      
        main_content_box.append(&user_profile_icon);

        let status_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

        // bold
        let user_label: gtk::Label = gtk::Label::builder()
        .label("Shoaib's Compute")
        .halign(gtk::Align::Center)
        .css_classes(["about-device-name"])
        .build();

        let status_img: gtk::Image = get_image_from_path(
            modules.pages_settings.device_info.active_status_icon.clone(),
            &["device-info-status-icon"],
        );

        status_box.append(&user_label);
        status_box.append(&status_img);

        main_content_box.append(&status_box);

        let id_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["device-info-border-box"])
        .build();

        let id_label: gtk::Label = gtk::Label::builder()
        .label("Machine ID")
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["device-id-text", "about-device-id"])
        .build();

        let id_value: gtk::Label = gtk::Label::builder()
        .label("1675 5467 398765")
        .halign(gtk::Align::End)
        .css_classes(["about-device-id"])
        .build();

        id_box.append(&id_label);
        id_box.append(&id_value);

        let certi_box_1 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["device-info-border-box"])
        .build();

        let certificate_serial_no: gtk::Label = gtk::Label::builder()
        .label("Certificate Serial No")
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["device-id-text", "about-device-id"])
        .build();

        let serial_no_value: gtk::Label = gtk::Label::builder()
        .label("GCR/BC/30124622")
        .halign(gtk::Align::End)
        .css_classes(["about-device-id"])
        .build();
     

        certi_box_1.append(&certificate_serial_no);
        certi_box_1.append(&serial_no_value);

        let certi_box_2 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["device-info-border-box"])
        .build();

        let certificate_valid_upto: gtk::Label = gtk::Label::builder()
        .label("Certificate Valid Upto")
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["device-id-text", "about-device-id"])
        .build();

        let certi_valid_value: gtk::Label = gtk::Label::builder()
        .label("12 March 2029")
        .halign(gtk::Align::End)
        .css_classes(["about-device-id"])
        .build();
     

        certi_box_2.append(&certificate_valid_upto);
        certi_box_2.append(&certi_valid_value);

        main_content_box.append(&id_box);
        main_content_box.append(&certi_box_1);
        // main_content_box.append(&certi_box_2);

        let sentence_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["device-info-sentence"])
        .build();

        let sentence: gtk::Label = gtk::Label::builder()
        .label("You can unlink your machine from your Mecha account")
        .wrap(true)
        .wrap_mode(pango::WrapMode::Word) 
        .hexpand(true)
        .build();

        sentence_box.append(&sentence);
        main_content_box.append(&sentence_box);

        let footer_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .vexpand(true)
        .build();

        let button_box = gtk::Box::builder().hexpand(true).build();

        let exit_icon_img: gtk::Image = get_image_from_path(
            widget_configs.footer.exit_icon,
            &[],
        );
      
        let exit_button = Button::builder().build();
        exit_button.set_child(Some(&exit_icon_img));
        exit_button.add_css_class("footer-container-button");

        exit_button.connect_clicked(clone!(@strong sender => move |_| {
            let _ =  sender.output(DevicePageOutput::BackPressed);
          }));

        // let back_icon_img: gtk::Image = get_image_from_path(
        //    widget_configs.footer.back_icon,
        //    &[],
        // );
        // let back_button = Button::builder().build();
        // back_button.set_child(Some(&back_icon_img));
        // back_button.add_css_class("footer-container-button");

        // back_button.connect_clicked(clone!(@strong sender => move |_| {
        //     let _ =  sender.output(DevicePageOutput::BackPressed);
        //   }));


        // let trash_icon_img: gtk::Image = get_image_from_path(
        //     widget_configs.footer.trash_icon,
        //     &[],
        // );
        // let trash_button = Button::new();
        // trash_button.set_child(Some(&trash_icon_img));
        // trash_button.add_css_class("footer-container-button");


        // trash_button.connect_clicked(clone!(@strong sender => move |_| {
        //     let _ =  sender.output(DevicePageOutput::NextPressed);
        //   }));
        // footer_box.append(&trash_button);


        footer_box.append(&button_box);
        footer_box.append(&exit_button);

        footer_content_box.append(&footer_box);
        main_content_box.append(&footer_content_box);

        root.append(&main_content_box);

        let widgets = AppWidgets {  };

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) { 

    }


}

fn init_services(sender: relm4::Sender<InputMessage>) {
    println!("device info init services called..."); 
}
