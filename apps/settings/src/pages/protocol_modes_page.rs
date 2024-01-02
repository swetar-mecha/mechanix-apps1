use custom_utils::get_image_from_path;
use gtk::{glib::clone, prelude::*};
use relm4::{
    gtk::{self, GestureClick},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent,
};

use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::custom_list_radio_button::{
            CustomListRadioButton, CustomListRadioButtonSettings,
            Message as CustomListRadioButtonMessage,
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
pub struct ProtocolModesPage {
    settings: Settings,
}

//Widgets
pub struct ProtocolModesPageWidgets {}

//Messages
#[derive(Debug)]
pub enum Message {
    AutoModePressed,
    StaticModePressed,
    BackPressed,
    HomeIconPressed,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for ProtocolModesPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = ProtocolModesPageWidgets;

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
            .label("Mode")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_title);

        let screen_off_timeout_items = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let auto_mode = CustomListRadioButton::builder()
            .launch(CustomListRadioButtonSettings {
                text: "Automatic [ DHCP ]".to_string(),
                active_icon: widget_configs.radio_item.active_icon.clone(),
                inactive_icon: widget_configs.radio_item.inactive_icon.clone(),
                is_active: true,
                ..Default::default()
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomListRadioButtonMessage::WidgetClicked => Message::AutoModePressed,
                }
            });

    
        let static_mode = CustomListRadioButton::builder()
            .launch(CustomListRadioButtonSettings {
                text: "Static".to_string(),
                active_icon: widget_configs.radio_item.active_icon.clone(),
                inactive_icon: widget_configs.radio_item.inactive_icon.clone(),
                is_active: false,
                description_text: Some("<span foreground='red'>*</span> Specifying IP Address, Subnet and Gateway is mandatory".to_string())
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomListRadioButtonMessage::WidgetClicked => Message::StaticModePressed,
                }
            });

        let auto_mode_widget = auto_mode.widget();
        let static_mode_widget = static_mode.widget();

        screen_off_timeout_items.append(auto_mode_widget);
        screen_off_timeout_items.append(static_mode_widget);

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&screen_off_timeout_items);

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
            .valign(gtk::Align::Center)
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
        back_icon_button.add_controller(back_click_gesture);
        back_icon_button.append(&back_icon);
        footer.append(&back_icon_button);

        root.append(&footer);

        let model = ProtocolModesPage { settings: init };

        let widgets = ProtocolModesPageWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            },
            Message::StaticModePressed => {
                let _ = sender.output(Message::StaticModePressed);
            },
            Message::AutoModePressed => {
                let _ = sender.output(Message::AutoModePressed);
            },
            Message::HomeIconPressed => {
                let _ = sender.output(Message::HomeIconPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}