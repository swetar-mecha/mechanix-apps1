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
pub struct DateTimePage {
    settings: Settings,
}

//Widgets
pub struct DateTimePageWidgets {}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    SetTimeOpted,
    SetDateOpted,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for DateTimePage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = DateTimePageWidgets;

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
            .label("Date & Time")
            .css_classes(["header-title"])
            .build();

        // modules.pages_settings.sound.display_icon.clone(),
        let header_icon: gtk::Image = get_image_from_path(
            modules.pages_settings.dateandtime.display_icon.clone(),
            &["header-icon"],
        );

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_icon);
        header.append(&header_title);

        let auto_time_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["settings-item-details-box"])
            .build();

        let enable_auto_time_row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["settings-item-details-box-row"])
            .build();

        let enable_auto_time_text = gtk::Label::builder()
            .label("Enable Automatic Time")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(["custom-switch-text"])
            .build();

        let switch = gtk::Switch::new();
        switch.set_active(true);
        let style_context = switch.style_context();
        style_context.add_class("custom-switch");

        enable_auto_time_row.append(&enable_auto_time_text);
        enable_auto_time_row.append(&switch);
        auto_time_box.append(&enable_auto_time_row);

        let screen_items = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

        let set_time = CustomListItem::builder()
            .launch(CustomListItemSettings {
                start_icon: None,
                text: "Set time".to_string(),
                value: "GST".to_owned(),
                end_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg { 
                    CustomListItemMessage::WidgetClicked => Message::SetTimeOpted,
                }
            });

        let set_time_widget = set_time.widget();
        screen_items.append(set_time_widget);


        let set_date = CustomListItem::builder()
        .launch(CustomListItemSettings {
            start_icon: None,
            text: "Set date".to_string(),
            value: "January 1, 2024".to_owned(),  
            end_icon: widget_configs.menu_item.end_icon.clone(),
        })
        .forward(sender.input_sender(), |msg| {
            info!("msg is {:?}", msg);
            match msg { 
                CustomListItemMessage::WidgetClicked => Message::SetDateOpted,
            }
        });

        let set_date_widget = set_date.widget();

        screen_items.append(set_date_widget);

     

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
        scrollable_content.append(&auto_time_box);
        scrollable_content.append(&screen_items);

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

        let model = DateTimePage { settings: init };

        let widgets = DateTimePageWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {}
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::SetTimeOpted => {
                let _ = sender.output(Message::SetTimeOpted);
            }
            Message::SetDateOpted => {
                let _ = sender.output(Message::SetDateOpted);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
