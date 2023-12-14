use custom_utils::get_image_from_path;
use gtk::{glib::clone, prelude::*};
use relm4::{
    gtk::{self, GestureClick},
    ComponentParts, ComponentSender, SimpleComponent,
};

use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::{
        custom_network_item::{
            CustomNetworkItem, CustomNetworkItemSettings, Message as CustomNetworkItemMessage,
        },
        menu_item::{MenuItem, MenuItemSettings, Message as MenuItemMessage},
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
pub struct BluetoothDetailsPage {
    settings: Settings,
}

//Widgets
pub struct BluetoothDetailsPageWidgets {}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    HomeIconPressed,
}

pub struct SettingItem {
    name: String,
}

impl SimpleComponent for BluetoothDetailsPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = BluetoothDetailsPageWidgets;

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

        let network_name = gtk::Label::builder()
            .label("Macbook Pro")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&network_name);

        let device_type_label = gtk::Label::builder()
            .label("Device Type")
            .css_classes(["bluetooth-details-list-label"])
            .halign(gtk::Align::Start)
            .build();

        let device_type_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["bluetooth-details-device-type-box"])
            .build();

        let device_type_value = gtk::Label::builder()
            .label("Mecha MX")
            .css_classes(["bluetooth-details-device-type-value"])
            .halign(gtk::Align::Start)
            .build();
        device_type_box.append(&device_type_value);

        let forget_network_button = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["bluetooth-details-forget-btn-box"])
            .build();

        let forget_network_text = gtk::Label::builder()
            .label("Forget this network")
            .css_classes(["bluetooth-details-forget-btn-text"])
            .halign(gtk::Align::Center)
            .build();
        forget_network_button.append(&forget_network_text);

        root.append(&header);
        root.append(&device_type_label);
        root.append(&device_type_box);
        root.append(&forget_network_button);

        let footer = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes(["footer"])
        .hexpand(true)
        .vexpand(true)
        .build();

        let back_icon_button = gtk::Box::builder()
            .vexpand(false)
            .hexpand(false)
            .valign(gtk::Align::End)
            .halign(gtk::Align::Start)
            .css_classes(["footer-icon-button"])
            .build();
        let back_icon = get_image_from_path(widget_configs.footer.back_icon, &["back-icon"]);
        back_icon.set_vexpand(true);
        back_icon.set_hexpand(true);
        back_icon.set_halign(gtk::Align::Center);
        back_icon.set_valign(gtk::Align::Center);
        let back_click_gesture = GestureClick::builder().button(0).build();
        back_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
            // sender.input_sender().send(Message::BackPressed);
        }));

        back_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button released is {}", this.current_button());
                let _ = sender.output_sender().send(Message::BackPressed);
        }));
        back_icon_button.append(&back_icon);
        back_icon_button.add_controller(back_click_gesture);

        footer.append(&back_icon_button);
        root.append(&footer);

        let model = BluetoothDetailsPage { settings: init };

        let widgets = BluetoothDetailsPageWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {}
            Message::BackPressed => {  
                let _ = sender.output(Message::BackPressed);}
            Message::HomeIconPressed => {
                let _ = sender.output(Message::HomeIconPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
