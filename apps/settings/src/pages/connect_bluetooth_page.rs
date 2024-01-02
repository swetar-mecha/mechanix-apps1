use custom_utils::get_image_from_path;
use gtk::{glib::clone, prelude::*};
use relm4::{
    gtk::{self, GestureClick},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent, Controller,
};
use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
};
use custom_widgets::icon_input::{
    IconInput, IconInputCss, InitSettings as IconInputSettings, OutputMessage as IconInputOutputMessage,
};

use tracing::info;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct ConnectBluetoothPage {
    settings: Settings,
}

//Widgets
pub struct ConnectBluetoothPageWidgets {
    code_input: Controller<IconInput>,
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

impl SimpleComponent for ConnectBluetoothPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = ConnectBluetoothPageWidgets;

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
            .label("Pair with 'Macbook Pro'")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&enter_password_label);

        let code_input_label = gtk::Label::builder()
            .label("Enter code shared by the device here")
            .halign(gtk::Align::Start)
            .css_classes(["text-14-label"])
            .build();

        let code_input = IconInput::builder()
            .launch(IconInputSettings {
                clear_icon: None,
                icon: None,
                placeholder: Option::from("".to_string()),
                css: IconInputCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconInputOutputMessage::InputChange(text) => Message::PasswordChange(text),
            });

        root.append(&header);
        root.append(&code_input_label);
        root.append(code_input.widget());

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
            .hexpand(false)
            .valign(gtk::Align::Center)
            .css_classes(["footer-back-icon"])
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
        footer_expand_box.append(&back_icon_button);

        let add_icon_button = gtk::Box::builder()
            .vexpand(false)
            .hexpand(true)
            .halign(gtk::Align::End)
            .valign(gtk::Align::End)
            .css_classes(["footer-back-icon"])
            .build();

        let add_icon = get_image_from_path(modules.submit.icon.default.clone(), &["back-icon"]);
        add_icon.set_vexpand(true);
        add_icon.set_hexpand(true);
        add_icon.set_halign(gtk::Align::Center);
        add_icon.set_valign(gtk::Align::Center);

        let add_click_gesture = GestureClick::builder().button(0).build();
        add_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
            // sender.input_sender().send(Message::BackPressed);

        }));

        // add_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
        //         info!("gesture button released is {}", this.current_button());
        //         let _ = sender.output_sender().send(Message::PairBluetoothPressed);

        // }));

        add_icon_button.append(&add_icon);
        add_icon_button.add_controller(add_click_gesture);

        footer_expand_box.append(&add_icon_button);
        footer.append(&footer_expand_box);
        root.append(&footer);

        let model = ConnectBluetoothPage { settings: init };

        let widgets = ConnectBluetoothPageWidgets { code_input };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {},
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            },
            Message::HomeIconPressed => {
                sender.output(Message::HomeIconPressed);
            },
            Message::PasswordChange(text) => {}
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
