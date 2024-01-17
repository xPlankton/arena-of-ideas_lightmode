use std::{sync::Mutex, thread::sleep};

use spacetimedb_sdk::{
    identity::{load_credentials, save_credentials},
    once_on_subscription_applied,
    reducer::Status,
    subscribe,
};

use crate::module_bindings::{
    login, login_by_identity, once_on_login, once_on_login_by_identity, once_on_register,
    once_on_register_empty, register, register_empty, GlobalData, User,
};

use super::*;

pub struct LoginPlugin;

const SPACETIMEDB_URI: &str = "http://localhost:3001";
// const SPACETIMEDB_URI: &str = "http://178.62.220.183:3000";
#[cfg(debug_assertions)]
const DB_NAME: &str = "aoi_dev";
#[cfg(not(debug_assertions))]
const DB_NAME: &str = "aoi";
const CREDS_DIR: &str = ".aoi";

static IS_CONNECTED: Mutex<bool> = Mutex::new(false);
pub static CURRENT_USER: Mutex<Option<UserData>> = Mutex::new(None);

#[derive(Clone)]
pub struct UserData {
    pub name: String,
    pub id: u64,
    pub identity: Identity,
}

fn on_connected(creds: &Credentials, _client_address: Address) {
    *IS_CONNECTED.lock().unwrap() = true;
    debug!("Current identity: {}", hex::encode(creds.identity.bytes()));
    if let Err(e) = save_credentials(CREDS_DIR, creds) {
        eprintln!("Failed to save credentials: {:?}", e);
    }
    subscribe(&["select * from User", "select * from GlobalData"]).unwrap();
    let creds = creds.clone();
    once_on_subscription_applied(move || {
        if !VERSION.eq(&GlobalData::filter_by_always_zero(0).unwrap().game_version) {
            AlertPlugin::add_error(
                Some("GAME VERSION ERROR".to_owned()),
                "Game version is too old".to_owned(),
                Some(Box::new(|w| {
                    egui_context(w).open_url(egui::OpenUrl {
                        url: "https://makscee.itch.io/arena-of-ideas".to_owned(),
                        new_tab: true,
                    });
                    w.send_event(AppExit);
                })),
            );
            return;
        }

        if User::find(|u| u.identities.contains(&creds.identity)).is_some() {
            LoginPlugin::login_by_identity();
        } else {
            register_empty();
            once_on_register_empty(|_, _, status| {
                debug!("Register empty: {status:?}");
                match status {
                    Status::Committed => LoginPlugin::login_by_identity(),
                    Status::Failed(e) => AlertPlugin::add_error(
                        Some("REGISTER ERROR".to_owned()),
                        e.to_owned(),
                        None,
                    ),
                    _ => panic!(),
                }
            });
        }
    });
}

#[derive(Resource, Default)]
pub struct LoginData {
    pub name: String,
    pub pass: String,
    pub login_sent: bool,
}
#[derive(Resource, Default)]
pub struct RegisterData {
    pub name: String,
    pub pass: String,
    pub pass_repeat: String,
}

#[derive(BevyEvent)]
pub struct LoginEvent;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup)
            .init_resource::<LoginData>()
            .init_resource::<RegisterData>()
            .add_event::<LoginEvent>();
    }
}

impl LoginPlugin {
    fn load_credentials() -> Option<Credentials> {
        load_credentials(CREDS_DIR).expect("Failed to load credentials")
    }

    pub fn is_connected() -> bool {
        *IS_CONNECTED.lock().unwrap()
    }
    pub fn get_user_data() -> Option<UserData> {
        CURRENT_USER.lock().unwrap().clone()
    }

    fn setup() {
        once_on_connect(on_connected);
        Self::connect();
    }

    pub fn connect() {
        if Self::is_connected() {
            return;
        }
        let creds = Self::load_credentials();
        let mut tries = 5;
        while let Err(e) = connect(SPACETIMEDB_URI, DB_NAME, creds.clone()) {
            error!("Connection error: {e}");
            sleep(Duration::from_secs(1));
            tries -= 1;
            if tries <= 0 {
                return;
            }
        }
    }

    pub fn clear_saved_credentials() {
        let mut path = home::home_dir().expect("Failed to get home dir");
        path.push(CREDS_DIR);
        std::fs::remove_dir_all(path).expect("Failed to clear credentials dir");
    }

    pub fn save_current_user(name: String, id: u64, identity: Identity) {
        *CURRENT_USER.lock().unwrap() = Some(UserData { name, id, identity });
        subscribe_to_tables(id);
    }

    fn on_login(status: &Status, identity: &Identity) {
        debug!("Login: {status:?} {identity:?}");
        match status {
            Status::Committed => {
                let user = User::find(|u| u.identities.contains(identity)).unwrap();
                Self::save_current_user(user.name, user.id, identity.clone());
            }
            Status::Failed(e) => {
                AlertPlugin::add_error(
                    Some("LOGIN ERROR".to_owned()),
                    format!("Failed to login {e}"),
                    None,
                );
            }
            _ => panic!(),
        }
    }

    fn login_by_password(name: String, pass: String) {
        login(name, pass);
        once_on_login(|identity, _, status, _, _| Self::on_login(status, identity));
    }

    fn login_by_identity() {
        debug!("Login by identity");
        login_by_identity();
        once_on_login_by_identity(|identity, _, status| Self::on_login(status, identity));
    }

    pub fn login(ui: &mut Ui, world: &mut World) {
        let mut login_data = world.resource_mut::<LoginData>();

        frame(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("name:");
                    ui.label("password:");
                });
                ui.vertical(|ui| {
                    TextEdit::singleline(&mut login_data.name)
                        .desired_width(ui.available_width())
                        .margin(egui::Vec2::ZERO)
                        .ui(ui);
                    TextEdit::singleline(&mut login_data.pass)
                        .password(true)
                        .desired_width(ui.available_width())
                        .margin(egui::Vec2::ZERO)
                        .ui(ui);
                });
            });
            ui.set_enabled(!login_data.name.is_empty() && !login_data.pass.is_empty());
            if ui.button("LOGIN").clicked() {
                Self::login_by_password(login_data.name.clone(), login_data.pass.clone());
            }
        });
    }

    pub fn register(ui: &mut Ui, world: &mut World) {
        frame(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.vertical_centered_justified(|ui| {
                let mut register_data = world.resource_mut::<RegisterData>();
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("name:");
                        ui.label("password:");
                        ui.label("repeat:");
                    });
                    ui.vertical(|ui| {
                        TextEdit::singleline(&mut register_data.name)
                            .desired_width(ui.available_width())
                            .margin(egui::Vec2::ZERO)
                            .ui(ui);
                        TextEdit::singleline(&mut register_data.pass)
                            .password(true)
                            .desired_width(ui.available_width())
                            .margin(egui::Vec2::ZERO)
                            .ui(ui);
                        TextEdit::singleline(&mut register_data.pass_repeat)
                            .password(true)
                            .desired_width(ui.available_width())
                            .margin(egui::Vec2::ZERO)
                            .ui(ui);
                    });
                });
                ui.set_enabled(
                    !register_data.name.is_empty()
                        && !register_data.pass.is_empty()
                        && register_data.pass.eq(&register_data.pass_repeat),
                );
                if ui.button("REGISTER").clicked() {
                    debug!(
                        "Register start: {} {}",
                        register_data.name, register_data.pass
                    );
                    register(register_data.name.clone(), register_data.pass.clone());
                    once_on_register(|_, _, status, name, pass| {
                        debug!("Register: {status:?} {name}");
                        match status {
                            Status::Committed => {
                                Self::login_by_password(name.to_owned(), pass.to_owned())
                            }
                            Status::Failed(e) => AlertPlugin::add_error(
                                Some("REGISTER ERROR".to_owned()),
                                e.to_owned(),
                                None,
                            ),
                            _ => panic!(),
                        }
                    });
                    set_context_bool(world, "register", false);
                }
            })
        });
    }
}

fn subscribe_to_tables(user_id: u64) {
    debug!("Subscribe to tables, user_id = {user_id}");
    match subscribe(&[
        "select * from User",
        "select * from GlobalData",
        "select * from TableUnit",
        "select * from House",
        "select * from Statuses",
        "select * from Ability",
        "select * from Vfx",
        &format!("select * from ArenaRun where user_id = {user_id}"),
    ]) {
        Ok(_) => debug!("Subscribe successful"),
        Err(e) => error!("Subscription error: {e}"),
    }
}
