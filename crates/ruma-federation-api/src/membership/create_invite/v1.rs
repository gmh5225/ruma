//! `/v1/` ([spec])
//!
//! [spec]: https://spec.matrix.org/latest/server-server-api/#put_matrixfederationv1inviteroomideventid

use ruma_common::{
    api::{request, response, Metadata},
    metadata,
    serde::Raw,
    MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId, OwnedServerName, OwnedUserId,
};
use ruma_events::{room::member::RoomMemberEventContent, AnyStrippedStateEvent, StateEventType};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue as RawJsonValue;

const METADATA: Metadata = metadata! {
    method: PUT,
    rate_limited: false,
    authentication: ServerSignatures,
    history: {
        1.0 => "/_matrix/federation/v1/invite/:room_id/:event_id",
    }
};

/// Request type for the `create_invite` endpoint.
#[request]
pub struct Request {
    /// The room ID that the user is being invited to.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    /// The event ID for the invite event, generated by the inviting server.
    #[ruma_api(path)]
    pub event_id: OwnedEventId,

    /// The matrix ID of the user who sent the original `m.room.third_party_invite`.
    pub sender: OwnedUserId,

    /// The name of the inviting homeserver.
    pub origin: OwnedServerName,

    /// A timestamp added by the inviting homeserver.
    pub origin_server_ts: MilliSecondsSinceUnixEpoch,

    /// The value `m.room.member`.
    #[serde(rename = "type")]
    pub kind: StateEventType,

    /// The user ID of the invited member.
    pub state_key: OwnedUserId,

    /// The content of the event.
    pub content: RoomMemberEventContent,

    /// Information included alongside the event that is not signed.
    #[serde(default, skip_serializing_if = "UnsignedEventContent::is_empty")]
    pub unsigned: UnsignedEventContent,
}

/// Response type for the `create_invite` endpoint.
#[response]
pub struct Response {
    /// The signed invite event.
    #[ruma_api(body)]
    #[serde(with = "crate::serde::v1_pdu")]
    pub event: Box<RawJsonValue>,
}

/// Information included alongside an event that is not signed.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct UnsignedEventContent {
    /// An optional list of simplified events to help the receiver of the invite identify the room.
    /// The recommended events to include are the join rules, canonical alias, avatar, and name of
    /// the room.
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub invite_room_state: Vec<Raw<AnyStrippedStateEvent>>,
}

impl UnsignedEventContent {
    /// Creates an empty `UnsignedEventContent`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks whether all of the fields are empty.
    pub fn is_empty(&self) -> bool {
        self.invite_room_state.is_empty()
    }
}

/// Initial set of fields of `Request`.
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)]
pub struct RequestInit {
    /// The room ID that the user is being invited to.
    pub room_id: OwnedRoomId,

    /// The event ID for the invite event, generated by the inviting server.
    pub event_id: OwnedEventId,

    /// The matrix ID of the user who sent the original `m.room.third_party_invite`.
    pub sender: OwnedUserId,

    /// The name of the inviting homeserver.
    pub origin: OwnedServerName,

    /// A timestamp added by the inviting homeserver.
    pub origin_server_ts: MilliSecondsSinceUnixEpoch,

    /// The user ID of the invited member.
    pub state_key: OwnedUserId,

    /// The content of the event.
    pub content: RoomMemberEventContent,

    /// Information included alongside the event that is not signed.
    pub unsigned: UnsignedEventContent,
}

impl From<RequestInit> for Request {
    /// Creates a new `Request` from `RequestInit`.
    fn from(init: RequestInit) -> Self {
        Self {
            room_id: init.room_id,
            event_id: init.event_id,
            sender: init.sender,
            origin: init.origin,
            origin_server_ts: init.origin_server_ts,
            kind: StateEventType::RoomMember,
            state_key: init.state_key,
            content: init.content,
            unsigned: init.unsigned,
        }
    }
}

impl Response {
    /// Creates a new `Response` with the given invite event.
    pub fn new(event: Box<RawJsonValue>) -> Self {
        Self { event }
    }
}
