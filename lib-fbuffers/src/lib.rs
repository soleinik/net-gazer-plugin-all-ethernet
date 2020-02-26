#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(clippy::redundant_field_names)]
mod allipv4_generated;
use allipv4_generated::*;

use flatbuffers::FlatBufferBuilder;


pub struct Builder<'fbb>{
    seq: u64,
    bldr: FlatBufferBuilder<'fbb>,
}

impl<'a> Default for Builder<'a> {
    #[inline]
    fn default() -> Self {
        Builder { seq:0, bldr: FlatBufferBuilder::new()}
    }
}

impl Builder<'_> {

    fn reset(&mut self){
        self.bldr.reset();
        self.seq += 1; //FIXME: overflow
    }

    pub fn build_message(&mut self, proto:&str, src:u32, dst:u32, len:u16, id:u16, flags:u16, opts:&[u8]) -> Vec<u8>{
        let mut msg = Vec::<u8>::new();
        self.reset();

        
        let packet_args = PacketArgs{
            proto: Some(self.bldr.create_string(proto)),
            src,
            dst,
            len,
            id,
            flags,
            opts:Some(self.bldr.create_vector_direct(opts))
        };

        let packet = Packet::create(& mut self.bldr, &packet_args);

        let args = MessageArgs{
            seq:self.seq,
            packets:Some(self.bldr.create_vector(&[packet]))
        };
        
        let message_offset = Message::create(&mut self.bldr, &args);

        finish_message_buffer(&mut self.bldr, message_offset);
        let finished_data = self.bldr.finished_data();
        msg.extend_from_slice(finished_data);
        msg
    }
}



