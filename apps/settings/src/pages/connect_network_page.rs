use custom_utils::get_image_from_path;
use gtk::{glib::clone, prelude::*};
use relm4::{
    gtk::{self, GestureClick},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent, Controller,
};

use crate::settings::{LayoutSettings, Modules, WidgetConfigs};
use custom_widgets::icon_input_password::{
    IconInputPassword, IconInputPasswordCss, InitSettings as IconInputPasswordSettings,
    InputMessage as IconInputPasswordInputMessage, OutputMessage as IconInputPasswordOutputMessage,
};

use tracing::info;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct ConnectNetworkPage {
    settings: Settings,
}

//Widgets
pub struct ConnectNetworkPageWidgets {
    password_input: Controller<IconInputPassword>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    HomeIconPressed,
    PasswordChange(String),
}

pub struct SettingItem {
    name: String,
}

impl SimpleComponent for ConnectNetworkPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = ConnectNetworkPageWidgets;

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

        let enter_password_label = gtk::Label::builder()
            .label("Enter password for 'Mecha 5g'")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&enter_password_label);

        let password_input = IconInputPassword::builder()
            .launch(IconInputPasswordSettings {
                icon: modules.peek_password.icon.default.to_owned(),
                toggle_icon: None,
                placeholder: Option::from("Password".to_string()),
                css: IconInputPasswordCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconInputPasswordOutputMessage::InputChange(text) => Message::PasswordChange(text),
            });

        root.append(&header);
        root.append(password_input.widget());

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
        let left_click_gesture = GestureClick::builder().button(0).build();
        left_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
        }));

        left_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button released is {}", this.current_button());
                let _ = sender.output(Message::BackPressed);
        }));

        back_icon_button.append(&back_icon); 
        back_icon_button.add_controller(left_click_gesture);

        footer.append(&back_icon_button);

        root.append(&footer);


        let model = ConnectNetworkPage { settings: init };

        let widgets = ConnectNetworkPageWidgets { password_input };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {}
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            },
            Message::HomeIconPressed => {
                sender.output(Message::HomeIconPressed);
            }
            Message::PasswordChange(text) => {}
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
