/* Error type based on the error type from es-rs:
 *
 * Copyright 2015-2018 Ben Ashford
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::http::transport::BuildError;
use serde_json;
use std::error;
use std::fmt;
use std::io;

/// An error within the client.
///
/// Errors that can occur include IO and parsing errors, as well as specific
/// errors from Elasticsearch and internal errors from this library
#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

#[derive(Debug)]
enum Kind {
    /// An error building the client
    Build(BuildError),

    /// A general error from this library
    Lib(String),

    /// HTTP library error
    Http(surf::Exception),

    /// IO error
    Io(io::Error),

    /// JSON error
    Json(serde_json::error::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error {
            kind: Kind::Io(err),
        }
    }
}

impl From<surf::Exception> for Error {
    fn from(err: surf::Exception) -> Error {
        Error {
            kind: Kind::Http(err),
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error {
            kind: Kind::Json(err),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error {
            kind: Kind::Lib(err.to_string()),
        }
    }
}

impl From<BuildError> for Error {
    fn from(err: BuildError) -> Error {
        Error {
            kind: Kind::Build(err),
        }
    }
}

impl Error {
    pub(crate) fn lib(err: impl Into<String>) -> Self {
        Error {
            kind: Kind::Lib(err.into()),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            Kind::Build(err) => Some(err),
            Kind::Lib(_) => None,
            Kind::Http(err) => None,  // TODO: cannot be some due to not Sized
            Kind::Io(err) => Some(err),
            Kind::Json(err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            Kind::Build(err) => err.fmt(f),
            Kind::Lib(err) => err.fmt(f),
            Kind::Http(err) => err.fmt(f),
            Kind::Io(err) => err.fmt(f),
            Kind::Json(err) => err.fmt(f),
        }
    }
}
