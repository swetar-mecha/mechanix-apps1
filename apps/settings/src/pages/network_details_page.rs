use custom_utils::get_image_from_path;
use gtk::{glib::clone, prelude::*};
use relm4::{
    gtk::{self, GestureClick},
    ComponentParts, ComponentSender, SimpleComponent,
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
pub struct NetworkDetailsPage {
    settings: Settings,
}

//Widgets
pub struct NetworkDetailsPageWidgets {}

//Messages
#[derive(Debug)]
pub enum Message {
    BackPressed,
    HomeIconPressed,
}

pub struct SettingItem {
    name: String,
}

impl SimpleComponent for NetworkDetailsPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = NetworkDetailsPageWidgets;

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

        let network_name = gtk::Label::builder()
            .label("Actonate 5g")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&network_name);

        let network_details_box_1 = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["settings-item-details-box"])
            .build();

        let network_ssid_row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["settings-item-details-box-row"])
            .build();

        let network_ssid_key = gtk::Label::builder()
            .label("Network SSID")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(["settings-item-details-box-row-key"])
            .build();
        let network_ssid_value = gtk::Label::builder()
            .label("Actonate 5g")
            .css_classes(["settings-item-details-box-row-value"])
            .build();

        network_ssid_row.append(&network_ssid_key);
        network_ssid_row.append(&network_ssid_value);

        network_details_box_1.append(&network_ssid_row);

        let network_id_row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["settings-item-details-box-row"])
            .build();

        let network_id_key = gtk::Label::builder()
            .label("Network ID")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(["settings-item-details-box-row-key"])
            .build();
        let network_id_value = gtk::Label::builder()
            .label("2")
            .css_classes(["settings-item-details-box-row-value"])
            .build();

        network_id_row.append(&network_id_key);
        network_id_row.append(&network_id_value);

        network_details_box_1.append(&network_id_row);

        let passphrase_row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["settings-item-details-box-row"])
            .build();

        let passphrase_key = gtk::Label::builder()
            .label("Passphrase")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(["settings-item-details-box-row-key"])
            .build();
        let passphrase_value = gtk::Label::builder()
            .label("WPA2")
            .css_classes(["settings-item-details-box-row-value"])
            .build();

        passphrase_row.append(&passphrase_key);
        passphrase_row.append(&passphrase_value);

        network_details_box_1.append(&passphrase_row);

        let frequency_row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["settings-item-details-box-row"])
            .build();

        let frequency_key = gtk::Label::builder()
            .label("Frequency")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(["settings-item-details-box-row-key"])
            .build();
        let frequency_value = gtk::Label::builder()
            .label("5GHz")
            .css_classes(["settings-item-details-box-row-value"])
            .build();

        frequency_row.append(&frequency_key);
        frequency_row.append(&frequency_value);

        network_details_box_1.append(&frequency_row);

        let network_details_box_2 = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .css_classes(["settings-item-details-box"])
            .build();

        let ip_address_row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["settings-item-details-box-row"])
            .build();

        let ip_address_key = gtk::Label::builder()
            .label("IP Address")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(["settings-item-details-box-row-key"])
            .build();
        let ip_address_value = gtk::Label::builder()
            .label("192.168.203.106")
            .css_classes(["settings-item-details-box-row-value"])
            .build();

        ip_address_row.append(&ip_address_key);
        ip_address_row.append(&ip_address_value);

        network_details_box_2.append(&ip_address_row);

        let subnet_mask_row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["settings-item-details-box-row"])
            .build();

        let subnet_mask_key = gtk::Label::builder()
            .label("Subnet Mask")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(["settings-item-details-box-row-key"])
            .build();
        let subnet_mask_value = gtk::Label::builder()
            .label("255.255.255.0")
            .css_classes(["settings-item-details-box-row-value"])
            .build();

        subnet_mask_row.append(&subnet_mask_key);
        subnet_mask_row.append(&subnet_mask_value);

        network_details_box_2.append(&subnet_mask_row);

        let gateway_row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["settings-item-details-box-row"])
            .build();

        let gateway_key = gtk::Label::builder()
            .label("Gateway")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(["settings-item-details-box-row-key"])
            .build();
        let gateway_value = gtk::Label::builder()
            .label("192.168.0.1")
            .css_classes(["settings-item-details-box-row-value"])
            .build();

        gateway_row.append(&gateway_key);
        gateway_row.append(&gateway_value);

        network_details_box_2.append(&gateway_row);

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&network_details_box_1);
        scrollable_content.append(&network_details_box_2);

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


        let footer_expand_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .valign(gtk::Align::End)
            .build();
     
        let back_icon_button = gtk::Box::builder()
        .vexpand(false)
        .hexpand(false)
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
        }));

        left_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button released is {}", this.current_button());
                let _ = sender.output(Message::BackPressed);
        }));

        back_icon_button.append(&back_icon); 
        back_icon_button.add_controller(left_click_gesture);
        footer_expand_box.append(&back_icon_button);

        let trash_icon_button = gtk::Box::builder()
        .vexpand(false)
        .hexpand(true)
        .halign(gtk::Align::End)
        .valign(gtk::Align::End)
        .css_classes(["footer-back-icon"])
        .build();

        let trash_icon = get_image_from_path(widget_configs.footer.trash_icon, &["back-icon"]);
        trash_icon.set_vexpand(true);
        trash_icon.set_hexpand(true);
        trash_icon.set_halign(gtk::Align::Center);
        trash_icon.set_valign(gtk::Align::Center);

        trash_icon_button.append(&trash_icon); 
        footer_expand_box.append(&trash_icon_button);

        footer.append(&footer_expand_box);
        root.append(&footer);

        let model = NetworkDetailsPage { settings: init };

        let widgets = NetworkDetailsPageWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::HomeIconPressed => {
                sender.output(Message::HomeIconPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
