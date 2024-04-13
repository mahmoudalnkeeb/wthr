use ureq;

pub fn json_req<T>(url: &str) -> Result<T, ureq::Error>
where
    T: serde::de::DeserializeOwned,
{
    let response: T = ureq::get(&url).call()?.into_json()?;
    Ok(response)
}

pub fn text_req(url: &str) -> Result<String, ureq::Error> {
    let response: String = ureq::get(&url).call()?.into_string()?;
    Ok(response)
}
