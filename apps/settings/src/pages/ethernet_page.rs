use custom_utils::get_image_from_path;
use gtk::{glib::clone, prelude::*};
use relm4::{
    gtk::{self, GestureClick},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent,
};

use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::{
        custom_list_item::{
            CustomListItem, CustomListItemSettings, Message as CustomListItemMessage,
        }, 
    },
};
use tracing::info;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct EthernetPage {
    settings: Settings,
}

//Widgets
pub struct EthernetPageWidgets {}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    IPSettingsPressed,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for EthernetPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = EthernetPageWidgets;

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
            .label("Ethernet")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_title);

        let enable_status_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["network-details-box"])
            .build();

        let enable_row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["network-details-box-row"])
            .build();

        let enable_text = gtk::Label::builder()
            .label("Enable Ethernet")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(["custom-switch-text"])
            .build();

        let switch = gtk::Switch::new();
        switch.set_active(true);
        let style_context = switch.style_context();
        style_context.add_class("custom-switch");

        enable_row.append(&enable_text);
        enable_row.append(&switch);
        enable_status_box.append(&enable_row);

        let items_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

        let ip_settings = CustomListItem::builder()
            .launch(CustomListItemSettings {
                start_icon: None,
                text: "IP Settings".to_string(),
                value: "".to_owned(),
                end_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("ETHERNET PAGE msg is {:?}", msg);
                match msg { 
                    CustomListItemMessage::WidgetClicked => Message::IPSettingsPressed,
                }
            });

        let ip_settings_widget = ip_settings.widget();

        items_box.append(ip_settings_widget);

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
        scrollable_content.append(&enable_status_box);
        scrollable_content.append(&items_box);

        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .min_content_height(360)
            .css_classes(["scrollable"])
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

        let back_icon_button = gtk::Box::builder()
            .vexpand(false)
            .hexpand(false)
            .valign(gtk::Align::Center)
            .css_classes(["footer-icon-button"])
            .build();

        let back_icon = get_image_from_path(widget_configs.footer.back_icon, &["back-icon"]);
        back_icon.set_vexpand(true);
        back_icon.set_hexpand(true);
        back_icon.set_halign(gtk::Align::Center);
        back_icon.set_valign(gtk::Align::Center);
        let left_click_gesture = GestureClick::builder().button(0).build();
        left_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
            // sender.input_sender().send(Message::BackPressed);

        }));

        left_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button released is {}", this.current_button());
                let _ = sender.output_sender().send(Message::BackPressed);

        }));
        back_icon_button.add_controller(left_click_gesture);
        back_icon_button.append(&back_icon);
        footer.append(&back_icon_button);

        root.append(&footer);

        let model = EthernetPage { settings: init };

        let widgets = EthernetPageWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {}
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            },
            Message::IPSettingsPressed => {
                let _ = sender.output(Message::IPSettingsPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
