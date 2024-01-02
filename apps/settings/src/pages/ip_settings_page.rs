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
pub struct IPSettingsPage {
    settings: Settings,
}

//Widgets
pub struct IPSettingsPageWidgets {}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    ProtocolModes,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for IPSettingsPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = IPSettingsPageWidgets;

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

        // let header_box = gtk::Box::builder()
        // .orientation(gtk::Orientation::Horizontal)
        // .css_classes(["header"])
        // .build();

        // let label = gtk::Label::builder()
        //     .label("IP Settings")
        //     .css_classes(["header-title"])
        //     .build();

        // header_box.append(&label);

        // 
        let header_title = gtk::Label::builder()
        .label("IP Settings")
        .css_classes(["header-title"])
        .build();


        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_title);

      
        let ip_items = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let protocol_mode = CustomListItem::builder()
            .launch(CustomListItemSettings {
                start_icon: None,
                text: "Mode".to_string(),
                value: "Auto [ DHCP ]".to_owned(),
                end_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("IP SETTINGS PAGE msg is {:?}", msg);
                match msg { 
                    CustomListItemMessage::WidgetClicked => Message::ProtocolModes,
                }
            });

        let protocol_mode_widget = protocol_mode.widget();


        ip_items.append(protocol_mode_widget);

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&ip_items);

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
        .build();

        let back_icon_button = gtk::Box::builder()
            .vexpand(false)
            .hexpand(false)
            .valign(gtk::Align::End)
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
        }));

        back_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button released is {}", this.current_button());
                let _ = sender.output(Message::BackPressed);
        }));

        back_icon_button.append(&back_icon);
        back_icon_button.add_controller(back_click_gesture);

        footer.append(&back_icon_button);

        root.append(&footer);

        let model = IPSettingsPage { settings: init };

        let widgets = IPSettingsPageWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("IP settings page - msg - Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {}
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::ProtocolModes => {
                let _ = sender.output(Message::ProtocolModes);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
