use device_connector::EmptyElementConf;
use device_connector::{
    ElementBuildable, ElementResult, Error, MsgReceiver, MsgType, Pipeline, Port,
};

pub struct HexdumpSinkElement {}

impl ElementBuildable for HexdumpSinkElement {
    type Config = EmptyElementConf;

    const NAME: &'static str = "hexdump-sink";
    const RECV_PORTS: Port = 1;
    const SEND_PORTS: Port = 0;

    fn acceptable_msg_types() -> Vec<Vec<MsgType>> {
        vec![vec![MsgType::any()]]
    }

    fn new(_conf: Self::Config) -> Result<Self, Error> {
        Ok(Self {})
    }

    fn next(&mut self, _pipeline: &mut Pipeline, receiver: &mut MsgReceiver) -> ElementResult {
        loop {
            let msg = receiver.recv(0)?;
            let bytes = msg.as_bytes();
            eprintln!(
                "msg ={}",
                bytes
                    .iter()
                    .map(|x| format!(" {:02X}", x))
                    .collect::<String>()
            );
        }
    }
}
