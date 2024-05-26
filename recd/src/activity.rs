use std::fmt;
use std::time::Duration;

use regex::Regex;

use serde_json::Value;

#[derive(Clone, Debug)]
pub struct Activity {
    pub url: String,
    pub start_time: f32,
    pub end_time: f32,
    pub duration: Duration,
    pub label: String,
    pub activity_type: ActivityType,
}

#[derive(Clone, Copy, Debug)]
pub enum ActivityType {
    Networking(NetDetail),
    Loading,
    Scripting,
}

#[derive(Clone, Copy, Debug)]
pub struct NetDetail {
    pub status: Status,
    pub mime_type: MimeType,
}

#[derive(Clone, Copy, Debug)]
pub struct Status {
    pub size: Option<usize>,
    pub code: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
pub enum MimeType {
    Html,
    JavaScript,
    Css,
    Image,
    Font,
    Unknown,
}

impl Activity {
    pub fn new(activity: &Value) -> Option<Activity> {
        if let Some(v) = activity.get("activityId") {
            let mut url = activity
                .get("url")
                .expect("Cannot found `url`.")
                .to_string();

            // Remove ""
            url.remove(url.len() - 1);
            url.remove(0);

            let start_time = activity
                .get("startTime")
                .expect("Cannot found `startTime`.")
                .as_f64()
                .unwrap() as f32;
            let end_time = activity
                .get("endTime")
                .expect("Cannot found `endTime`.")
                .as_f64()
                .unwrap() as f32;
            let duration = Duration::from_micros(((end_time - start_time) * 1000.0) as u64);

            let label = v.as_str().expect("activityId is not a string.").to_string();
            lazy_static! {
                static ref RE: Regex = Regex::new(r#"^(\w+)_(\d+)$"#).unwrap();
            }
            let category = RE
                .captures(&label)
                .unwrap()
                .get(1)
                .map_or("", |m| m.as_str());
            match category {
                "Networking" => {
                    let transfer_size;
                    if let Some(c) = activity.get("transferSize") {
                        transfer_size = Some(c.as_u64().unwrap() as usize);
                    } else {
                        transfer_size = None;
                    }

                    let status_code;
                    if let Some(c) = activity.get("statusCode") {
                        status_code = Some(c.as_u64().unwrap() as usize);
                    } else {
                        status_code = None;
                    }
                        
                    let status = Status {
                        size: transfer_size,
                        code: status_code,
                    };

                    let mime_type;
                    if let Some(c) = activity.get("mimeType") {
                        let mut mime_type_raw = c.to_string();
                        mime_type_raw.remove(mime_type_raw.len() - 1);
                        mime_type_raw.remove(0);
                        if mime_type_raw.starts_with("image") {
                            mime_type = MimeType::Image;
                        } else if mime_type_raw.starts_with("font") {
                            mime_type = MimeType::Font;
                        } else if mime_type_raw.starts_with("text") {
                            mime_type = match mime_type_raw.as_str() {
                                "text/html" => MimeType::Html,
                                "text/javascript" => MimeType::JavaScript,
                                "text/css" => MimeType::Css,
                                _ => MimeType::Unknown,
                            }
                        } else if mime_type_raw.starts_with("application") {
                            mime_type = match mime_type_raw.as_str() {
                                "application/javascript" |
                                "application/x-javascript" => MimeType::JavaScript,
                                _ => MimeType::Unknown,
                            }
                        } else {
                            mime_type = MimeType::Unknown
                        }
                    } else {
                        mime_type = MimeType::Unknown;
                    }

                    let net_detail = NetDetail{ status, mime_type };

                    Some(Activity {
                        url,
                        start_time,
                        end_time,
                        duration,
                        label,
                        activity_type: ActivityType::Networking(net_detail),
                    })
                }
                "Scripting" => Some(Activity {
                    url,
                    start_time,
                    end_time,
                    duration,
                    label,
                    activity_type: ActivityType::Scripting,
                }),
                "Loading" => Some(Activity {
                    url,
                    start_time,
                    end_time,
                    duration,
                    label,
                    activity_type: ActivityType::Scripting,
                }),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn halve(&mut self, time: f32) {
        self.end_time = time;
        self.duration = Duration::from_micros(((self.end_time - self.start_time) * 1000.0) as u64);
    }

    pub fn duplicate(&self, time: f32) -> Activity {
        let duration = Duration::from_micros(((self.end_time - time) * 1000.0) as u64);
        Activity {
            url: self.url.clone(),
            start_time: time,
            end_time: self.end_time,
            duration,
            label: self.label.clone(),
            activity_type: self.activity_type,
        }
    }

    pub fn halve_and_duplicate(&mut self, time: f32) -> Activity {
        let duration = Duration::from_micros(((self.end_time - time) * 1000.0) as u64);
        let latter = Activity {
            url: self.url.clone(),
            start_time: time,
            end_time: self.end_time,
            duration,
            label: self.label.clone(),
            activity_type: self.activity_type,
        };

        self.end_time = time;
        self.duration = Duration::from_micros(((self.end_time - self.start_time) * 1000.0) as u64);

        latter
    }
}

impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MimeType::Html => write!(f, "Html"),
            MimeType::JavaScript => write!(f, "JavaScript"),
            MimeType::Css => write!(f, "Css"),
            MimeType::Image => write!(f, "Image"),
            MimeType::Font => write!(f, "Font"),
            MimeType::Unknown => write!(f, "Unknown"),
        }
    }
}