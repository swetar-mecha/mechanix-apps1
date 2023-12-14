use custom_utils::get_image_from_path;
use gtk::{glib::clone, prelude::*};
use relm4::{
    gtk::{self, GestureClick},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent, Controller,
};

use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
};

use custom_widgets::{
    icon_input::{
        IconInput, IconInputCss,
        InitSettings as IconInputSettings,
        OutputMessage as IconInputOutputMessage,
    },
    icon_input_password::{
        IconInputPassword, IconInputPasswordCss, InitSettings as IconInputPasswordSettings,
        OutputMessage as IconInputPasswordOutputMessage,
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
pub struct AddNetworkPage {
    settings: Settings,
}

//Widgets
pub struct AddNetworkPageWidgets {
    network_name_input: Controller<IconInput>,
    password_input: Controller<IconInputPassword>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    BackPressed,
    HomeIconPressed,
    PasswordChange(String),
    NetworkNameChange(String),
}

impl SimpleComponent for AddNetworkPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = AddNetworkPageWidgets;

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

        let new_network_label = gtk::Label::builder()
            .label("New Network")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&new_network_label);

        let network_name_label = gtk::Label::builder()
            .label("Network Name")
            .halign(gtk::Align::Start)
            .css_classes(["add-network-name-label"])
            .build();

        let network_name_input = IconInput::builder()
            .launch(IconInputSettings {
                clear_icon: None,
                icon: None,
                placeholder: Option::from("Network Name".to_string()),
                css: IconInputCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconInputOutputMessage::InputChange(text) => Message::NetworkNameChange(text),
            });

        let password_label = gtk::Label::builder()
            .label("Password")
            .halign(gtk::Align::Start)
            .css_classes(["add-network-password-label"])
            .build();

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
        root.append(&network_name_label);
        root.append(network_name_input.widget());
        root.append(&password_label);
        root.append(password_input.widget());

        let footer = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes(["footer"])
        .hexpand(true)
        .vexpand(true)
        .valign(gtk::Align::End)
        .build();

        let footer_expand_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .build();


        let back_icon_button = gtk::Box::builder()
            .hexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(["footer-icon-button"])
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
            .css_classes(["footer-icon-button"])
            .build();

        let add_icon = get_image_from_path(widget_configs.footer.add_icon, &["back-icon"]);
        add_icon.set_vexpand(true);
        add_icon.set_hexpand(true);
        add_icon.set_halign(gtk::Align::Center);
        add_icon.set_valign(gtk::Align::Center);

        let add_click_gesture = GestureClick::builder().button(0).build();
        add_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
            // sender.input_sender().send(Message::BackPressed);

        }));

        add_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button released is {}", this.current_button());
                // let _ = sender.output_sender().send(Message::AddNetworkPressed);

        }));

        add_icon_button.append(&add_icon);
        add_icon_button.add_controller(add_click_gesture);

        footer_expand_box.append(&add_icon_button);
        footer.append(&footer_expand_box);
        root.append(&footer);


        let model = AddNetworkPage { settings: init };

        let widgets = AddNetworkPageWidgets {
            password_input,
            network_name_input,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            },
            Message::HomeIconPressed => {
                sender.output(Message::HomeIconPressed);
            }
            Message::PasswordChange(text) => {}
            Message::NetworkNameChange(text) => {}
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
