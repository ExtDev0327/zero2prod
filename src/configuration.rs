use actix_web::web::Data;
use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut builder = Config::builder()
        .set_default("default", "1")?
        .add_source(File::new("configuration", FileFormat::Yaml))
        .set_override("override", "1")?;

    // match builder.build() {
    //     Ok(config) => {
    //         config.try_deserialize().unwrap()
    //         // println!("{:?}", config);
    //     }
    //     Err(e) => {
    //         // println!("{:?}", e);
    //     }
    // }
    // Ok(())
    builder.build()?.try_deserialize()
}