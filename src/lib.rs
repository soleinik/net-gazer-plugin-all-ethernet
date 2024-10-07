#[macro_use] extern crate log;
extern crate net_gazer_core as core;

use core::*;
use pnet::packet::ethernet::EthernetPacket;
use pnet::datalink::NetworkInterface;

mod app;
mod conf;

const ID:u8=core::PLUGIN_ID_ALLIPV4;
const NAME:&str="All ipv4";

#[derive(Default)]
pub struct AllEtherPlugin{
    app:Option<app::App>,
}

impl Plugin for AllEtherPlugin{

    fn get_name(&self)->&str{NAME}

    fn get_id(&self) -> u8 {ID}

    fn on_load(&mut self, _iface:&NetworkInterface, tx:CoreSender){
        env_logger::init();

        let mut conf = conf::Conf::default();
        conf.load( env!("CARGO_PKG_NAME"));

        self.app = Some(app::App::new(tx, conf));
        info!("Hello from \"{}\"(message_id:{}), ! ", NAME, ID);
    }

    fn on_unload(&mut self){
        info!("Good bye from \"{}\"(message_id:{})! ", NAME, ID);
    }

    fn process(&self, pkt:&EthernetPacket){
        self.app.as_ref().unwrap().process(pkt);
    }
}

#[no_mangle]
pub extern "C" fn net_gazer_plugin_new () -> * mut AllEtherPlugin{
     let boxed:Box<AllEtherPlugin> = Box::new(AllEtherPlugin::default());
     Box::into_raw(boxed)
}



