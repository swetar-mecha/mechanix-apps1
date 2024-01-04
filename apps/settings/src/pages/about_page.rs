use custom_utils::get_image_from_path;
use gtk::prelude::*;
use relm4::{
    gtk::{self},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent, Controller,
};
use crate::settings::{LayoutSettings, Modules, WidgetConfigs};
use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonStetings, OutputMessage as IconButtonOutputMessage,
};
use tracing::info;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct AboutPage {
    settings: Settings,
}

//Widgets
pub struct AboutPageWidgets {
    back_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message { 
    BackPressed,
    HomeIconPressed,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for AboutPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = AboutPageWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["page-container"])
            .build()
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let modules = init.modules.clone();
        let layout = init.layout.clone();
        let widget_configs = init.widget_configs.clone();

        let header_title = gtk::Label::builder()
            .label("About")
            .css_classes(["header-title"])
            .build();
        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();
        header.append(&header_title);

        let about_details_list1 = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["settings-item-details-box"])
            .build();
        
        let about_details_list2 = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["settings-item-details-box"])
            .build();

        let about_details_row_1 = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["settings-item-details-box-row"])
            .build();

        let os_label = gtk::Label::builder()
        .label("OS")
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        let os_value = gtk::Label::builder()
        .label("Mechanix OS")
        .hexpand(true)
        .halign(gtk::Align::End)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        about_details_row_1.append(&os_label);
        about_details_row_1.append(&os_value);
        about_details_list1.append(&about_details_row_1);

        let about_details_row_2 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["settings-item-details-box-row"])
        .build();

        let version_label = gtk::Label::builder()
        .label("Version")
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        let version_value = gtk::Label::builder()
        .label("24.01")
        .hexpand(true)
        .halign(gtk::Align::End)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        about_details_row_2.append(&version_label);
        about_details_row_2.append(&version_value);
        about_details_list1.append(&about_details_row_2);

        let about_details_row_3 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["settings-item-details-box-row"])
        .build();

        let serial_no_label = gtk::Label::builder()
        .label("Serial Number")
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        let serial_no_value = gtk::Label::builder()
        .label("1245 6789")
        .hexpand(true)
        .halign(gtk::Align::End)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        about_details_row_3.append(&serial_no_label);
        about_details_row_3.append(&serial_no_value);
        about_details_list2.append(&about_details_row_3);


        let about_details_row_4 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["settings-item-details-box-row"])
        .build();

        let wifi_address_label = gtk::Label::builder()
        .label("Wi-Fi MAC Address")
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        let wifi_address_value = gtk::Label::builder()
        .label("B0:35:B5:DA:A6:75")
        .hexpand(true)
        .halign(gtk::Align::End)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        about_details_row_4.append(&wifi_address_label);
        about_details_row_4.append(&wifi_address_value);
        about_details_list2.append(&about_details_row_4);


        let about_details_row_5 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["settings-item-details-box-row"])
        .build();

        let ethernet_address_label = gtk::Label::builder()
        .label("Ethernet MAC Address")
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        let ethernet_address_value = gtk::Label::builder()
        .label("B0:35:B5:DA:A6:75")
        .hexpand(true)
        .halign(gtk::Align::End)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        about_details_row_5.append(&ethernet_address_label);
        about_details_row_5.append(&ethernet_address_value);
        about_details_list2.append(&about_details_row_5);

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        scrollable_content.append(&about_details_list1);
        scrollable_content.append(&about_details_list2);

        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .min_content_height(360)
            .child(&scrollable_content)
            .build();
        root.append(&scrolled_window);

        let footer = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes(["footer"])
        .hexpand(true)
        .vexpand(true)
        .valign(gtk::Align::End)
        .build();

        let back_button = IconButton::builder()
            .launch(IconButtonStetings {
                icon: widget_configs.footer.back_icon.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconButtonOutputMessage::Clicked => Message::BackPressed,
            });

        footer.append(back_button.widget());

        root.append(&footer);

        let model = AboutPage { settings: init };

        let widgets = AboutPageWidgets {
            back_button
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("About- Update message is {:?}", message);
        match message {
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            },
            Message::HomeIconPressed => {
                let _ = sender.output(Message::HomeIconPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
