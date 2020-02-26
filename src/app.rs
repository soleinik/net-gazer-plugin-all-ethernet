extern crate net_gazer_core as core;


use core::{CoreSender, PLUGIN_ID_ALLIPV4};

use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use std::sync::Mutex;

pub struct App{
    data_sender:CoreSender,
    bldr:Mutex<lib_fbuffers::Builder<'static>>,
    conf:super::conf::Conf,
}

impl <'a> App{
    pub fn new(tx:CoreSender, conf:super::conf::Conf) -> Self{
            App{
                data_sender: tx, 
                bldr:Mutex::new(lib_fbuffers::Builder::default()),
                conf
            }
    }

    pub fn process(&self, pkt: &EthernetPacket) {
        // let mac_dst = pkt.get_destination();
        // let mac_src = pkt.get_source();

        let ether_type = pkt.get_ethertype();
        if let  EtherTypes::Ipv4 = ether_type{

            if let Some(ip4pkt) = Ipv4Packet::new(pkt.payload()) {
                let proto = ip4pkt.get_next_level_protocol();    //string
                if self.conf.ignored_proto.contains(&proto.to_string().to_uppercase()){
                    return;
                }
                let src = ip4pkt.get_source();                  //
                let dst = ip4pkt.get_destination();             //

                let pkt_len = ip4pkt.get_total_length();        //u16
                let pkt_id = ip4pkt.get_identification();       //u16
                let pkt_flags = 
                    if proto == pnet::packet::ip::IpNextHeaderProtocols::Tcp{
                        if let Some(tcp_pkt) = pnet::packet::tcp::TcpPacket::new(ip4pkt.payload()){
                            tcp_pkt.get_flags()
                        }else{
                            ip4pkt.get_flags() as u16            //u8
                        }
                    }else{
                        ip4pkt.get_flags() as u16            //u8
                    };

                let pkt_opts = ip4pkt.get_options_raw();        //&[u8]

                let msg = self.bldr.lock().unwrap().
                    build_message(&proto.to_string(), src.into(), dst.into(), pkt_len, pkt_id, pkt_flags, pkt_opts);
                    
                self.data_sender.send((PLUGIN_ID_ALLIPV4, msg)).unwrap();
            }

        }

    }
}