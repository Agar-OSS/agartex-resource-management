use crate::constants;
use headers::{Header, HeaderName, HeaderValue};
pub struct XUserId(pub i32);

impl Header for XUserId {
    fn name() -> &'static HeaderName {
        &constants::XUSERID_HEADER_NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        if let Ok(s) = values.next().ok_or_else(headers::Error::invalid)?.to_str() {
            if let Ok(num) = s.parse::<i32>() {
                return Ok(XUserId(num));
            }
        }
        Err(headers::Error::invalid())
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        let value = HeaderValue::from(self.0);

        values.extend(std::iter::once(value));
    }
}
