use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    Get,
    Subscribe,
    Broadcast,
    Options,
}

#[derive(Debug, Serialize, Deserialize)]
struct Header {
    #[serde(rename = "msgId")]
    id: String,
    #[serde(rename = "verb")]
    method: Method,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(flatten)]
    header: Header,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<Payload>,
}

impl Message {
    pub fn with<S>(method: Method, path: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            header: Header {
                id: "".to_owned(),
                method,
                path: path.into(),
            },
            payload: None,
        }
    }

    pub fn payload(&self) -> &Option<Payload> {
        &self.payload
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
    Devices(DevicePayload),
    Battery(BatteryPayload),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevicePayload {
    #[serde(rename = "deviceInfos")]
    pub devices: Vec<Device>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryPayload {
    percentage: u8,
    charging: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}
