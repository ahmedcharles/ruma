//! Matrix-spec compliant server names.
use std::{convert::TryFrom, fmt, mem, rc::Rc, str::FromStr, sync::Arc};

use ruma_identifiers_validation::server_name::validate;

/// A Matrix-spec compliant server name.
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent, crate = "serde"))]
pub struct ServerName(str);

/// An owned server name.
pub type ServerNameBox = Box<ServerName>;

impl ServerName {
    fn from_borrowed(s: &str) -> &Self {
        unsafe { mem::transmute(s) }
    }

    fn from_owned(s: Box<str>) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(s) as _) }
    }

    fn into_owned(self: Box<Self>) -> Box<str> {
        unsafe { Box::from_raw(Box::into_raw(self) as _) }
    }

    /// Creates a string slice from this `ServerName`.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Creates a byte slice from this `ServerName`.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl fmt::Debug for ServerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Clone for Box<ServerName> {
    fn clone(&self) -> Self {
        (**self).to_owned()
    }
}

impl ToOwned for ServerName {
    type Owned = Box<ServerName>;

    fn to_owned(&self) -> Self::Owned {
        Self::from_owned(self.0.into())
    }
}

impl From<&ServerName> for Box<ServerName> {
    fn from(s: &ServerName) -> Self {
        s.to_owned()
    }
}

impl From<&ServerName> for Rc<ServerName> {
    fn from(s: &ServerName) -> Self {
        let rc = Rc::<str>::from(s.as_str());
        unsafe { Rc::from_raw(Rc::into_raw(rc) as *const ServerName) }
    }
}

impl From<&ServerName> for Arc<ServerName> {
    fn from(s: &ServerName) -> Self {
        let arc = Arc::<str>::from(s.as_str());
        unsafe { Arc::from_raw(Arc::into_raw(arc) as *const ServerName) }
    }
}

fn try_from<S>(server_name: S) -> Result<Box<ServerName>, crate::Error>
where
    S: AsRef<str> + Into<Box<str>>,
{
    validate(server_name.as_ref())?;
    Ok(ServerName::from_owned(server_name.into()))
}

impl AsRef<str> for ServerName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for Box<ServerName> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<Box<ServerName>> for String {
    fn from(s: Box<ServerName>) -> Self {
        s.into_owned().into()
    }
}

impl<'a> TryFrom<&'a str> for &'a ServerName {
    type Error = crate::Error;

    fn try_from(server_name: &'a str) -> Result<Self, Self::Error> {
        validate(server_name)?;
        Ok(ServerName::from_borrowed(server_name))
    }
}

impl FromStr for Box<ServerName> {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        try_from(s)
    }
}

impl TryFrom<&str> for Box<ServerName> {
    type Error = crate::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        try_from(s)
    }
}

impl TryFrom<String> for Box<ServerName> {
    type Error = crate::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        try_from(s)
    }
}

impl fmt::Display for ServerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Box<ServerName> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        crate::deserialize_id(deserializer, "An IP address or hostname")
    }
}

partial_eq_string!(ServerName);
partial_eq_string!(Box<ServerName>);

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::ServerName;

    #[test]
    fn ipv4_host() {
        assert!(<&ServerName>::try_from("127.0.0.1").is_ok());
    }

    #[test]
    fn ipv4_host_and_port() {
        assert!(<&ServerName>::try_from("1.1.1.1:12000").is_ok());
    }

    #[test]
    fn ipv6() {
        assert!(<&ServerName>::try_from("[::1]").is_ok());
    }

    #[test]
    fn ipv6_with_port() {
        assert!(<&ServerName>::try_from("[1234:5678::abcd]:5678").is_ok());
    }

    #[test]
    fn dns_name() {
        assert!(<&ServerName>::try_from("example.com").is_ok());
    }

    #[test]
    fn dns_name_with_port() {
        assert!(<&ServerName>::try_from("ruma.io:8080").is_ok());
    }

    #[test]
    fn empty_string() {
        assert!(<&ServerName>::try_from("").is_err());
    }

    #[test]
    fn invalid_ipv6() {
        assert!(<&ServerName>::try_from("[test::1]").is_err());
    }

    #[test]
    fn ipv4_with_invalid_port() {
        assert!(<&ServerName>::try_from("127.0.0.1:").is_err());
    }

    #[test]
    fn ipv6_with_invalid_port() {
        assert!(<&ServerName>::try_from("[fe80::1]:100000").is_err());
        assert!(<&ServerName>::try_from("[fe80::1]!").is_err());
    }

    #[test]
    fn dns_name_with_invalid_port() {
        assert!(<&ServerName>::try_from("matrix.org:hello").is_err());
    }
}
