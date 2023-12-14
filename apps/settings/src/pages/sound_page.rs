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
pub struct SoundPage {
    settings: Settings,
}

//Widgets
pub struct SoundPageWidgets {}

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

impl SimpleComponent for SoundPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = SoundPageWidgets;

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
            .label("Sound")
            .css_classes(["header-title"])
            .build();

        let header_icon = get_image_from_path(
            modules.pages_settings.sound.display_icon.clone(),
            &["header-icon"],
        );

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_icon);
        header.append(&header_title);

        let output_volume_label = gtk::Label::builder()
            .label("Output Volume")
            .halign(gtk::Align::Start)
            .build();

        let volume_scale = gtk::Scale::builder()
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

        let output_volumes_items = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        output_volumes_items.append(&volume_scale);

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&output_volume_label);
        scrollable_content.append(&output_volumes_items);

        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .min_content_height(360)
            .css_classes(["scrollable"])
            .child(&scrollable_content)
            .build();
        root.append(&scrolled_window);

        let model = SoundPage { settings: init };

        let widgets = SoundPageWidgets {};

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
