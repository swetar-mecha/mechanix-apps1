mod pages;
mod settings;
pub mod errors;
mod server; 
mod handlers;

use async_trait::async_trait;
use gtk::prelude::{BoxExt, GtkWindowExt};
use pages::{
    check_internet::{
        self, CheckInternet, CheckInternetOutput, Settings as CheckInternetSettings
    }, configure_machine::{
        ConfigureMachine, ConfigureOutput, Settings as ConfigureMachineSettings,
    }, link_machine::{LinkMachine, LinkMachineOutput, Settings as LinkMachineSettings}, machine_info::{DevicePageOutput, MachineInfo, Settings as DeviceInfoSettings}, no_internet::{NoInternet, PageOutput, Settings as NoInternetSettings}, setup_failed::{Settings as SetupFailedSettings, SetupFailOutput, SetupFailed}, setup_success::{Settings as SetupSuccessSettings, SetupSuccess, SetupSuccessOutput}, start_screen::{Settings as StartScreenSettings, StartScreen, StartScreenOutput}, timeout_screen::{Settings as TimeoutScreenSettings, TimeoutOutput, TimeoutScreen }
};
use relm4::{component::{AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncController}, gtk::glib::clone, AsyncComponentSender, SimpleComponent};
use relm4::{gtk, ComponentController};
use relm4::{Component, Controller, RelmApp};
use settings::{Modules, ScreenSettings, WidgetConfigs};
use std::{fmt};
use tracing::info;

use crate::pages::start_screen;

#[derive(Debug)]

struct ErrorMessage {
    error: String
}

struct MechaConnectApp {
    current_page: Pages,
    pages_stack: gtk::Stack,
    link_machine: AsyncController<LinkMachine>,
    start_screen: Controller<StartScreen>,
    check_internet: AsyncController<CheckInternet>,
    configure_machine: AsyncController<ConfigureMachine>,
    timeout_screen: Controller<TimeoutScreen>,
    setup_failed:  Controller<SetupFailed>,
    // machine_info: AsyncController<MachineInfo>
}

struct errorInfo {
    error_message : String,
    from_screen: String
}

#[derive(Debug)]
enum Pages {
    StartScreen,
    CheckInternet,
    NoInternet,
    LinkMachine,
    ConfigureMachine,
    TimeoutScreen,
    SetupSuccess,
    SetupFailed(String, String),
    MachineInfo, 
}

impl fmt::Display for Pages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pages::StartScreen => write!(f, "start_screen"),
            Pages::CheckInternet => write!(f, "check_internet"),
            Pages::NoInternet => write!(f, "no_internet"),
            Pages::LinkMachine => write!(f, "link_machine"),
            Pages::ConfigureMachine => write!(f, "configure_machine"),
            Pages::TimeoutScreen => write!(f, "timeout_screen"),
            Pages::SetupSuccess => write!(f, "setup_success"),
            Pages::SetupFailed(error, from_screen) => write!(f, "setup_failed"),
            Pages::MachineInfo => write!(f, "machine_info"),
        }
    }
}

#[derive(Debug)]
enum Message {
    ChangeScreen(Pages),
    // NextPressed
}
#[derive(Debug)]
enum AppInput {}

struct AppWidgets {
    pages_stack: gtk::Stack,
}

fn init_window(settings: ScreenSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title("Mecha Connect")
        .default_width(window_settings.size.0)
        .default_height(window_settings.size.1)
        .css_classes(["window"])
        .build();
    window
}

#[async_trait(?Send)]
impl AsyncComponent for MechaConnectApp {
    type Input = Message;
    type Output = ();
    type Init = ();
    type Root = gtk::Window;
    type Widgets = AppWidgets;
    type CommandOutput = Message;

    fn init_root() -> Self::Root {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => ScreenSettings::default(),
        };

        info!(
            task = "initalize_settings",
            "settings initialized for Lock Screen: {:?}", settings
        );

        let window = init_window(settings);
        window
    }

    /// Initialize the UI and model.
    async fn init(
        _: Self::Init,
        window: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {

        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => ScreenSettings::default(),
        };

        let css = settings.css.clone();
        relm4::set_global_css_from_file(css.default);

        let modules = settings.modules.clone();
        let widget_configs = settings.widget_configs.clone();

        // // HERE grpc call
        // let generate_code_data = get_provision_data().await;


        let start_screen = create_start_screen(
            settings.modules.clone(),
            settings.widget_configs.clone(),
            sender.input_sender().clone(),
        );

        let check_internet = create_check_internet(
            settings.modules.clone(),
            settings.widget_configs.clone(),
            sender.input_sender().clone(),
        );


        // let start_screen: Controller<StartScreen> = StartScreen::builder()
        //     .launch(StartScreenSettings {
        //         modules: modules.clone(),
        //         widget_configs: widget_configs.clone(),
        //     })
        //     .forward(
        //         &sender.input_sender().clone(),
        //         clone!(@strong modules => move|msg| match msg {
        //             StartScreenOutput::BackPressed => Message::ChangeScreen(Pages::StartScreen),
        //             StartScreenOutput::NextPressed => Message::ChangeScreen(Pages::CheckInternet)
        //         }),
        //     );

        // let check_internet = CheckInternet::builder()
        //     .launch(CheckInternetSettings {
        //         modules: modules.clone(),
        //         widget_configs: widget_configs.clone(),
        //     })
        //     .forward(
        //         sender.input_sender(),
        //         clone!(@strong modules => move|msg| match msg {
        //             CheckInternetOutput::BackPressed => Message::ChangeScreen(Pages::StartScreen),
        //             CheckInternetOutput::LinkMachine => Message::ChangeScreen(Pages::LinkMachine),
        //             CheckInternetOutput::ConnectionNotFound => Message::ChangeScreen(Pages::NoInternet),
        //             CheckInternetOutput::ShowError(error) => {
        //                 println!("MAIN ShowError : {:?} ", error);
        //                 Message::ChangeScreen(Pages::SetupFailed(error, "check_internet".to_string()))
        //             },
        //             // CheckInternetOutput::ShowError => Message::ChangeScreen(Pages::SetupFailed()),
        //         }),
        //     );

        let no_internet: Controller<NoInternet> = NoInternet::builder()
            .launch(NoInternetSettings {
                modules: modules.clone(),
                widget_configs: widget_configs.clone(),
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| match msg {
                    PageOutput::BackPressed => Message::ChangeScreen(Pages::CheckInternet),
                    // PageOutput::NextPressed => { return Message::NextPressed}
                    PageOutput::NextPressed => Message::ChangeScreen(Pages::LinkMachine)
                }),
            );

        let link_machine = LinkMachine::builder().launch(LinkMachineSettings{
            modules: modules.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| match msg {
                LinkMachineOutput::BackPressed => Message::ChangeScreen(Pages::CheckInternet),
                LinkMachineOutput::NextPressed => Message::ChangeScreen(Pages::ConfigureMachine),
                LinkMachineOutput::ShowError => Message::ChangeScreen(Pages::SetupFailed("".to_owned(), "".to_owned())),
            }),
        );

        let configure_machine = ConfigureMachine::builder()
            .launch(ConfigureMachineSettings {
                modules: modules.clone(),
                widget_configs: widget_configs.clone(),
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| match msg {
                    // ConfigureOutput::BackPressed => Message::ChangeScreen(Pages::TimeoutScreen),
                    // ConfigureOutput::NextPressed => Message::ChangeScreen(Pages::SetupSuccess)
                    ConfigureOutput::NextPressed => Message::ChangeScreen(Pages::TimeoutScreen)
                }),
            );

        let timeout_screen = TimeoutScreen::builder().launch(TimeoutScreenSettings {
            modules: modules.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| match msg {
                TimeoutOutput::refreshPressed => Message::ChangeScreen(Pages::MachineInfo),
                TimeoutOutput::BackPressed => Message::ChangeScreen(Pages::ConfigureMachine)
            }),
        );

        let setup_success = SetupSuccess::builder().launch(SetupSuccessSettings {
            modules: modules.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| match msg {
                SetupSuccessOutput::BackPressed => Message::ChangeScreen(Pages::ConfigureMachine),
                SetupSuccessOutput::NextPressed => 
                Message::ChangeScreen(Pages::SetupFailed(String::from(""),"".to_owned()))
            }),
        );

        let setup_failed = SetupFailed::builder()
            .launch(SetupFailedSettings {
                modules: modules.clone(),
                widget_configs: widget_configs.clone(),
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| match msg {
                    SetupFailOutput::BackPressed=>Message::ChangeScreen(Pages::CheckInternet),SetupFailOutput::NextPressed=>Message::ChangeScreen(Pages::MachineInfo),
                    SetupFailOutput::refresh(screen) => {
                        println!("REFRESH SCREEN: {:?}", screen.to_owned());

                        match screen {
                            screen if screen == String::from("check_internet") =>  Message::ChangeScreen(Pages::CheckInternet),
                            screen if screen == String::from("configure_machine") =>  Message::ChangeScreen(Pages::ConfigureMachine),
                            _ => {
                                println!("Found something else");
                            Message::ChangeScreen(Pages::MachineInfo)},

                        }
                        // Message::ChangeScreen(Pages::MachineInfo)

                    }

                    }),
            );

        let machine_info = MachineInfo::builder()
            .launch(DeviceInfoSettings {
                modules: modules.clone(),
                widget_configs: widget_configs.clone(),
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| match msg {
                    DevicePageOutput::BackPressed => Message::ChangeScreen(Pages::CheckInternet),
                    DevicePageOutput::NextPressed => Message::ChangeScreen(Pages::MachineInfo)
                }),
            );

        let pages_stack = gtk::Stack::builder().build();

        pages_stack.add_named(
            start_screen.widget(),
            Option::from(Pages::StartScreen.to_string().as_str()),
        );

        pages_stack.add_named(
            check_internet.widget(),
            Option::from(Pages::CheckInternet.to_string().as_str()),
        );

        pages_stack.add_named(
            no_internet.widget(),
            Option::from(Pages::NoInternet.to_string().as_str()),
        );

        pages_stack.add_named(
            link_machine.widget(),
            Option::from(Pages::LinkMachine.to_string().as_str()),
        );

        pages_stack.add_named(
            configure_machine.widget(),
            Option::from(Pages::ConfigureMachine.to_string().as_str()),
        );

        pages_stack.add_named(
            timeout_screen.widget(),
            Option::from(Pages::TimeoutScreen.to_string().as_str()),
        );

        pages_stack.add_named(
            setup_success.widget(),
            Option::from(Pages::SetupSuccess.to_string().as_str()),
        );

        pages_stack.add_named(
            setup_failed.widget(),
            Option::from(Pages::SetupFailed("".to_owned(),"".to_owned()).to_string().as_str()),
        );

        pages_stack.add_named(
            machine_info.widget(),
            Option::from(Pages::MachineInfo.to_string().as_str()),
        );

        let current_page = Pages::StartScreen;   // OG
        // let current_page = Pages::ConfigureMachine;
        // let current_page = Pages::SetupSuccess;

        //Setting current active screen in stack
        pages_stack.set_visible_child_name(&current_page.to_string());
        // pages_stack.set_transition_type(gtk::StackTransitionType::Crossfade);
        // pages_stack.set_transition_duration(300);

        // add pages here
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .hexpand(true)
            .build();

        vbox.append(&pages_stack);

        let model = MechaConnectApp { 
            current_page, 
            pages_stack:pages_stack.clone(),
            start_screen,
            link_machine,
            check_internet,
            configure_machine,
            timeout_screen,
            setup_failed
            // machine_info
        };

        window.set_child(Some(&vbox));
        let widgets = AppWidgets { 
            pages_stack,
         };

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        println!("MAIN UPDATE {:?}", message);


        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => ScreenSettings::default(),
        };


        let modules = settings.modules.clone();
        let widget_configs = settings.widget_configs.clone();

        match message {
            Message::ChangeScreen(page)=>{
            // __self.current_page=page;

            // match &self.current_page {
            match &page {
                Pages::StartScreen => {
                    self.start_screen.detach_runtime();

                    let start_screen = create_start_screen(
                        settings.modules.clone(),
                        settings.widget_configs.clone(),
                        sender.input_sender().clone(),
                    );

                    self.pages_stack.remove(
                        &self
                            .pages_stack
                            .child_by_name(
                                Pages::StartScreen.to_string().as_str(),
                            )
                            .unwrap(),
                    );

               
                    self.pages_stack.add_named(
                            start_screen.widget(),
                            Option::from(Pages::StartScreen.to_string().as_str()),
                        );

                    self.start_screen = start_screen;

                    // self.start_screen.sender().send(message)
                

                },
                Pages::CheckInternet => {

                    let _ = __self.check_internet.sender().send(pages::check_internet::InputMessage::ActiveScreen(self.current_page.to_string()));


                    self.check_internet.detach_runtime();

                    let check_internet = create_check_internet(
                        settings.modules.clone(),
                        settings.widget_configs.clone(),
                        sender.input_sender().clone(),
                    );

                    self.pages_stack.remove(
                        &self
                            .pages_stack
                            .child_by_name(
                                Pages::CheckInternet.to_string().as_str(),
                            )
                            .unwrap(),
                    );

               
                    self.pages_stack.add_named(
                            check_internet.widget(),
                            Option::from(Pages::CheckInternet.to_string().as_str()),
                        );

                    self.check_internet = check_internet;


                },
                Pages::NoInternet => {},
                Pages::LinkMachine => {
                    println!("THIS  IS  link_machine : {:?} ", self.current_page);

                    // active screen
                    // let _ = __self.link_machine.sender().send(pages::link_machine::InputMessage::ActiveScreen(self.current_page));

                    let _ = __self.link_machine.sender().send(pages::link_machine::InputMessage::ActiveScreen(self.current_page.to_string()));
                },
                Pages::ConfigureMachine => {
                    println!("ConfigureMachine ");
                    let _ = __self.configure_machine.sender().send(pages::configure_machine::InputMessage::ActiveScreen(self.current_page.to_string()));
                },
                Pages::TimeoutScreen => {
                    println!("TimeoutScreen ");
                    // let _ = __self.timeout_screen.sender().send(pages::timeout_screen::InputMessage::ActiveScreen(self.current_page.to_string()));
                },
                Pages::SetupSuccess => {},
                Pages::SetupFailed(error, from_screen) => {
                    println!("SetupFailed error {:?}",  error.to_string());
                    let _ = __self.setup_failed.sender().send(pages::setup_failed::InputMessage::ShowError(error.to_string(), from_screen.to_string()));

                },
                Pages::MachineInfo => {
                    println!("MachineInfo ");
                    // let _ = __self.machine_info.sender().send(pages::machine_info::InputMessage::ActiveScreen(self.current_page.to_string()));
                },
            }
            self.current_page=page;

                        

            // let _ = __self.link_machine.sender().send(pages::link_machine::InputMessage::GenerateCodeRequest);
        
        
        }

     }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: AsyncComponentSender<Self>) {
        println!("main update_view {:?} ", self.current_page);

        widgets
            .pages_stack
            .set_visible_child_name(self.current_page.to_string().as_str());
    }
}

fn create_start_screen(
    modules: Modules,
    widget_configs : WidgetConfigs,
    sender: relm4::Sender<Message>,
) -> Controller<StartScreen> {
    let start_screen: Controller<StartScreen> = StartScreen::builder()
    .launch(StartScreenSettings {
        modules: modules.clone(),
        widget_configs: widget_configs.clone(),
    })
    .forward(
         &sender,
        clone!(@strong modules => move|msg| match msg {
            StartScreenOutput::BackPressed => Message::ChangeScreen(Pages::StartScreen),
            StartScreenOutput::NextPressed => Message::ChangeScreen(Pages::CheckInternet)
        }),
    );

    start_screen
}


fn create_check_internet(
    modules: Modules,
    widget_configs : WidgetConfigs,
    sender: relm4::Sender<Message>,
) -> AsyncController<CheckInternet> {
    let check_internet = CheckInternet::builder()
        .launch(CheckInternetSettings {
            modules: modules.clone(),
            widget_configs: widget_configs.clone(),
        })
        .forward(&sender, move |msg| {
            info!("check internet {:?}", msg);
            match msg {
                CheckInternetOutput::BackPressed => Message::ChangeScreen(Pages::StartScreen),
                CheckInternetOutput::LinkMachine => Message::ChangeScreen(Pages::LinkMachine),
                CheckInternetOutput::ConnectionNotFound => Message::ChangeScreen(Pages::NoInternet),
                CheckInternetOutput::ShowError(error) => {
                    println!("MAIN OUTSIDE ShowError : {:?} ", error);
                    Message::ChangeScreen(Pages::SetupFailed(error, "check_internet".to_string()))
                },
            }
        });
    check_internet
}



#[tokio::main]
async fn main() {
    let app = RelmApp::new("mecha.connect.app");

    app.run_async::<MechaConnectApp>(());
}
