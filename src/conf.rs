use std::path::Path;
use config;

const APP_NAME:&str="net-gazer";

const KEY_IGNORED_PROTOS:&str="protocols.ignore";

#[derive(Default)]
pub struct Conf {
    pub ignored_proto: Vec<String>,
}


impl Conf{

    pub fn load(&mut self, lib_name: &str){

        let current_dir = std::env::current_dir().unwrap();
        let current_dir = current_dir.to_str().unwrap();

        let cfg_file_name = format!("{}.toml", lib_name);

        let paths = vec![
            format!("{}/etc/{}/{}",current_dir, APP_NAME, cfg_file_name), 
            //user home?
            format!("/usr/local/etc/{}/{}", APP_NAME, cfg_file_name), 
            format!("/etc/{}/{}", APP_NAME, cfg_file_name)
        ];

        let path = paths.iter()
            .map(|n| Path::new(n))
            .filter(|p| p.exists())
            .find(|p|p.is_file())
            .map( |p|p.to_str().unwrap().to_owned());



        if let Some(cfg_file) = path{

            let cfg_file = Path::new(&cfg_file).canonicalize().unwrap();
            let cfg_file = cfg_file.to_str().unwrap();
            info!("Loading configuration from {}...", cfg_file);
            let mut settings = config::Config::default();
            settings.merge(config::File::with_name(cfg_file)).unwrap();

            if let Ok(ignored_protos) = settings.get_array(KEY_IGNORED_PROTOS){
                ignored_protos.into_iter().for_each(|v|{
                    if let Ok(s) = v.into_str(){
                        self.ignored_proto.push(s.to_uppercase());
                    }
                });
            }
        }
    }

}
