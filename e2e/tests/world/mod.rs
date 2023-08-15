//! E2E tests [`World`][1].
//!
//! [1]: cucumber::World

pub mod member;

use std::{collections::HashMap, fmt, time::Duration};

use derive_more::{Display, Error, From};
use medea_control_api_mock::{
    callback::{CallbackEvent, CallbackItem},
    proto,
    proto::PublishPolicy,
};
use medea_e2e::{
    browser::{self, WebDriverClientBuilder, WindowFactory},
    object::{self, Jason, MediaKind, MediaSourceKind, Object},
};
use tokio::time::interval;
use uuid::Uuid;

use crate::{conf, control};

pub use self::member::{Builder as MemberBuilder, Member};

/// Returns Control API path for the provided `room_id`, `member_id` and
/// `endpoint_id`.
macro_rules! control_api_path {
    ($room_id:expr) => {
        format!("{}", $room_id)
    };
    ($room_id:expr, $member_id:expr) => {
        format!("{}/{}", $room_id, $member_id)
    };
    ($room_id:expr, $member_id:expr, $endpoint_id:expr) => {
        format!("{}/{}/{}", $room_id, $member_id, $endpoint_id)
    };
}

/// All errors which can happen while working with [`World`].
#[derive(Debug, Display, Error, From)]
pub enum Error {
    Control(control::Error),
    Object(object::Error),
    Member(member::Error),
    Browser(browser::Error),
    MemberNotFound(#[error(not(source))] String),
}

type Result<T> = std::result::Result<T, Error>;

/// [`World`][1] used by all E2E tests.
///
/// [1]: cucumber::World
#[derive(cucumber::World)]
#[world(init = Self::try_new)]
pub struct World {
    /// ID of the `Room` created for this [`World`].
    room_id: String,

    /// Client of a Medea Control API.
    control_client: control::Client,

    /// All [`Member`]s created in this [`World`].
    members: HashMap<String, Member>,

    /// All [`Jason`] [`Object`]s created in this [`World`].
    jasons: HashMap<String, Object<Jason>>,

    /// [WebDriver] client that all [`Object`]s of this [`World`] will be
    /// created with.
    ///
    /// [WebDriver]: https://w3.org/TR/webdriver
    window_factory: WindowFactory,
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("World")
            .field("room_id", &self.room_id)
            .field("members", &self.members)
            .finish_non_exhaustive()
    }
}

impl World {
    /// Attempts to initialized a new fresh [`World`].
    ///
    /// # Errors
    ///
    /// If fails to instantiate a [`control::Client`] or a [WebDriver] client.
    ///
    /// [WebDriver]: https://w3.org/TR/webdriver
    pub async fn try_new() -> Result<Self> {
        let room_id = Uuid::new_v4().to_string();

        let control_client = control::Client::new(&conf::CONTROL_API_ADDR);
        control_client
            .create(
                &room_id,
                proto::Element::Room(proto::Room {
                    id: room_id.clone(),
                    pipeline: HashMap::new(),
                }),
            )
            .await?;

        Ok(Self {
            room_id,
            control_client,
            window_factory: WebDriverClientBuilder::new(&conf::WEBDRIVER_ADDR)
                .headless_firefox(*conf::HEADLESS)
                .headless_chrome(*conf::HEADLESS)
                .connect(&conf::FILE_SERVER_HOST)
                .await?
                .into(),
            members: HashMap::new(),
            jasons: HashMap::new(),
        })
    }

    /// Creates a new [`Member`] from the provided [`MemberBuilder`].
    ///
    /// `Room` for this [`Member`] will be created, but joining won't be done.
    ///
    /// # Errors
    ///
    /// - If the performed request to Control API fails.
    /// - If initializing the [`Member`] fails on JS side.
    #[allow(clippy::too_many_lines)]
    pub async fn create_member(
        &mut self,
        builder: MemberBuilder,
    ) -> Result<()> {
        let is_sfu = std::env::var("SFU").is_ok();

        let mut pipeline = HashMap::new();
        let mut send_state = HashMap::new();
        let mut recv_state = HashMap::new();

        if builder.is_send {
            send_state
                .insert((MediaKind::Audio, MediaSourceKind::Device), true);
            send_state
                .insert((MediaKind::Video, MediaSourceKind::Device), true);
            if is_sfu {
                send_state
                    .insert((MediaKind::Video, MediaSourceKind::Display), true);
            }
            pipeline.insert(
                "publish".to_owned(),
                proto::Endpoint::WebRtcPublishEndpoint(
                    proto::WebRtcPublishEndpoint {
                        id: "publish".to_owned(),
                        p2p: if is_sfu {
                            proto::P2pMode::Never
                        } else {
                            proto::P2pMode::Always
                        },
                        force_relay: false,
                        audio_settings: proto::AudioSettings::default(),
                        video_settings: proto::VideoSettings::default(),
                    },
                ),
            );
        }
        if builder.is_recv {
            recv_state
                .insert((MediaKind::Audio, MediaSourceKind::Device), true);
            recv_state
                .insert((MediaKind::Video, MediaSourceKind::Device), true);
            if is_sfu {
                recv_state
                    .insert((MediaKind::Video, MediaSourceKind::Display), true);
            }
            self.members.values().filter(|m| m.is_send()).for_each(|m| {
                let endpoint_id = format!("play-{}", m.id());
                pipeline.insert(
                    endpoint_id.clone(),
                    proto::Endpoint::WebRtcPlayEndpoint(
                        proto::WebRtcPlayEndpoint {
                            id: endpoint_id,
                            src: format!(
                                "local://{}/{}/publish",
                                self.room_id,
                                m.id(),
                            ),
                            force_relay: false,
                        },
                    ),
                );
            });
        }

        self.control_client
            .create(
                &format!("{}/{}", self.room_id, builder.id),
                proto::Element::Member(Box::new(proto::Member {
                    id: builder.id.clone(),
                    pipeline,
                    credentials: Some(proto::Credentials::Plain(
                        "test".to_owned(),
                    )),
                    on_join: Some("grpc://127.0.0.1:9099".to_owned()),
                    on_leave: Some("grpc://127.0.0.1:9099".to_owned()),
                    idle_timeout: None,
                    reconnect_timeout: None,
                    ping_interval: None,
                })),
            )
            .await?;

        if builder.is_send {
            let recv_endpoints = self
                .members
                .values()
                .filter_map(|m| {
                    m.is_recv().then(|| {
                        let endpoint_id = format!("play-{}", builder.id);
                        let id = format!(
                            "{}/{}/{endpoint_id}",
                            self.room_id,
                            m.id(),
                        );
                        let elem = proto::Element::WebRtcPlayEndpoint(
                            proto::WebRtcPlayEndpoint {
                                id: endpoint_id,
                                src: format!(
                                    "local://{}/{}/publish",
                                    self.room_id, builder.id,
                                ),
                                force_relay: false,
                            },
                        );
                        (id, elem)
                    })
                })
                .collect::<HashMap<_, _>>();
            for (path, element) in recv_endpoints {
                self.control_client.create(&path, element).await?;
            }
        }
        let window = self.window_factory.new_window().await;
        let jason = Object::spawn(Jason, window.clone()).await?;
        let room = jason.init_room().await?;
        let member =
            builder.build(room, window, send_state, recv_state).await?;

        self.jasons.insert(member.id().to_owned(), jason);
        self.members.insert(member.id().to_owned(), member);

        Ok(())
    }

    /// Returns reference to a [`Member`] with the provided ID.
    ///
    /// Returns [`None`] if a [`Member`] with the provided ID doesn't exist.
    #[must_use]
    pub fn get_member(&self, member_id: &str) -> Option<&Member> {
        self.members.get(member_id)
    }

    /// Joins a [`Member`] with the provided ID to the `Room` created for this
    /// [`World`].
    ///
    /// # Errors
    ///
    /// - If the specified [`Member`] doesn't exist in this [`World`].
    /// - If joining the `Room` fails on JS side.
    pub async fn join_room(&mut self, member_id: &str) -> Result<()> {
        let member = self
            .members
            .get_mut(member_id)
            .ok_or_else(|| Error::MemberNotFound(member_id.to_owned()))?;
        member.join_room(&self.room_id).await?;
        Ok(())
    }

    /// Waits until a [`Member`] with the provided ID will connect with his
    /// responders.
    ///
    /// # Errors
    ///
    /// If waiting fails on JS side.
    ///
    /// # Panics
    ///
    /// If the specified [`Member`] doesn't exist in this [`World`].
    pub async fn wait_for_interconnection(
        &mut self,
        member_id: &str,
    ) -> Result<()> {
        let interconnected_members = self.members.values().filter(|m| {
            m.is_joined() && m.id() != member_id && (m.is_recv() || m.is_send())
        });
        let member = self.members.get(member_id).unwrap();
        for partner in interconnected_members {
            let (send_count, recv_count) =
                member.count_of_tracks_between_members(partner);
            let conn = member
                .connections()
                .wait_for_connection(partner.id().to_owned())
                .await?;
            conn.tracks_store()
                .await?
                .wait_for_count(recv_count)
                .await?;

            let partner_conn = partner
                .connections()
                .wait_for_connection(member_id.to_owned())
                .await?;
            partner_conn
                .tracks_store()
                .await?
                .wait_for_count(send_count)
                .await?;
        }
        Ok(())
    }

    /// Closes a [`Room`] of the provided [`Member`].
    ///
    /// # Errors
    ///
    /// If the [`Room`] fails to be closed.
    ///
    /// # Panics
    ///
    /// If the provided [`Member`] doesn't exist in this [`World`], or there are
    /// no [`Jason`] objects present in this [`World`] for him.
    ///
    /// [`Room`]: crate::object::room::Room
    pub async fn close_room(&mut self, member_id: &str) -> Result<()> {
        let jason = self.jasons.get(member_id).unwrap();
        let member = self.members.get(member_id).unwrap();
        let room = member.room();
        jason.close_room(room).await?;
        Ok(())
    }

    /// Waits for the [`Member`]'s [`Room`] being closed.
    ///
    /// # Errors
    ///
    /// - If the specified [`Member`] doesn't exist in this [`World`].
    /// - If waiting for the [`Member`] fails on JS side.
    ///
    /// [`Room`]: crate::object::room::Room
    pub async fn wait_for_on_close(&self, member_id: &str) -> Result<String> {
        let member = self
            .members
            .get(member_id)
            .ok_or_else(|| Error::MemberNotFound(member_id.to_owned()))?;

        Ok(member.room().wait_for_close().await?)
    }

    /// Waits for `OnLeave` Control API callback for the provided [`Member`] ID.
    ///
    /// # Panics
    ///
    /// Asserts the `OnLeave` reason to be equal to the provided one.
    pub async fn wait_for_on_leave(
        &mut self,
        member_id: String,
        reason: String,
    ) {
        let mut interval = interval(Duration::from_millis(50));
        loop {
            let callbacks = self.get_callbacks().await;
            let on_leave = callbacks
                .into_iter()
                .filter(|e| e.fid.contains(&member_id))
                .find_map(|e| {
                    if let CallbackEvent::OnLeave(on_leave) = e.event {
                        Some(on_leave)
                    } else {
                        None
                    }
                });
            if let Some(on_leave) = on_leave {
                assert_eq!(on_leave.reason.to_string(), reason);
                break;
            }
            interval.tick().await;
        }
    }

    /// Waits for `OnJoin` Control API callback for the provided [`Member`] ID.
    pub async fn wait_for_on_join(&mut self, member_id: String) {
        let mut interval = interval(Duration::from_millis(50));
        loop {
            let callbacks = self.get_callbacks().await;
            let on_join_found = callbacks
                .into_iter()
                .filter(|e| e.fid.contains(&member_id))
                .any(|e| matches!(e.event, CallbackEvent::OnJoin(_)));
            if on_join_found {
                break;
            }
            interval.tick().await;
        }
    }

    /// Creates `WebRtcPublishEndpoint`s and `WebRtcPlayEndpoint`s for the
    /// provided [`MembersPair`] using an `Apply` method of Control API.
    ///
    /// # Panics
    ///
    /// If the provided [`MembersPair`] is misconfigured.
    pub async fn interconnect_members_via_apply(&mut self, pair: MembersPair) {
        let mut spec = self.get_spec().await;
        if let Some(proto::RoomElement::Member(member)) =
            spec.pipeline.get_mut(&pair.left.id)
        {
            member.pipeline.insert(
                "publish".to_string(),
                proto::Endpoint::WebRtcPublishEndpoint(
                    pair.left.publish_endpoint().unwrap(),
                ),
            );
            let play_endpoint = pair
                .left
                .play_endpoint_for(&self.room_id, &pair.right)
                .unwrap();
            member.pipeline.insert(
                play_endpoint.id.clone(),
                proto::Endpoint::WebRtcPlayEndpoint(play_endpoint),
            );
        }
        if let Some(proto::RoomElement::Member(member)) =
            spec.pipeline.get_mut(&pair.right.id)
        {
            member.pipeline.insert(
                "publish".to_string(),
                proto::Endpoint::WebRtcPublishEndpoint(
                    pair.right.publish_endpoint().unwrap(),
                ),
            );
            let play_endpoint = pair
                .right
                .play_endpoint_for(&self.room_id, &pair.left)
                .unwrap();
            member.pipeline.insert(
                play_endpoint.id.clone(),
                proto::Endpoint::WebRtcPlayEndpoint(play_endpoint),
            );
        }
        self.apply(spec).await;
    }

    /// Creates `WebRtcPublishEndpoint`s and `WebRtcPlayEndpoint`s for the
    /// provided [`MembersPair`].
    ///
    /// # Errors
    ///
    /// If the performed requests to Control API fail.
    ///
    /// # Panics
    ///
    /// If any [`Member`] of the provided [`MembersPair`] doesn't exist in this
    /// [`World`].
    #[allow(clippy::too_many_lines)]
    pub async fn interconnect_members(
        &mut self,
        pair: MembersPair,
    ) -> Result<()> {
        if let Some(publish_endpoint) = pair.left.publish_endpoint() {
            let left_member = self.members.get_mut(&pair.left.id).unwrap();
            if publish_endpoint.audio_settings.publish_policy
                != PublishPolicy::Disabled
            {
                left_member.update_send_media_state(
                    Some(MediaKind::Audio),
                    None,
                    true,
                );
            }
            if publish_endpoint.video_settings.publish_policy
                != PublishPolicy::Disabled
            {
                left_member.update_send_media_state(
                    Some(MediaKind::Video),
                    None,
                    true,
                );
            }
            self.control_client
                .create(
                    &control_api_path!(self.room_id, pair.left.id, "publish"),
                    publish_endpoint.into(),
                )
                .await?;
        }
        if let Some(publish_endpoint) = pair.right.publish_endpoint() {
            let right_member = self.members.get_mut(&pair.right.id).unwrap();
            if publish_endpoint.audio_settings.publish_policy
                != PublishPolicy::Disabled
            {
                right_member.update_send_media_state(
                    Some(MediaKind::Audio),
                    None,
                    true,
                );
            }
            if publish_endpoint.video_settings.publish_policy
                != PublishPolicy::Disabled
            {
                right_member.update_send_media_state(
                    Some(MediaKind::Video),
                    None,
                    true,
                );
            }
            self.control_client
                .create(
                    &control_api_path!(self.room_id, pair.right.id, "publish"),
                    publish_endpoint.into(),
                )
                .await?;
        }

        if let Some(play_endpoint) =
            pair.left.play_endpoint_for(&self.room_id, &pair.right)
        {
            let left_member = self.members.get_mut(&pair.left.id).unwrap();

            left_member.update_recv_media_state(
                Some(MediaKind::Video),
                None,
                true,
            );
            left_member.update_recv_media_state(
                Some(MediaKind::Audio),
                None,
                true,
            );

            self.control_client
                .create(
                    &control_api_path!(
                        self.room_id,
                        pair.left.id,
                        play_endpoint.id
                    ),
                    play_endpoint.into(),
                )
                .await?;
        }
        if let Some(play_endpoint) =
            pair.right.play_endpoint_for(&self.room_id, &pair.left)
        {
            let right_member = self.members.get_mut(&pair.right.id).unwrap();

            right_member.update_recv_media_state(
                Some(MediaKind::Video),
                None,
                true,
            );
            right_member.update_recv_media_state(
                Some(MediaKind::Audio),
                None,
                true,
            );

            self.control_client
                .create(
                    &control_api_path!(
                        self.room_id,
                        pair.right.id,
                        play_endpoint.id
                    ),
                    play_endpoint.into(),
                )
                .await?;
        }

        {
            let left_member = self.members.get_mut(&pair.left.id).unwrap();
            left_member.set_is_send(pair.left.is_send());
            left_member.set_is_recv(pair.right.recv);
        }
        {
            let right_member = self.members.get_mut(&pair.right.id).unwrap();
            right_member.set_is_send(pair.right.is_send());
            right_member.set_is_recv(pair.right.recv);
        }

        Ok(())
    }

    /// Disposes a [`Jason`] object of the provided [`Member`] ID.
    ///
    /// # Errors
    ///
    /// If the [`Jason`] object fails to be disposed on JS side.
    ///
    /// # Panics
    ///
    /// If no [`Jason`] objects exist for the provided [`Member`] in this
    /// [`World`].
    pub async fn dispose_jason(&mut self, member_id: &str) -> Result<()> {
        let jason = self.jasons.remove(member_id).unwrap();
        jason.dispose().await?;
        Ok(())
    }

    /// Deletes a Control API element of a `WebRtcPublishEndpoint` with the
    /// provided ID.
    ///
    /// # Panics
    ///
    /// If the performed request to Control API fails.
    pub async fn delete_publish_endpoint(&mut self, member_id: &str) {
        let resp = self
            .control_client
            .delete(&format!("{}/{member_id}/publish", self.room_id))
            .await
            .unwrap();
        assert!(resp.error.is_none());
    }

    /// Deletes a Control API element of a `WebRtcPlayEndpoint` with the
    /// provided ID.
    ///
    /// # Panics
    ///
    /// If the performed request to Control API fails.
    pub async fn delete_play_endpoint(
        &mut self,
        member_id: &str,
        partner_member_id: &str,
    ) {
        let play_endpoint_id = format!("play-{partner_member_id}");
        let resp = self
            .control_client
            .delete(&format!("{}/{member_id}/{play_endpoint_id}", self.room_id))
            .await
            .unwrap();
        assert!(resp.error.is_none());
    }

    /// Deletes a Control API element of the [`Member`] with the provided ID.
    ///
    /// # Panics
    ///
    /// If the performed request to Control API fails.
    pub async fn delete_member_element(&mut self, member_id: &str) {
        let resposne = self
            .control_client
            .delete(&format!("{}/{member_id}", self.room_id))
            .await
            .unwrap();
        assert!(resposne.error.is_none());
    }

    /// Deletes a Control API element of the [`Room`] with the provided ID.
    ///
    /// # Panics
    ///
    /// If the performed request to Control API fails.
    ///
    /// [`Room`]: crate::object::room::Room
    pub async fn delete_room_element(&mut self) {
        let resp = self
            .control_client
            .delete(self.room_id.as_str())
            .await
            .unwrap();
        assert!(resp.error.is_none());
    }

    /// Returns all [`CallbackItem`]s sent by Control API for this [`World`]'s
    /// `Room`.
    async fn get_callbacks(&mut self) -> Vec<CallbackItem> {
        self.control_client
            .callbacks()
            .await
            .unwrap()
            .into_iter()
            .filter(|i| i.fid.contains(&self.room_id))
            .collect()
    }

    /// Returns [`proto::Room`] spec of the `Room` created for this [`World`].
    ///
    /// # Panics
    ///
    /// - If the performed request to Control API fails.
    /// - If the element returned by Control API is absent or is not a `Room`.
    pub async fn get_spec(&mut self) -> proto::Room {
        let el = self
            .control_client
            .get(&self.room_id)
            .await
            .unwrap()
            .element
            .unwrap();
        if let proto::Element::Room(room) = el {
            room
        } else {
            panic!("Returned not `Room` element")
        }
    }

    /// Applies provided [`proto::Room`] spec to the `Room` created for this
    /// [`World`].
    ///
    /// # Panics
    ///
    /// If the performed request to Control API fails.
    pub async fn apply(&mut self, el: proto::Room) {
        self.control_client
            .apply(&self.room_id, proto::Element::Room(el))
            .await
            .unwrap();
    }
}

/// `Member`s pairing configuration.
///
/// Based on this configuration [`World`] can dynamically create `Endpoint`s for
/// this `Member`s.
pub struct MembersPair {
    /// First [`PairedMember`] in a pair.
    pub left: PairedMember,

    /// Second [`PairedMember`] in a pair.
    pub right: PairedMember,
}

/// `Endpoint`s configuration of a `Member`.
pub struct PairedMember {
    /// Unique ID of this [`PairedMember`].
    pub id: String,

    /// Audio settings to be sent by this [`PairedMember`].
    pub send_audio: Option<proto::AudioSettings>,

    /// Video settings to be sent by this [`PairedMember`].
    pub send_video: Option<proto::VideoSettings>,

    /// Indicator whether this is a receiving configuration, rather than
    /// publishing.
    pub recv: bool,
}

impl PairedMember {
    /// Indicates whether this [`PairedMember`] should publish media.
    fn is_send(&self) -> bool {
        self.send_audio.is_some() || self.send_video.is_some()
    }

    /// Returns a [`proto::WebRtcPublishEndpoint`] for this [`PairedMember`] if
    /// publishing is enabled.
    fn publish_endpoint(&self) -> Option<proto::WebRtcPublishEndpoint> {
        self.is_send().then(|| proto::WebRtcPublishEndpoint {
            id: "publish".to_owned(),
            p2p: proto::P2pMode::Always,
            force_relay: false,
            audio_settings: self.send_audio.unwrap_or(proto::AudioSettings {
                publish_policy: PublishPolicy::Disabled,
            }),
            video_settings: self.send_video.unwrap_or(proto::VideoSettings {
                publish_policy: PublishPolicy::Disabled,
            }),
        })
    }

    /// Returns a [`proto::WebRtcPlayEndpoint`] for this [`PairedMember`] which
    /// will receive media from the provided [`PairedMember`] if receiving is
    /// enabled.
    fn play_endpoint_for(
        &self,
        room_id: &str,
        publisher: &PairedMember,
    ) -> Option<proto::WebRtcPlayEndpoint> {
        self.recv.then(|| proto::WebRtcPlayEndpoint {
            id: format!("play-{}", publisher.id),
            src: format!("local://{room_id}/{}/publish", publisher.id),
            force_relay: false,
        })
    }
}
