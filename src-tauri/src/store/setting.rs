use serde::{Deserialize,Serialize};
use once_cell::sync::Lazy;
use std::path::Path;
use std::sync::RwLock;
use std::fs::write;
const DEFAULT_CONFIG: &str = "setting"; // "Setting.yaml";

static CONFIG: Lazy<RwLock<Configuration>> = Lazy::new(|| {
    let default_config: Configuration = Configuration {
        api: API {
            url: "".into(),
            key: "".into(),
            model: "".into(),
        },
        hotkey: HotKey {
            dialog: "".into(),
        },
    };
    RwLock::new(default_config)
});

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct  API {
    pub url: String,
    pub key: String,
    pub model :String
}
#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct HotKey{
    pub dialog: String,
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct Configuration {
    pub api: API,
    pub hotkey: HotKey,
}


pub fn get(key: &str)->Option<String>{
    let config_content = CONFIG.read().unwrap().clone();
    
    match key {
        "api.url" => Some(config_content.api.url.clone()),
        "api.key" => Some(config_content.api.key.clone()),
        "api.model" => Some(config_content.api.model.clone()),
        "hotkey.dialog" => Some(config_content.hotkey.dialog.clone()),
        _ => None
    }
}

pub fn set(key: &str,value: &str)-> Result<(), Box<dyn std::error::Error>> {
    Configuration::update_config(|config| {
        match key {
            "api.url" => config.api.url = value.to_string(),
            "api.key" => config.api.key = value.to_string(),
            "api.model" => config.api.model = value.to_string(),
            "hotkey.dialog" => config.hotkey.dialog = value.to_string(),
            _ => {}
        }
    })
}

impl Configuration {
    fn default() -> Self {
        Configuration {
            api: API {
                url: "https://api.example.com".into(),
                key: "your-api-key".into(),
                model: "default-model".into(),
            },
            hotkey: HotKey {
                dialog: "".into(),
            },
        }
    }
    // 初始化配置（从文件加载）
    pub fn init_config() -> Result<Configuration, Box<dyn std::error::Error>> {
        let config_path = format!("{}.yaml", DEFAULT_CONFIG);
        
        // 检查配置文件是否存在
        if !Path::new(&config_path).exists() {
            // 创建默认配置
            let default_config = Configuration::default();
            let yaml_content = serde_yaml::to_string(&default_config)?;
            write(&config_path, yaml_content)?;
            println!("Created default config file");
        }

        let config_content = std::fs::read_to_string(&config_path)?;
        let new_config: Configuration = match serde_yaml::from_str(&config_content){
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to parse config file: {}", e);
                let default_config = Configuration::default();
                let yaml_content = serde_yaml::to_string(&default_config)?;
                write(&config_path, yaml_content).unwrap();
                println!("Created default config file");
                default_config
            }
        };
        let mut config = CONFIG.write().expect("set config lock error");
        *config = new_config.clone();
        Ok(new_config)


    }
    pub fn get_config() -> Self {
        CONFIG.read().unwrap().clone()
    }

    pub fn update_config<F>(updater: F) ->Result<(),Box<dyn std::error::Error>>
        where F: FnOnce(&mut Configuration) {

        let old_config = CONFIG.read().unwrap().clone();
        let mut config = CONFIG.write().unwrap();
        updater(&mut config);
        match serde_yaml::to_string(&*config)
            .map_err(|e| -> Box<dyn std::error::Error> { e.into() })
            .and_then(|data| {
                write(format!("{}.yaml", DEFAULT_CONFIG),data)
                .map_err(|e:std::io::Error|e.into())
            }){
                Ok(_) => return Ok(()),
                Err(e) => {
                    *config = old_config;
                    eprintln!("Failed to save config: {}", e);
                    return Err(e);
            }
        }
    }
}

