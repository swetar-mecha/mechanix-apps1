use crate::settings::{Modules, WidgetConfigs};
use custom_utils::{get_gif_from_path, get_image_from_path};
use gtk::prelude::*;
use relm4::{
    gtk::{
        self,
        gdk::Display,
        glib::clone,
        prelude::{ButtonExt, WidgetExt},
        Button, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION,
    },
    ComponentParts, ComponentSender, SimpleComponent,
};

pub struct Settings {
    pub modules: Modules,
    pub widget_configs: WidgetConfigs,
}
pub struct SetupFailedPage {
    settings: Settings,
}

#[derive(Debug)]
enum AppInput {}

#[derive(Debug)]
pub enum SetupFailOutput {
    BackPressed,
    NextPressed,
}

pub struct AppWidgets {}

impl SimpleComponent for SetupFailedPage {
    type Init = Settings;
    type Input = ();
    type Output = SetupFailOutput;
    type Root = gtk::Box;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("../assets/css/style.css"));
        gtk::style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        gtk::Box::builder().build()
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let modules = init.modules.clone();
        let widget_configs = init.widget_configs.clone();

        let model = SetupFailedPage { settings: init };

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

        // get gif
        let gif_path = modules.pages_settings.setup_failure.failure.clone();
        let paintable = get_gif_from_path(gif_path);

        let image_from = gtk::Image::builder()
            .width_request(262)
            .height_request(262)
            .css_classes(["gif-img"])
            .paintable(&paintable)
            .build();

        main_content_box.append(&image_from);

        // bold
        let label: gtk::Label = gtk::Label::builder()
            .label("Error setting up your machine")
            .css_classes(["setup-status-label"])
            .build();

        main_content_box.append(&label);

        let info_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();

        let info_label: gtk::Label = gtk::Label::builder()
            .label("Low internet connectivity")
            .hexpand(true)
            .css_classes(["setup-fail-info"])
            .build();

        info_box.append(&info_label);
        main_content_box.append(&info_box);

        // footer_box
        let footer_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .vexpand(true)
            .valign(gtk::Align::End)
            .build();

        let back_icon_img: gtk::Image = get_image_from_path(widget_configs.footer.back_icon, &[]);

        let back_button_box = gtk::Box::builder().hexpand(true).build();
        let back_button = Button::builder().build();
        back_button.set_child(Some(&back_icon_img));
        back_button.add_css_class("footer-container-button");

        back_button.connect_clicked(clone!(@strong sender => move |_| {
          let _ =  sender.output(SetupFailOutput::BackPressed);
        }));

        let refresh_icon_img: gtk::Image =
            get_image_from_path(widget_configs.footer.refresh_icon, &[]);
        let refresh_button = Button::new();
        refresh_button.set_child(Some(&refresh_icon_img));
        refresh_button.add_css_class("footer-container-button");

        refresh_button.connect_clicked(clone!(@strong sender => move |_| {
          let _ =  sender.output(SetupFailOutput::NextPressed);
        }));

        back_button_box.append(&back_button);
        footer_box.append(&back_button_box);
        footer_box.append(&refresh_button);

        footer_content_box.append(&footer_box);
        main_content_box.append(&footer_content_box);

        root.append(&main_content_box);

        let widgets = AppWidgets {};

        ComponentParts { model, widgets }
    }
}
