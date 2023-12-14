use custom_utils::get_image_from_path;
use gtk::prelude::BoxExt;
use relm4::{
    gtk, ComponentParts, ComponentSender, SimpleComponent,
};

use crate::settings::{LayoutSettings, Modules, WidgetConfigs};

use tracing::info;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct SecurityPage {
    settings: Settings,
}

//Widgets
pub struct SecurityPageWidgets {}

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

impl SimpleComponent for SecurityPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = SecurityPageWidgets;

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
            .label("Security")
            .css_classes(["header-title"])
            .build();

        // modules.pages_settings.sound.display_icon.clone(),
        let header_icon: gtk::Image = get_image_from_path(
            modules.pages_settings.security.display_icon.clone(),
            &["header-icon"],
        );

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_icon);
        header.append(&header_title);

        let lock_status_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["security-label-box"])
            .build();

        let lock_status_text = gtk::Label::builder()
            .label("Enable lock")
            .css_classes(["security-label-text"])
            .build();

        lock_status_box.append(&lock_status_text);

        let lock_timeout_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["security-label-box"])
            .build();

        let lock_timeout_text = gtk::Label::builder()
            .label("Lock timeout")
            .css_classes(["security-label-text"])
            .build();

        lock_timeout_box.append(&lock_timeout_text);

        let reset_pin_button = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["reset-pin-btn-box"])
        .build();

        let reset_pin_text = gtk::Label::builder()
            .label("Reset Pin")
            .css_classes(["reset-pin-btn-text"])
            .halign(gtk::Align::Center)
            .build();
        reset_pin_button.append(&reset_pin_text);

        // let scrollable_content = gtk::Box::builder()
        //     .orientation(gtk::Orientation::Vertical)
        //     .build();
        // scrollable_content.append(&output_volume_label);
        // scrollable_content.append(&output_volumes_items);

        // let scrolled_window = gtk::ScrolledWindow::builder()
        //     .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        //     .min_content_width(360)
        //     .min_content_height(360)
        //     .css_classes(["scrollable"])
        //     .child(&scrollable_content)
        //     .build();
        // root.append(&scrolled_window);

        root.append(&header);
        root.append(&lock_status_box);
        root.append(&lock_timeout_box);
        root.append(&reset_pin_button);

        let model = SecurityPage { settings: init };

        let widgets = SecurityPageWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {}
            Message::BackSpacePressed => {}
            Message::HomeIconPressed => {}
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
