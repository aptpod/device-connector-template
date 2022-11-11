use device_connector::{
    ElementBuildable, ElementResult, ElementValue, Error, MsgReceiver, MsgType, Pipeline, Port,
};
use serde_derive::Deserialize;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

pub struct HelloSrcElement {
    conf: HelloSrcElementConf,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HelloSrcElementConf {
    text: String,
}

impl ElementBuildable for HelloSrcElement {
    type Config = HelloSrcElementConf;

    const NAME: &'static str = "hello-src";

    const SEND_PORTS: Port = 1;

    fn new(conf: Self::Config) -> Result<Self, Error> {
        Ok(HelloSrcElement { conf })
    }

    fn next(&mut self, pipeline: &mut Pipeline, _receiver: &mut MsgReceiver) -> ElementResult {
        pipeline.check_send_msg_type(0, || MsgType::from_mime("text/plain").unwrap())?;

        sleep(Duration::from_millis(100));

        let mut buf = pipeline.msg_buf(0);
        buf.write_all(self.conf.text.as_bytes())?;

        Ok(ElementValue::MsgBuf)
    }
}
