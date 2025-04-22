use dc_core::{
    ElementBuildable, ElementResult, ElementValue, Error, MsgReceiver, MsgType, Pipeline, Port,
};
use serde::Deserialize;
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

    fn send_msg_types() -> Vec<Vec<MsgType>> {
        vec![vec![MsgType::from_mime("text/plain").unwrap()]]
    }

    fn new(conf: Self::Config) -> Result<Self, Error> {
        Ok(HelloSrcElement { conf })
    }

    fn next(&mut self, pipeline: &mut Pipeline, _receiver: &mut MsgReceiver) -> ElementResult {
        sleep(Duration::from_millis(100));

        let mut buf = pipeline.msg_buf(0);
        buf.write_all(self.conf.text.as_bytes())?;

        Ok(ElementValue::MsgBuf)
    }
}
