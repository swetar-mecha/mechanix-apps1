use custom_utils::get_image_from_path;
use gtk::{glib::clone, prelude::BoxExt};
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent,
};

use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::{
        custom_list_radio_button::{
            CustomListRadioButton, CustomListRadioButtonSettings,
            Message as CustomListRadioButtonMessage,
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
pub struct PerformanceModePage {
    settings: Settings,
}

//Widgets
pub struct PerformanceModePageWidgets {}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackSpacePressed,
    HomeIconPressed,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for PerformanceModePage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = PerformanceModePageWidgets;

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
            .label("Performance Mode")
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

        let low = CustomListRadioButton::builder()
            .launch(CustomListRadioButtonSettings {
                text: "Low".to_string(),
                active_icon: widget_configs.radio_item.active_icon.clone(),
                inactive_icon: widget_configs.radio_item.inactive_icon.clone(),
                is_active: false,
                ..Default::default()
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomListRadioButtonMessage::WidgetClicked => Message::HomeIconPressed,
                }
            });

        let medium = CustomListRadioButton::builder()
            .launch(CustomListRadioButtonSettings {
                text: "Medium".to_string(),
                active_icon: widget_configs.radio_item.active_icon.clone(),
                inactive_icon: widget_configs.radio_item.inactive_icon.clone(),
                is_active: true,
                ..Default::default()
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomListRadioButtonMessage::WidgetClicked => Message::HomeIconPressed,
                }
            });
        let high = CustomListRadioButton::builder()
            .launch(CustomListRadioButtonSettings {
                text: "High".to_string(),
                active_icon: widget_configs.radio_item.active_icon.clone(),
                inactive_icon: widget_configs.radio_item.inactive_icon.clone(),
                is_active: false,
                description_text: Some("** Higher performance will use battery faster and \nincrease the temperature of the device significantly. \nCheck ambient temperature before proceeding.".to_string())
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomListRadioButtonMessage::WidgetClicked => Message::HomeIconPressed,
                }
            });

        let low_widget = low.widget();
        let medium_widget = medium.widget();
        let high_widget = high.widget();

        screen_off_timeout_items.append(low_widget);
        screen_off_timeout_items.append(medium_widget);
        screen_off_timeout_items.append(high_widget);

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

        let model = PerformanceModePage { settings: init };

        let widgets = PerformanceModePageWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {}
            Message::BackSpacePressed => {}
            Message::HomeIconPressed => {
                sender.output(Message::HomeIconPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
