use custom_utils::get_image_from_path;
use gtk::{glib::clone, prelude::*};
use relm4::{
    gtk::{self, GestureClick},
    Component, ComponentParts, ComponentSender, SimpleComponent,
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
pub struct ProtocolDetailsPage {
    settings: Settings,
}

//Widgets
pub struct ProtocolDetailsPageWidgets {}

//Messages
#[derive(Debug)]
pub enum Message { 
    BackPressed,
    HomeIconPressed,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for ProtocolDetailsPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = ProtocolDetailsPageWidgets;

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
            .label("IP Settings")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_title);

        let details_list1 = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["settings-item-details-box"])
            .build();

        
        let details_list2 = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["settings-item-details-box"])
            .build();


        let details_row_1 = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["settings-item-details-box-row"])
            .build();

        let mode_label = gtk::Label::builder()
        .label("Mode")
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        let mode_value = gtk::Label::builder()
        .label("Static")
        .hexpand(true)
        .halign(gtk::Align::End)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        details_row_1.append(&mode_label);
        details_row_1.append(&mode_value);
        details_list1.append(&details_row_1);

        let details_row_2 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["settings-item-details-box-row"])
        .build();

        let ip_address_label = gtk::Label::builder()
        .label("IP Address")
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        let ip_address_value = gtk::Label::builder()
        .label("192.160.12.1")
        .hexpand(true)
        .halign(gtk::Align::End)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        details_row_2.append(&ip_address_label);
        details_row_2.append(&ip_address_value);
        details_list2.append(&details_row_2);


        let details_row_3 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["settings-item-details-box-row"])
        .build();

        let subnet_mask_label = gtk::Label::builder()
        .label("Subnet Mask")
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        let subnet_mask_value = gtk::Label::builder()
        .label("255.255.255.0")
        .hexpand(true)
        .halign(gtk::Align::End)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        details_row_3.append(&subnet_mask_label);
        details_row_3.append(&subnet_mask_value);
        details_list2.append(&details_row_3);


        let details_row_4 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["settings-item-details-box-row"])
        .build();

        let gateway_label = gtk::Label::builder()
        .label("Gateway")
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        let gateway_value = gtk::Label::builder()
        .label("None")
        .hexpand(true)
        .halign(gtk::Align::End)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

        details_row_4.append(&gateway_label);
        details_row_4.append(&gateway_value);
        details_list2.append(&details_row_4);

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        scrollable_content.append(&details_list1);
        scrollable_content.append(&details_list2);

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
            .build();

        let back_icon_button = gtk::Box::builder()
            .vexpand(false)
            .hexpand(false)
            .valign(gtk::Align::End)
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
        }));

        back_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button released is {}", this.current_button());
                let _ = sender.output(Message::BackPressed);
        }));

        back_icon_button.append(&back_icon);
        back_icon_button.add_controller(back_click_gesture);

        footer.append(&back_icon_button);

        root.append(&footer);

        let model = ProtocolDetailsPage { settings: init };

        let widgets = ProtocolDetailsPageWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Protocol Details- Update message is {:?}", message);
        match message {
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            },
            Message::HomeIconPressed => {
                let _ = sender.output(Message::HomeIconPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
