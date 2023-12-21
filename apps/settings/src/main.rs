use std::fmt;

use gtk::{glib::clone, prelude::GtkWindowExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent};
use relm4::{Component, ComponentController, Controller};

mod pages;
mod settings;
mod theme;
mod widgets;
use pages::{
    add_network_page::{
        AddNetworkPage, Message as AddNetworkPageMessage, Settings as AddNetworkPageSettings,
    },
    bluetooth_details_page::{
        BluetoothDetailsPage, Message as BluetoothDetailsPageMessage,
        Settings as BluetoothDetailsPageSettings,
    },
    bluetooth_pair_request_page::{
        BluetoothPairRequestPage, Message as BluetoothPairRequestPageMessage,
        Settings as BluetoothPairRequestPageSettings,
    },
    connect_bluetooth_page::{
        ConnectBluetoothPage, Message as ConnectBluetoothPageMessage,
        Settings as ConnectBluetoothPageSettings,
    },
    connect_network_page::{
        ConnectNetworkPage, Message as ConnectNetworkPageMessage,
        Settings as ConnectNetworkPageSettings,
    },
    display_page::{DisplayPage, Message as DisplayPageMessage, Settings as DisplayPageSettings},
    battery_page::{BatteryPage, Message as BatteryPageMessage, Settings as BatteryPageSettings},
    home::{HomePage, Message as HomePageMessage, Settings as HomePageSettings},
    lock_timeout_page::{
        LockTimeoutPage, Message as LockTimeoutPageMessage, Settings as LockTimeoutPageSettings,
    },
    reset_pin_page::{
        ResetPinPage, Message as ResetPinPageMessage, Settings as ResetPinPageSettings,
    },
    manage_bluetooth_page::{
        ManageBluetoothPage, Message as ManageBluetoothPageMessage,
        Settings as ManageBluetoothPageSettings,
    },
    manage_networks_page::{
        ManageNetworksPage, Message as ManageNetworksPageMessage,
        Settings as ManageNetworksPageSettings,
    },
    network_details_page::{
        Message as NetworkDetailsPageMessage, NetworkDetailsPage,
        Settings as NetworkDetailsPageSettings,
    },
    networks_page::{
        Message as NetworksPageMessage, NetworksPage, Settings as NetworksPageSettings,
    },
    password_authentication::{
        Message as PasswordAuthenticationMessage, PasswordAuthentication,
        Settings as PasswordAuthenticationSettings,
    },
    performance_mode_page::{
        Message as PerformanceModePageMessage, PerformanceModePage,
        Settings as PerformanceModePageSettings,
    },
    pin_authentication::{
        Message as PinAuthenticationMessage, PinAuthentication,
        Settings as PinAuthenticationSettings,
    },
    screen_timeout_page::{
        Message as ScreenTimeoutPageMessage, ScreenTimeoutPage,
        Settings as ScreenTimeoutPageSettings,
    },
    security_page::{
        Message as SecurityPageMessage, SecurityPage, Settings as SecurityPageSettings,
    },
    settings_page::{
        OutputMessage as SettingsPageMessage, Settings as SettingsPageSettings, SettingsPage,
    },
    sound_page::{Message as SoundPageMessage, Settings as SoundPageSettings, SoundPage},
    date_time_page::{
        Message as DateTimePageMessage, DateTimePage, Settings as DateTimePageSettings,
    },
    set_time_page::{
        Message as SetTimePageMessage, SetTimePage, Settings as SetTimePageSettings,
    },
    set_date_page::{
        Message as SetDatePageMessage, SetDatePage, Settings as SetDatePageSettings,
    },
    about_page::{
        Message as AboutPageMessage, AboutPage, Settings as AboutPageSettings,
    }
};
use settings::LockScreenSettings;
use tracing::info;
pub mod errors;
use crate::theme::LockScreenTheme;

/// # LockScreen State
///
/// This struct is the state definition of the entire application
struct LockScreen {
    current_screen: Screens,
    settings: LockScreenSettings,
    custom_theme: LockScreenTheme,
    home_page: Controller<HomePage>,
    settings_page: Controller<SettingsPage>,
    network_page: Controller<NetworksPage>,
    manage_networks_page: Controller<ManageNetworksPage>,
    network_details_page: Controller<NetworkDetailsPage>,
    connect_network_page: Controller<ConnectNetworkPage>,
    add_network_page: Controller<AddNetworkPage>,
    manage_bluetooth_page: Controller<ManageBluetoothPage>,
    bluetooth_details_page: Controller<BluetoothDetailsPage>,
    connect_bluetooth_page: Controller<ConnectBluetoothPage>,
    bluetooth_pair_request_page: Controller<BluetoothPairRequestPage>,
    display_page: Controller<DisplayPage>,
    screen_timeout_page: Controller<ScreenTimeoutPage>,
    sound_page: Controller<SoundPage>,
    performance_mode_page: Controller<PerformanceModePage>,
    security_page: Controller<SecurityPage>,
    lock_timeout_page: Controller<LockTimeoutPage>,
    battery_page: Controller<BatteryPage>,
    reset_pin_page: Controller<ResetPinPage>,
    date_time_page: Controller<DateTimePage>,
    set_time_page: Controller<SetTimePage>,
    set_date_page: Controller<SetDatePage>,
    about_page: Controller<AboutPage>,
}

#[derive(Debug, Clone)]
pub enum Screens {
    LockScreen,
    PasswordScreen,
    PinScreen,
    Home,
    Settings,
    Network,
    ManageNetworks,
    NetworkDetails,
    ConnectNetwork,
    AddNetwork,
    ManageBluetooth,
    BluetoothDetails,
    ConnectBluetooth,
    BluetoothPairRequest,
    Display,
    ScreenTimeout,
    Sound,
    PerformanceMode,
    Security,
    LockTimeout,
    Battery,
    ResetPin,
    DateTime,
    SetTime,
    SetDate,
    About
}

impl fmt::Display for Screens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Screens::LockScreen => write!(f, "lock_screen"),
            Screens::PasswordScreen => write!(f, "password_screen"),
            Screens::PinScreen => write!(f, "pin_screen"),
            Screens::Home => write!(f, "home"),
            Screens::Network => write!(f, "network"),
            Screens::ManageNetworks => write!(f, "manage_networks"),
            Screens::NetworkDetails => write!(f, "network_details"),
            Screens::ConnectNetwork => write!(f, "connect_network"),
            Screens::AddNetwork => write!(f, "add_network"),
            Screens::ManageBluetooth => write!(f, "manage_bluetooth"),
            Screens::BluetoothDetails => write!(f, "bluetooth_details"),
            Screens::ConnectBluetooth => write!(f, "connect_bluetooth"),
            Screens::BluetoothPairRequest => write!(f, "bluetooth_pair_request"),
            Screens::Display => write!(f, "display"),
            Screens::ScreenTimeout => write!(f, "screen_timeout"),
            Screens::Sound => write!(f, "sound"),
            Screens::PerformanceMode => write!(f, "performance_mode"),
            Screens::Settings => write!(f, "settings"),
            Screens::Security => write!(f, "security"),
            Screens::LockTimeout => write!(f, "lock_timeout"),
            Screens::Battery => write!(f, "battery"),
            Screens::ResetPin => write!(f, "reset_pin"),
            Screens::DateTime => write!(f, "date_time"),
            Screens::SetTime => write!(f, "set_time"), 
            Screens::SetDate => write!(f, "set_date"), 
            Screens::About => write!(f, "about")
        }
    }
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screens),
    Dummy,
}

struct AppWidgets {
    screens_stack: gtk::Stack,
}

// #[cfg(not(feature = "layer-shell"))]
fn init_window(settings: LockScreenSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .default_width(window_settings.size.0)
        .default_height(window_settings.size.1)
        .css_classes(["window"])
        .build();
    window
}

// #[cfg(feature = "layer-shell")]
// fn init_window(settings: LockScreenSettings) -> gtk::Window {
//     let window_settings = settings.window;
//     let window = gtk::Window::builder()
//         .title(settings.title)
//         .default_width(window_settings.size.0)
//         .default_height(window_settings.size.1)
//         .css_classes(["window"])
//         .build();

//     gtk4_layer_shell::init_for_window(&window);

//     // Display above normal windows
//     gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Top);

//     // The margins are the gaps around the window's edges
//     // Margins and anchors can be set like this...
//     gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Left, 0);
//     gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Right, 0);
//     gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Top, 0);
//     gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Bottom, 0);

//     gtk4_layer_shell::set_keyboard_mode(&window, gtk4_layer_shell::KeyboardMode::OnDemand);

//     // ... or like this
//     // Anchors are if the window is pinned to each edge of the output
//     let anchors = [
//         (gtk4_layer_shell::Edge::Left, true),
//         (gtk4_layer_shell::Edge::Right, true),
//         (gtk4_layer_shell::Edge::Top, true),
//         (gtk4_layer_shell::Edge::Bottom, true),
//     ];

//     for (anchor, state) in anchors {
//         gtk4_layer_shell::set_anchor(&window, anchor, state);
//     }

//     window
// }

impl SimpleComponent for LockScreen {
    /// The type of the messages that this component can receive.
    type Input = Message;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = ();
    /// The root GTK widget that this component will create.
    type Root = gtk::Window;
    /// A data structure that contains the widgets that you will need to update.
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => LockScreenSettings::default(),
        };

        info!(
            task = "initalize_settings",
            "settings initialized for Lock Screen: {:?}", settings
        );

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => LockScreenTheme::default(),
        };

        info!(
            task = "initalize_theme",
            "theme initialized for Lock Screen: {:?}", custom_theme
        );

        let window = init_window(settings);
        window
    }

    /// Initialize the UI and model.
    fn init(
        _: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let icon_theme = gtk::IconTheme::builder().build();
        info!("icon paths are {:?}", icon_theme.resource_path());
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => LockScreenSettings::default(),
        };

        let css = settings.css.clone();
        relm4::set_global_css_from_file(css.default);

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => LockScreenTheme::default(),
        };

        let modules = settings.modules.clone();
        let layout = settings.layout.clone();
        let widget_configs = settings.widget_configs.clone();

        //Stack used to render different screens
        //At a time one screen will be rendered
        let screens_stack = gtk::Stack::builder().build();

        let home_page = HomePage::builder()
            .launch(HomePageSettings {
                lock_icon: modules.lock.icon.default.to_owned(),
                unlock_icon: modules.unlock.icon.default.to_owned(),
                password_icon: modules.home_password.icon.default.to_owned(),
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| match msg {
                    HomePageMessage::ChangeScreen(screen) => Message::ChangeScreen(screen)
                }),
            );

        screens_stack.add_named(
            home_page.widget(),
            Option::from(Screens::Home.to_string().as_str()),
        );

        let settings_page = SettingsPage::builder()
            .launch(SettingsPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone(),
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("settings_page - auth page message to parent - settings page {:?}", msg);
                    match msg {
                        SettingsPageMessage::ChangeScreen(screen) => Message::ChangeScreen(screen)
                     }
                }),
            );

        screens_stack.add_named(
            settings_page.widget(),
            Option::from(Screens::Settings.to_string().as_str()),
        );

        let network_page = NetworksPage::builder()
            .launch(NetworksPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("network_page - auth page message to parent {:?}", msg);
                    match msg {
                        
                        NetworksPageMessage::BackPressed => Message::ChangeScreen(Screens::Settings),
                        NetworksPageMessage::EnableNetworkPressed => Message::ChangeScreen(Screens::NetworkDetails),
                        NetworksPageMessage::ManageNetworkPressed => Message::ChangeScreen(Screens::ManageNetworks),
                        NetworksPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            network_page.widget(),
            Option::from(Screens::Network.to_string().as_str()),
        );

        let manage_networks_page: Controller<ManageNetworksPage> = ManageNetworksPage::builder()
            .launch(ManageNetworksPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("manage_networks_page - auth page message to parent {:?}", msg);
                    match msg {
                        ManageNetworksPageMessage::BackPressed => Message::ChangeScreen(Screens::Network),
                        ManageNetworksPageMessage::KnownNetworkPressed => 
                        Message::ChangeScreen(Screens::NetworkDetails),
                        ManageNetworksPageMessage::AvailableNetworkPressed => 
                        Message::ChangeScreen(Screens::ConnectNetwork),
                        ManageNetworksPageMessage::AddNetworkPressed => Message::ChangeScreen(Screens::AddNetwork),
                        ManageNetworksPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            manage_networks_page.widget(),
            Option::from(Screens::ManageNetworks.to_string().as_str()),
        );

        let network_details_page: Controller<NetworkDetailsPage> = NetworkDetailsPage::builder()
            .launch(NetworkDetailsPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("network_details_page - auth page message to parent {:?}", msg);
                    match msg {
                        NetworkDetailsPageMessage::BackPressed =>
                        Message::ChangeScreen(Screens::ManageNetworks),
                        NetworkDetailsPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                            _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            network_details_page.widget(),
            Option::from(Screens::NetworkDetails.to_string().as_str()),
        );

        let connect_network_page: Controller<ConnectNetworkPage> = ConnectNetworkPage::builder()
            .launch(ConnectNetworkPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        ConnectNetworkPageMessage::BackPressed => Message::ChangeScreen(Screens::ManageNetworks),
                        ConnectNetworkPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            connect_network_page.widget(),
            Option::from(Screens::ConnectNetwork.to_string().as_str()),
        );

        let add_network_page: Controller<AddNetworkPage> = AddNetworkPage::builder()
            .launch(AddNetworkPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        AddNetworkPageMessage::BackPressed => Message::ChangeScreen(Screens::ManageNetworks),
                        AddNetworkPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                            _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            add_network_page.widget(),
            Option::from(Screens::AddNetwork.to_string().as_str()),
        );

        let manage_bluetooth_page: Controller<ManageBluetoothPage> = ManageBluetoothPage::builder()
            .launch(ManageBluetoothPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        ManageBluetoothPageMessage::BackPressed => Message::ChangeScreen(Screens::Settings),
                        ManageBluetoothPageMessage::AvaiableDevicePressed => Message::ChangeScreen(Screens::BluetoothDetails),
                        ManageBluetoothPageMessage::OtherDevicePressed => Message::ChangeScreen(Screens::ConnectBluetooth),
                        // ManageBluetoothPageMessage::OtherDevicePressed => Message::ChangeScreen(Screens::BluetoothPairRequest),
                        ManageBluetoothPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            manage_bluetooth_page.widget(),
            Option::from(Screens::ManageBluetooth.to_string().as_str()),
        );

        let bluetooth_details_page: Controller<BluetoothDetailsPage> = BluetoothDetailsPage::builder()
            .launch(BluetoothDetailsPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                       BluetoothDetailsPageMessage::BackPressed => Message::ChangeScreen(Screens::ManageBluetooth),
                       BluetoothDetailsPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            bluetooth_details_page.widget(),
            Option::from(Screens::BluetoothDetails.to_string().as_str()),
        );

        let connect_bluetooth_page: Controller<ConnectBluetoothPage> = ConnectBluetoothPage::builder()
            .launch(ConnectBluetoothPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                       ConnectBluetoothPageMessage::BackPressed => Message::ChangeScreen(Screens::ManageBluetooth),
                       ConnectBluetoothPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            connect_bluetooth_page.widget(),
            Option::from(Screens::ConnectBluetooth.to_string().as_str()),
        );

        let bluetooth_pair_request_page: Controller<BluetoothPairRequestPage> = BluetoothPairRequestPage::builder()
            .launch(BluetoothPairRequestPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                       BluetoothPairRequestPageMessage::BackPressed => Message::ChangeScreen(Screens::ManageBluetooth),
                       BluetoothPairRequestPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            bluetooth_pair_request_page.widget(),
            Option::from(Screens::BluetoothPairRequest.to_string().as_str()),
        );

        let display_page: Controller<DisplayPage> = DisplayPage::builder()
            .launch(DisplayPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        DisplayPageMessage::BackPressed => Message::ChangeScreen(Screens::Settings),
                        DisplayPageMessage::ScreenTimeoutOpted => Message::ChangeScreen(Screens::ScreenTimeout),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            display_page.widget(),
            Option::from(Screens::Display.to_string().as_str()),
        );

        let battery_page: Controller<BatteryPage> = BatteryPage::builder()
            .launch(BatteryPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("battery_page - auth page message to parent {:?}", msg);
                    match msg {
                        BatteryPageMessage::BackPressed => Message::ChangeScreen(Screens::Settings),
                        BatteryPageMessage::ScreenTimeoutOpted => Message::ChangeScreen(Screens::ScreenTimeout),
                        BatteryPageMessage::PerformanceOpted => Message::ChangeScreen(Screens::PerformanceMode),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            battery_page.widget(),
            Option::from(Screens::Battery.to_string().as_str()),
        );

        let screen_timeout_page: Controller<ScreenTimeoutPage> = ScreenTimeoutPage::builder()
            .launch(ScreenTimeoutPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        ScreenTimeoutPageMessage::BackPressed => Message::ChangeScreen(Screens::Display),
                        ScreenTimeoutPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            screen_timeout_page.widget(),
            Option::from(Screens::ScreenTimeout.to_string().as_str()),
        );

        let sound_page: Controller<SoundPage> = SoundPage::builder()
            .launch(SoundPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        SoundPageMessage::BackPressed => Message::ChangeScreen(Screens::Settings),
                        SoundPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                            _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            sound_page.widget(),
            Option::from(Screens::Sound.to_string().as_str()),
        );

        let performance_mode_page: Controller<PerformanceModePage> = PerformanceModePage::builder()
            .launch(PerformanceModePageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        PerformanceModePageMessage::BackPressed => Message::ChangeScreen(Screens::Battery),
                       PerformanceModePageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            performance_mode_page.widget(),
            Option::from(Screens::PerformanceMode.to_string().as_str()),
        );

        let security_page: Controller<SecurityPage> = SecurityPage::builder()
        .launch(SecurityPageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("security_page - auth page message to parent {:?}", msg);
                match msg {
                    SecurityPageMessage::BackPressed => Message::ChangeScreen(Screens::Settings),
                    SecurityPageMessage::LockTimeoutOpted => Message::ChangeScreen(Screens::LockTimeout),
                    SecurityPageMessage::ResetPinOpted => Message::ChangeScreen(Screens::ResetPin),
                    _ => Message::Dummy
                }
            }),
        );

        screens_stack.add_named(
            security_page.widget(),
            Option::from(Screens::Security.to_string().as_str()),
        );

        let lock_timeout_page: Controller<LockTimeoutPage> = LockTimeoutPage::builder()
            .launch(LockTimeoutPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                            LockTimeoutPageMessage::BackPressed => Message::ChangeScreen(Screens::Security),
                            LockTimeoutPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            lock_timeout_page.widget(),
            Option::from(Screens::LockTimeout.to_string().as_str()),
        );

        let reset_pin_page:Controller<ResetPinPage> = ResetPinPage::builder()
        .launch(ResetPinPageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("auth page message to parent {:?}", msg);
                match msg {
                   ResetPinPageMessage::BackPressed => Message::ChangeScreen(Screens::Security),
                   ResetPinPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                    _ => Message::Dummy
                }
            }),
        );

        screens_stack.add_named(
            reset_pin_page.widget(),
            Option::from(Screens::ResetPin.to_string().as_str()),
        );


        let date_time_page: Controller<DateTimePage> = DateTimePage::builder()
        .launch(DateTimePageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("date_time_page - auth page message to parent {:?}", msg);
                match msg {
                    DateTimePageMessage::BackPressed => Message::ChangeScreen(Screens::Settings),
                    DateTimePageMessage::SetTimeOpted => Message::ChangeScreen(Screens::SetTime),
                    DateTimePageMessage::SetDateOpted => Message::ChangeScreen(Screens::SetDate),
                    _ => Message::Dummy
                }
            }),
        );

        screens_stack.add_named(
            date_time_page.widget(),
            Option::from(Screens::DateTime.to_string().as_str()),
        );

        
        let set_time_page:Controller<SetTimePage> = SetTimePage::builder()
        .launch(SetTimePageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("auth page message to parent {:?}", msg);
                match msg {
                   SetTimePageMessage::BackPressed => Message::ChangeScreen(Screens::DateTime),
                   SetTimePageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                    _ => Message::Dummy
                }
            }),
        );
        screens_stack.add_named(
            set_time_page.widget(),
            Option::from(Screens::SetTime.to_string().as_str()),
        );


        let set_date_page:Controller<SetDatePage> = SetDatePage::builder()
        .launch(SetDatePageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("auth page message to parent {:?}", msg);
                match msg {
                   SetDatePageMessage::BackPressed => Message::ChangeScreen(Screens::DateTime),
                   SetDatePageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                    _ => Message::Dummy
                }
            }),
        );
        screens_stack.add_named(
            set_date_page.widget(),
            Option::from(Screens::SetDate.to_string().as_str()),
        );


        let about_page:Controller<AboutPage> = AboutPage::builder()
        .launch(AboutPageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("auth page message to parent {:?}", msg);
                match msg {
                   AboutPageMessage::BackPressed => Message::ChangeScreen(Screens::Settings),
                    _ => Message::Dummy
                }
            }),
        );
        screens_stack.add_named(
            about_page.widget(),
            Option::from(Screens::About.to_string().as_str()),
        );


        let current_screen = Screens::Settings;

        //Setting current active screen in stack
        screens_stack.set_visible_child_name(&current_screen.to_string());

        //Adding stack to window
        window.set_child(Some(&screens_stack));

        let model = LockScreen {
            settings,
            custom_theme,
            current_screen,
            home_page,
            settings_page,
            network_page,
            manage_networks_page,
            network_details_page,
            connect_network_page,
            add_network_page,
            manage_bluetooth_page,
            bluetooth_details_page,
            connect_bluetooth_page,
            bluetooth_pair_request_page,
            display_page,
            screen_timeout_page,
            sound_page,
            performance_mode_page,
            security_page,
            lock_timeout_page,
            battery_page,
            reset_pin_page,
            date_time_page,
            set_time_page,
            set_date_page,
            about_page
        };

        let widgets = AppWidgets { screens_stack };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::ChangeScreen(screen) => {
                self.current_screen = screen;
            }
            _ => (),
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        //updating stack screen when current screen changes
        widgets
            .screens_stack
            .set_visible_child_name(self.current_screen.to_string().as_str());
    }
}

fn main() {
    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("mecha_settings_app=trace")
        .with_thread_names(true)
        .init();
    let app = RelmApp::new("lock.screen").with_args(vec![]);
    app.run::<LockScreen>(());
}
