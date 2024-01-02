use custom_utils::get_image_from_path;
use gtk::{glib::clone, prelude::*};
use relm4::{
    gtk::{self, GestureClick},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent,
};

use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::custom_network_item::{
        CustomNetworkItem, CustomNetworkItemSettings, Message as CustomNetworkItemMessage,
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
pub struct ManageNetworksPage {
    settings: Settings,
}

//Widgets
pub struct ManageNetworksPageWidgets {}

//Messages
#[derive(Debug)]
pub enum Message {
    BackPressed,
    KnownNetworkPressed,
    AvailableNetworkPressed,
    AddNetworkPressed,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for ManageNetworksPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = ManageNetworksPageWidgets;

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
            .label("Manage Networks")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_title);

        let known_networks_label = gtk::Label::builder()
            .label("Known Networks")
            .css_classes(["list-label"])
            .halign(gtk::Align::Start)
            .build();

        let known_networks_list = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let known_network_1 = CustomNetworkItem::builder()
            .launch(CustomNetworkItemSettings {
                name: "Actonate 5g".to_string(),
                is_connected: true,
                is_private: true,
                strength: 80,
                connected_icon: widget_configs.network_item.connected_icon.clone(),
                private_icon: widget_configs.network_item.private_icon.clone(),
                strength_icon: widget_configs.network_item.wifi_100_icon.clone(),
                info_icon: widget_configs.network_item.info_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("Actonate 5g - info click- msg is {:?}", msg);
                match msg {
                    CustomNetworkItemMessage::WidgetClicked => Message::KnownNetworkPressed,
                }
            });

        let known_network_2 = CustomNetworkItem::builder()
            .launch(CustomNetworkItemSettings {
                name: "Actonate 2g".to_string(),
                is_connected: false,
                is_private: true,
                strength: 80,
                connected_icon: widget_configs.network_item.connected_icon.clone(),
                private_icon: widget_configs.network_item.private_icon.clone(),
                strength_icon: widget_configs.network_item.wifi_100_icon.clone(),
                info_icon: widget_configs.network_item.info_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomNetworkItemMessage::WidgetClicked => Message::KnownNetworkPressed,
                }
            });

        known_networks_list.append(known_network_1.widget());
        known_networks_list.append(known_network_2.widget());

        let available_networks_label = gtk::Label::builder()
            .label("Available Networks")
            .css_classes(["list-label"])
            .halign(gtk::Align::Start)
            .build();

        let available_networks_list = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let available_network_1 = CustomNetworkItem::builder()
            .launch(CustomNetworkItemSettings {
                name: "Mecha 5g".to_string(),
                is_connected: false,
                is_private: true,
                strength: 80,
                connected_icon: widget_configs.network_item.connected_icon.clone(),
                private_icon: widget_configs.network_item.private_icon.clone(),
                strength_icon: widget_configs.network_item.wifi_100_icon.clone(),
                info_icon: widget_configs.network_item.info_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomNetworkItemMessage::WidgetClicked => Message::AvailableNetworkPressed,
                }
            });

        let available_network_2 = CustomNetworkItem::builder()
            .launch(CustomNetworkItemSettings {
                name: "Mecha 5g".to_string(),
                is_connected: false,
                is_private: false,
                strength: 80,
                connected_icon: widget_configs.network_item.connected_icon.clone(),
                private_icon: widget_configs.network_item.private_icon.clone(),
                strength_icon: widget_configs.network_item.wifi_100_icon.clone(),
                info_icon: widget_configs.network_item.info_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomNetworkItemMessage::WidgetClicked => Message::AvailableNetworkPressed,
                }
            });

        available_networks_list.append(available_network_1.widget());
        available_networks_list.append(available_network_2.widget());

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&known_networks_label);
        scrollable_content.append(&known_networks_list);
        scrollable_content.append(&available_networks_label);
        scrollable_content.append(&available_networks_list);

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
            .build();

        let footer_expand_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .valign(gtk::Align::End)
            .build();

        let back_icon_button = gtk::Box::builder()
            .vexpand(false)
            .halign(gtk::Align::Start)
            .css_classes(["footer-back-icon"])
            .build();

        let back_icon = get_image_from_path(widget_configs.footer.back_icon, &[]);
        back_icon.set_vexpand(true);
        back_icon.set_hexpand(true);
        back_icon.set_halign(gtk::Align::Center);
        back_icon.set_valign(gtk::Align::Center);
        let left_click_gesture = GestureClick::builder().button(0).build();
        left_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
            // sender.input_sender().send(Message::BackSpacePressed);

        }));

        left_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button released is {}", this.current_button());
                let _ = sender.output_sender().send(Message::BackPressed);

        }));

        back_icon_button.append(&back_icon);
        back_icon_button.add_controller(left_click_gesture);
        footer_expand_box.append(&back_icon_button);

        let add_icon_button = gtk::Box::builder()
            .vexpand(false)
            .hexpand(true)
            .halign(gtk::Align::End)
            .valign(gtk::Align::End)
            .css_classes(["footer-back-icon"])
            .build();

        let add_icon = get_image_from_path(widget_configs.footer.add_icon, &["back-icon"]);
        add_icon.set_vexpand(true);
        add_icon.set_hexpand(true);
        add_icon.set_halign(gtk::Align::Center);
        add_icon.set_valign(gtk::Align::Center);

        let add_click_gesture = GestureClick::builder().button(0).build();
        add_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
            // sender.input_sender().send(Message::BackSpacePressed);

        }));

        add_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button released is {}", this.current_button());
                let _ = sender.input_sender().send(Message::AddNetworkPressed);
        }));

        add_icon_button.append(&add_icon);
        add_icon_button.add_controller(add_click_gesture);

        footer_expand_box.append(&add_icon_button);
        footer.append(&footer_expand_box);

        root.append(&footer);

        let model = ManageNetworksPage { settings: init };

        let widgets = ManageNetworksPageWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::KnownNetworkPressed => {
                let _ = sender.output(Message::KnownNetworkPressed);
            }
            Message::AvailableNetworkPressed => {
                let _ = sender.output(Message::AvailableNetworkPressed);
            }
            Message::AddNetworkPressed => {
                let _ = sender.output(Message::AddNetworkPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
