use crate::error::Error;
use crate::Result;
use crate::{error::Kind, network};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    result: i32,
    ss5: String,
    ss6: String,
    ss4: String,
    time: i32,
    flow: f64,
    uid: String,
}

pub fn logout() -> Result<Response> {
    const NODE: &str = "/drcom/logout";
    const CALLBACK: &str = env!("CARGO_PKG_NAME");

    let mut url = url::Url::parse(network::ENTRANCE)
        .unwrap()
        .join(NODE)
        .unwrap();
    let url = url
        .query_pairs_mut()
        .append_pair("callback", CALLBACK)
        .finish();

    reqwest::blocking::get(url.as_str()).map_or(Err(Error::new(Kind::Request)), |res| {
        if res.status() != 200 {
            Err(Error::with_detail(
                Kind::Request,
                Some(res.status().as_u16()),
                res.text().ok(),
            ))
        } else {
            let template = format!(r"{CALLBACK}\({{}}\)");
            let source = res.text().map_err(|_| Error::new(Kind::Request))?;
            let json = network::util::fuck_cnu_api(&source, &template);

            serde_json::from_str(json).map_err(|_| Error::new(Kind::Parse))
        }
    })
}
