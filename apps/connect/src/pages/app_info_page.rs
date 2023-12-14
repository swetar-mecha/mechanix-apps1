use custom_utils::get_image_from_path;
use gtk::prelude::*;
use relm4::{gtk::{self, prelude::{WidgetExt, ButtonExt, StyleContextExt}, Button,  glib::clone, pango}, ComponentParts, ComponentSender, SimpleComponent};
use crate::settings::{Modules, WidgetConfigs};

pub struct Settings {
    pub modules: Modules,
    pub widget_configs: WidgetConfigs,
}
pub struct AppInfoPage {
    settings: Settings,
}

#[derive(Debug)]
pub enum AppInfoOutput {
    BackPressed,
    NextPressed
}

pub struct AppWidgets {
}

impl SimpleComponent for AppInfoPage {

    type Init = Settings;
    type Input = ();
    type Output = AppInfoOutput;
    type Root = gtk::Box;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
        .build()
    }

    /// Initialize the UI and model.
    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        // let model = AppInfoPage { };
        let modules = init.modules.clone();
        let widget_configs = init.widget_configs.clone();


        let model = AppInfoPage { settings: init };

        let main_content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["app-container"])
        .build();

        let footer_content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .vexpand(true)
        .valign(gtk::Align::End)
        .css_classes(["footer-container"])
        .build();

        // hbox_line1
        let hbox_line1 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes(["app-info-header-box"])
        .build();

        let app_icon_path = modules.pages_settings.app_info.app_icon.clone();

        let app_icon: gtk::Image = get_image_from_path(
            app_icon_path,
            &["app-icon"],
        );

        let label1 = gtk::Label::builder()
        .label("Mecha Connect")
        .halign(gtk::Align::Start)
        .build();
        label1.style_context().add_class("app-info-header");

        hbox_line1.append(&app_icon);
        hbox_line1.append(&label1);

        main_content_box.append(&hbox_line1);   // main box

        let sentence = gtk::Label::builder()
        .label("Please sign up on mecha.so before getting started.")
        .halign(gtk::Align::Start)
        .build();

        sentence.style_context().add_class("app-info-header-label");

        main_content_box.append(&sentence);

        let info_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["app-info-steps-container"])
        .build();

        let hbox_line2 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["app-info-steps-box"])
        .build();
       
        let icon2: gtk::Image = get_image_from_path(
            modules.pages_settings.app_info.virtual_network_icon.clone(),
            &["app-info-steps-icon"],
        );

        let label2 = gtk::Label::builder()
        .label("Virtual networking to enable connecting to your machine remotely")
        .css_classes(["app-info-steps-label"])
        .wrap(true)
        .wrap_mode(pango::WrapMode::Word) 
        .hexpand(true)
        .build();

        hbox_line2.append(&icon2);
        hbox_line2.append(&label2);

        info_box.append(&hbox_line2);

        let hbox_line3 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["app-info-steps-box"])
        .build(); 

        let icon3: gtk::Image = get_image_from_path(
            modules.pages_settings.app_info.real_time_icon.clone(),
            &["app-info-steps-icon"],
        );

        let label3 = gtk::Label::builder()
        .label("Integrated Telemetry that collects logs and metrics in real-time")
        .css_classes(["app-info-steps-label"])
        .wrap(true)
        .wrap_mode(pango::WrapMode::Word) 
        .hexpand(true)
        .build();

        hbox_line3.append(&icon3);
        hbox_line3.append(&label3);

        info_box.append(&hbox_line3);

        let hbox_line4 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["app-info-steps-box"])
        .build();

        let icon4: gtk::Image = get_image_from_path(
            modules.pages_settings.app_info.encypt_icon.clone(),
            &["app-info-steps-icon"],
        );

        let label4: gtk::Label = gtk::Label::builder()
        .label("Secure and encrypted messaging from-to your machine")
        .css_classes(["app-info-steps-label"])
        .wrap(true)
        .wrap_mode(pango::WrapMode::Word) 
        .hexpand(true)
        .build();

        hbox_line4.append(&icon4);
        hbox_line4.append(&label4);

        info_box.append(&hbox_line4);
        main_content_box.append(&info_box);

        let footer_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .valign(gtk::Align::End)
        .build();

        let back_icon_img: gtk::Image = get_image_from_path(
            widget_configs.footer.back_icon,
            &[],
        );
        let back_button_box = gtk::Box::builder().hexpand(true).build();
        let back_button = Button::builder().build();
        back_button.set_child(Some(&back_icon_img));
        back_button.add_css_class("footer-container-button");

        back_button.connect_clicked(clone!(@strong sender => move |_| {
            let _ =  sender.output(AppInfoOutput::BackPressed);
          }));

        let next_icon_img: gtk::Image = get_image_from_path(
            widget_configs.footer.next_icon,
            &[],
        );
        let next_button = Button::new();
        next_button.set_child(Some(&next_icon_img));
        next_button.add_css_class("footer-container-button");

        next_button.connect_clicked(clone!(@strong sender => move |_| {
          let _ =  sender.output(AppInfoOutput::NextPressed);
        }));

        back_button_box.append(&back_button);
        footer_box.append(&back_button_box);
        footer_box.append(&next_button);

        footer_content_box.append(&footer_box);
        main_content_box.append(&footer_content_box);

        root.append(&main_content_box);

        let widgets = AppWidgets {  };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
    }

}