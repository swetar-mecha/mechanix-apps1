use gtk::prelude::BoxExt;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent,
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
pub struct LockTimeoutPage {
    settings: Settings,
}

//Widgets
pub struct LockTimeoutPageWidgets {}

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

impl SimpleComponent for LockTimeoutPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = LockTimeoutPageWidgets;

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


        let footer = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["footer"])
            .hexpand(true)
            .vexpand(true)
            .build();

        let header_title = gtk::Label::builder()
            .label("Lock Timeout")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_title);

        let lock_timeout_items = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let timeout_10_s = CustomListRadioButton::builder()
            .launch(CustomListRadioButtonSettings {
                text: "10s".to_string(),
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

        let timeout_30_s = CustomListRadioButton::builder()
            .launch(CustomListRadioButtonSettings {
                text: "30s".to_string(),
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
        let timeout_60_s = CustomListRadioButton::builder()
            .launch(CustomListRadioButtonSettings {
                text: "60s".to_string(),
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

        let timeout_5_m = CustomListRadioButton::builder()
            .launch(CustomListRadioButtonSettings {
                text: "5m".to_string(),
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
        let timeout_15_m = CustomListRadioButton::builder()
            .launch(CustomListRadioButtonSettings {
                text: "15m".to_string(),
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

        let timeout_30_m = CustomListRadioButton::builder()
            .launch(CustomListRadioButtonSettings {
                text: "30m".to_string(),
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

        let timeout_10_s_widget = timeout_10_s.widget();
        let timeout_30_s_widget = timeout_30_s.widget();
        let timeout_60_s_widget = timeout_60_s.widget();
        let timeout_5_m_widget = timeout_5_m.widget();
        let timeout_15_m_widget = timeout_15_m.widget();
        let timeout_30_m_widget = timeout_30_m.widget();
        lock_timeout_items.append(timeout_10_s_widget);
        lock_timeout_items.append(timeout_30_s_widget);
        lock_timeout_items.append(timeout_60_s_widget);
        lock_timeout_items.append(timeout_5_m_widget);
        lock_timeout_items.append(timeout_15_m_widget);
        lock_timeout_items.append(timeout_30_m_widget);

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&lock_timeout_items);

        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .min_content_height(360)
            .css_classes(["scrollable"])
            .child(&scrollable_content)
            .build();
        root.append(&scrolled_window);

        // footer-buttons
        let cancel_button = gtk::Button::builder()
            .label("Cancel")
            .hexpand(true)
            .vexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(["footer-btn-box", "cancel-btn-txt"])
            .build();

        footer.append(&cancel_button);

        let submit_button = gtk::Button::builder()
            .label("Done")
            .hexpand(true)
            .vexpand(true)
            .halign(gtk::Align::End)
            .css_classes(["footer-btn-box", "save-btn-txt"])
            .build();

        footer.append(&submit_button);
        root.append(&footer);

        let model = LockTimeoutPage { settings: init };

        let widgets = LockTimeoutPageWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {}
            Message::BackSpacePressed => {}
            Message::HomeIconPressed => {
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
