use crate::settings::{Modules, WidgetConfigs};
use custom_utils::{get_gif_from_path, get_image_from_path};
use gtk::prelude::*;
use relm4::{
    gtk::{
        self,
        glib::clone,
        prelude::{ButtonExt, WidgetExt},
        Button,
    },
    ComponentParts, ComponentSender, SimpleComponent,
};

pub struct Settings {
    pub modules: Modules,
    pub widget_configs: WidgetConfigs,
}
pub struct CheckInternetPage {
    settings: Settings,
}

#[derive(Debug)]
enum AppInput {}

#[derive(Debug)]
pub enum CheckInternetOutput {
    BackPressed,
    NextPressed,
}

pub struct AppWidgets {}

impl SimpleComponent for CheckInternetPage {
    type Init = Settings;
    type Input = ();
    type Output = CheckInternetOutput;
    type Root = gtk::Box;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder().build()
    }

    /// Initialize the UI and model.
    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let modules = init.modules.clone();
        let widget_configs = init.widget_configs.clone();

        let model = CheckInternetPage { settings: init };

        let main_content_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["app-container", "check-internet-text"])
            .build();

        let footer_content_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(true)
            .valign(gtk::Align::End)
            .css_classes(["footer-container"])
            .build();

        // get gif
        let gif_path = modules.pages_settings.check_internet.search_wifi.clone();
        let paintable = get_gif_from_path(gif_path);

        let image_from = gtk::Image::builder()
            .width_request(290)
            .height_request(290)
            .paintable(&paintable)
            .css_classes(["gif-img"])
            .build();

        let label1: gtk::Label = gtk::Label::builder()
            .label("Checking for internet connectivity...")
            .build();

        let label2: gtk::Label = gtk::Label::builder().label("Please wait").build();

        main_content_box.append(&image_from);
        main_content_box.append(&label1);
        main_content_box.append(&label2);

        let footer_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .build();

        let back_icon_img: gtk::Image = get_image_from_path(widget_configs.footer.back_icon, &[]);
        let back_button_box = gtk::Box::builder().hexpand(true).build();
        let back_button = Button::builder().build();
        back_button.set_child(Some(&back_icon_img));
        back_button.add_css_class("footer-container-button");

        back_button.connect_clicked(clone!(@strong sender => move |_| {
          let _ =  sender.output(CheckInternetOutput::BackPressed);
        }));

        let next_icon_img: gtk::Image = get_image_from_path(widget_configs.footer.next_icon, &[]);
        let next_button = Button::new();
        next_button.set_child(Some(&next_icon_img));
        next_button.add_css_class("footer-container-button");

        next_button.connect_clicked(clone!(@strong sender => move |_| {
          let _ =  sender.output(CheckInternetOutput::NextPressed);
        }));

        back_button_box.append(&back_button);
        footer_box.append(&back_button_box);
        footer_box.append(&next_button);

        footer_content_box.append(&footer_box);
        main_content_box.append(&footer_content_box);

        root.append(&main_content_box);

        let widgets = AppWidgets {};

        ComponentParts { model, widgets }
    }
}
