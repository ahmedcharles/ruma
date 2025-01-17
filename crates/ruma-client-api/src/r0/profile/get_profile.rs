//! [GET /_matrix/client/r0/profile/{userId}](https://matrix.org/docs/spec/client_server/r0.6.0#get-matrix-client-r0-profile-userid)

use ruma_api::ruma_api;
use ruma_identifiers::{MxcUri, UserId};

ruma_api! {
    metadata: {
        description: "Get all profile information of an user.",
        method: GET,
        name: "get_profile",
        path: "/_matrix/client/r0/profile/:user_id",
        rate_limited: false,
        authentication: None,
    }

    request: {
        /// The user whose profile will be retrieved.
        #[ruma_api(path)]
        pub user_id: &'a UserId,
    }

    #[derive(Default)]
    response: {
        /// The user's avatar URL, if set.
        ///
        /// If you activate the `compat` feature, this field being an empty string in JSON will give
        /// you `None` here.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(
            feature = "compat",
            serde(default, deserialize_with = "ruma_serde::empty_string_as_none")
        )]
        pub avatar_url: Option<MxcUri>,

        /// The user's display name, if set.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub displayname: Option<String>,

        /// The [BlurHash](https://blurha.sh) for the avatar pointed to by `avatar_url`.
        ///
        /// This uses the unstable prefix in
        /// [MSC2448](https://github.com/matrix-org/matrix-doc/pull/2448).
        #[cfg(feature = "unstable-pre-spec")]
        #[cfg_attr(docsrs, doc(cfg(feature = "unstable-pre-spec")))]
        #[serde(rename = "xyz.amorgan.blurhash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub blurhash: Option<String>,
    }

    error: crate::Error
}

impl<'a> Request<'a> {
    /// Creates a new `Request` with the given user ID.
    pub fn new(user_id: &'a UserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a new `Response` with the given avatar URL and display name.
    pub fn new(avatar_url: Option<MxcUri>, displayname: Option<String>) -> Self {
        Self {
            avatar_url,
            displayname,
            #[cfg(feature = "unstable-pre-spec")]
            blurhash: None,
        }
    }
}
