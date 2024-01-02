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
pub struct BatteryPage {
    settings: Settings,
}

//Widgets
pub struct BatteryPageWidgets {}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    ScreenTimeoutOpted,
    PerformanceOpted
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for BatteryPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = BatteryPageWidgets;

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
            .label("Battery")
            .css_classes(["header-title"])
            .build();

        let header_icon = get_image_from_path(
            modules.pages_settings.battery.display_icon.clone(),
            &["header-icon"],
        );

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_icon);
        header.append(&header_title);

        let battery_label = gtk::Label::builder()
            .label("Battery Percentage")
            .halign(gtk::Align::Start)
            .build();

        let battery_percentage_level = gtk::LevelBar::builder()
        .min_value(0.0)
        .max_value(100.0)
        .value(70.0)
        .orientation(gtk::Orientation::Horizontal) 
        .css_classes(["custom-levelbar"])
        .build();

        let battery_items = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let screen_off_timeout = CustomListItem::builder()
            .launch(CustomListItemSettings {
                start_icon: None,
                text: "Screen off timeout".to_string(),
                value: "30s".to_owned(),
                end_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                println!("BATTERY PAGE - SCREEN clicked {:?}", msg);
                match msg { 
                    CustomListItemMessage::WidgetClicked => Message::ScreenTimeoutOpted,
                }
            });

        let screen_off_timeout_widget = screen_off_timeout.widget();

        let battery_performance_mode = CustomListItem::builder()
        .launch(CustomListItemSettings {
            start_icon: None,
            text: "Performance Mode".to_string(),
            value: "Balenced".to_owned(),
            end_icon: widget_configs.menu_item.end_icon.clone(),
        })
        .forward(sender.input_sender(), |msg| {
            info!("msg is {:?}", msg);
            match msg {
                CustomListItemMessage::WidgetClicked => Message::PerformanceOpted,
            }
        });
        let battery_performance_mode_widget = battery_performance_mode.widget();


        battery_items.append(&battery_percentage_level);
        battery_items.append(screen_off_timeout_widget);
        battery_items.append(battery_performance_mode_widget);
        // battery_items.append(&screen_off_timeout_widget.clone());

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&battery_label);
        scrollable_content.append(&battery_items);

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

        let model = BatteryPage { settings: init };

        let widgets = BatteryPageWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("battery page - msg - Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {}
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::ScreenTimeoutOpted => {
                let _ = sender.output(Message::ScreenTimeoutOpted);
            }
            Message::PerformanceOpted => {
                let _ = sender.output(Message::PerformanceOpted);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
