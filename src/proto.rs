use message::{ChannelInfo, IotMessage};

pub mod message {
    include!(concat!(env!("OUT_DIR"), "/com.circue.iot.proto.rs"));
}

pub fn create_iot_message(
    server_time: Option<i64>,
    client_ip: String,
    client_port: u32,
    server_ip: String,
    server_port: u32,
    protocol: Option<String>,
    message: Vec<u8>,
    message_type: Option<String>,
) -> IotMessage {
    let mut channel_info = ChannelInfo::default();
    channel_info.client_ip = client_ip;
    channel_info.client_port = client_port;
    channel_info.server_ip = server_ip;
    channel_info.server_port = server_port;
    channel_info.protocol = protocol;

    let mut iot_message = IotMessage::default();
    iot_message.channel = channel_info;
    iot_message.message_type = message_type;
    iot_message.message = message;
    iot_message.server_time = server_time;
    iot_message
}