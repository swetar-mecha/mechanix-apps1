use custom_utils::{get_gif_from_path, get_image_from_path};
use gtk::prelude::*;
use relm4::{gtk::{self, prelude::{WidgetExt, ButtonExt, StyleContextExt}, Button,  glib::clone, pango}, ComponentParts, ComponentSender, SimpleComponent};
use crate::settings::{Modules, WidgetConfigs};

pub struct Settings {
    pub modules: Modules,
    pub widget_configs: WidgetConfigs,
}
pub struct StartScreen {
    settings: Settings,
}

#[derive(Debug)]
pub enum StartScreenOutput {
    BackPressed,
    NextPressed
}

pub struct AppWidgets {
}

impl SimpleComponent for StartScreen {

    type Init = Settings;
    type Input = ();
    type Output = StartScreenOutput;
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
        // let model = StartScreen { };
        let modules = init.modules.clone();
        let widget_configs = init.widget_configs.clone();

        let model = StartScreen { settings: init };

        let main_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();


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
        let header_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes(["start-screen-header-box"])
        .build();

        let app_icon_path: Option<String> = modules.pages_settings.start_screen.app_icon.clone();

        let app_icon: gtk::Image = get_image_from_path(
            app_icon_path,
            &["app-icon"],
        );

        let label1 = gtk::Label::builder()
        .label("Connect to Mecha")
        .halign(gtk::Align::Start)
        .build();
        label1.style_context().add_class("start-screen-header");

        header_box.append(&app_icon);
        header_box.append(&label1);

        // main_content_box.append(&header_box);   // main box
        main_container.append(&header_box);   // main box

        let sentence = gtk::Label::builder()
        .label("Please sign up on mecha.so before getting started.")
        .css_classes(["start-screen-header-label"])
        .halign(gtk::Align::Start)
        .build();

        // sentence.style_context().add_class("start-screen-header-label");

        main_content_box.append(&sentence);

        let info_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["start-screen-steps-container"])
        .build();

        let hbox_line2 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["start-screen-steps-box"])
        .build();
       
        let icon2: gtk::Image = get_image_from_path(
            modules.pages_settings.start_screen.virtual_network_icon.clone(),
            &["start-screen-steps-icon"],
        );

        let label2 = gtk::Label::builder()
        .label("Mesh Networking to enable global connectivity between your machines")
        .css_classes(["start-screen-steps-label"])
        .wrap(true)
        .wrap_mode(pango::WrapMode::Word) 
        .hexpand(true)
        // .justify(gtk::Justification::Fill)
        .halign(gtk::Align::Start)
        .build();

        hbox_line2.append(&icon2);
        hbox_line2.append(&label2);

        info_box.append(&hbox_line2);

        let hbox_line3 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["start-screen-steps-box"])
        .build(); 

        let icon3: gtk::Image = get_image_from_path(
            modules.pages_settings.start_screen.real_time_icon.clone(),
            &["start-screen-steps-icon"],
        );

        let label3 = gtk::Label::builder()
        .label("Integrated metrics and logs collection, compatible with OpenTelemetry")
        .css_classes(["start-screen-steps-label"])
        .wrap(true)
        .wrap_mode(pango::WrapMode::Word) 
        .hexpand(true)
        // .justify(gtk::Justification::Fill)
        .halign(gtk::Align::Start)
        .build();

        hbox_line3.append(&icon3);
        hbox_line3.append(&label3);

        info_box.append(&hbox_line3);

        let hbox_line4 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["start-screen-steps-box"])
        .build();

        let icon4: gtk::Image = get_image_from_path(
            modules.pages_settings.start_screen.encypt_icon.clone(),
            &["start-screen-steps-icon"],
        );

        let label4: gtk::Label = gtk::Label::builder()
        .label("Identity management using secure x.509 certificates")
        .css_classes(["start-screen-steps-label"])
        .wrap(true)
        .wrap_mode(pango::WrapMode::Word) 
        .hexpand(true)
        // .justify(gtk::Justification::Fill)
        .halign(gtk::Align::Start)
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
            let _ =  sender.output(StartScreenOutput::BackPressed);
          }));

        let next_icon_img: gtk::Image = get_image_from_path(
            widget_configs.footer.next_icon,
            &[],
        );
        let next_button = Button::new();
        next_button.set_child(Some(&next_icon_img));
        next_button.add_css_class("footer-container-button");

        next_button.connect_clicked(clone!(@strong sender => move |_| {
          let _ =  sender.output(StartScreenOutput::NextPressed);
        }));

        back_button_box.append(&back_button);
        footer_box.append(&back_button_box);
        footer_box.append(&next_button);

        footer_content_box.append(&footer_box);
        main_content_box.append(&footer_content_box);
        main_container.append(&main_content_box);   // main box


        root.append(&main_container);

        let widgets = AppWidgets {  };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
    }

}