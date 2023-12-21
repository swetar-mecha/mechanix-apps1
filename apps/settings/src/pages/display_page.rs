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
pub struct DisplayPage {
    settings: Settings,
}

//Widgets
pub struct DisplayPageWidgets {}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    ScreenTimeoutOpted,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for DisplayPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = DisplayPageWidgets;

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
            .label("Display")
            .css_classes(["header-title"])
            .build();

        let header_icon = get_image_from_path(
            modules.pages_settings.display.display_icon.clone(),
            &["header-icon"],
        );

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_icon);
        header.append(&header_title);

        let brigntness_label = gtk::Label::builder()
            .label("Brigtness")
            .halign(gtk::Align::Start)
            .build();

        let brigtness_scale = gtk::Scale::builder()
            .draw_value(true)
            .adjustment(
                &gtk::Adjustment::builder()
                    .lower(0.0)
                    .upper(100.0)
                    .value(50.0)
                    .step_increment(10.0)
                    .page_increment(10.0)
                    .build(),
            )
            .orientation(gtk::Orientation::Horizontal)
            .value_pos(gtk::PositionType::Right)
            .build();

        let brigtness_items = gtk::Box::builder()
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
                println!("DISPLAY PAGE - SCREEN clicked {:?}", msg);
                match msg { 
                    CustomListItemMessage::WidgetClicked => Message::ScreenTimeoutOpted,
                }
            });

        let screen_off_timeout_widget = screen_off_timeout.widget();
        brigtness_items.append(&brigtness_scale);
        brigtness_items.append(screen_off_timeout_widget);
        // brigtness_items.append(&screen_off_timeout_widget.clone());

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&brigntness_label);
        scrollable_content.append(&brigtness_items);

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

        let model = DisplayPage { settings: init };

        let widgets = DisplayPageWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("dispay msg - Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {}
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::ScreenTimeoutOpted => {
                let _ = sender.output(Message::ScreenTimeoutOpted);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
