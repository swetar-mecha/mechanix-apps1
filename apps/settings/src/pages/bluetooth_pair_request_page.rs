use custom_utils::get_image_from_path;
use gtk::{glib::clone, prelude::*};
use relm4::{
    gtk::{self, GestureClick},
    ComponentParts, ComponentSender, SimpleComponent, Controller,
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
pub struct BluetoothPairRequestPage {
    settings: Settings,
}

//Widgets
pub struct BluetoothPairRequestPageWidgets {}

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

impl SimpleComponent for BluetoothPairRequestPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = BluetoothPairRequestPageWidgets;

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
            .label("Pairing Request")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_title);

        let pairing_request_label = gtk::Label::builder()
            .label("'Macbook Pro' has requested to pair with your device. \nConfirm this code on their device to connect.")
            .css_classes(["bluetooth-pair-request-list-label"])
            .halign(gtk::Align::Start)
            .build();

        let pairing_request_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["bluetooth-pair-request-box"])
            .build();

        let pairing_request_value = gtk::Label::builder()
            .label("243 562")
            .css_classes(["bluetooth-pair-request-value"])
            .halign(gtk::Align::Center)
            .build();
        pairing_request_box.append(&pairing_request_value);

        root.append(&header);
        root.append(&pairing_request_label);
        root.append(&pairing_request_box);

        let footer = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes(["footer"])
        .hexpand(true)
        .vexpand(true)
        .build();

        let footer_expand_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .valign(gtk::Align::End)
            .build();

        let back_icon_button = gtk::Box::builder()
            .vexpand(false)
            .hexpand(false)
            .valign(gtk::Align::Center)
            .css_classes(["footer-back-icon"])
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
        back_icon_button.add_controller(back_click_gesture);
        back_icon_button.append(&back_icon);
        footer_expand_box.append(&back_icon_button);

        let add_icon_button = gtk::Box::builder()
            .vexpand(false)
            .hexpand(true)
            .halign(gtk::Align::End)
            .valign(gtk::Align::End)
            .css_classes(["footer-back-icon"])
            .build();

        let add_icon = get_image_from_path(modules.submit.icon.default.clone(), &["back-icon"]);
        add_icon.set_vexpand(true);
        add_icon.set_hexpand(true);
        add_icon.set_halign(gtk::Align::Center);
        add_icon.set_valign(gtk::Align::Center);

        let add_click_gesture = GestureClick::builder().button(0).build();
        add_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
            // sender.input_sender().send(Message::BackPressed);

        }));

        // add_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
        //         info!("gesture button released is {}", this.current_button());
        //         let _ = sender.output_sender().send(Message::PairBluetoothPressed);

        // }));

        add_icon_button.append(&add_icon);
        add_icon_button.add_controller(add_click_gesture);

        footer_expand_box.append(&add_icon_button);
        footer.append(&footer_expand_box);
        root.append(&footer);

        let model = BluetoothPairRequestPage { settings: init };

        let widgets = BluetoothPairRequestPageWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {},
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            },
            Message::HomeIconPressed => {
                sender.output(Message::HomeIconPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
