//! `Connection` JS object's representation.

use crate::{
    browser::Statement,
    object::{tracks_store, MediaKind, Object},
};

use super::Error;

/// Representation of a `Connection` JS object.
#[derive(Clone, Copy, Debug)]
pub struct Connection;

impl Object<Connection> {
    /// Returns a [`tracks_store::Remote`] of this [`Connection`].
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn tracks_store(
        &self,
    ) -> Result<Object<tracks_store::Remote>, Error> {
        self.execute_and_fetch(Statement::new(
            // language=JavaScript
            r#"async (conn) => conn.tracksStore"#,
            [],
        ))
        .await
    }

    /// Enables remote media receiving for the provided [`MediaKind`].
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn enable_remote_media(
        &self,
        kind: MediaKind,
    ) -> Result<(), Error> {
        let enable = match kind {
            MediaKind::Audio => "c.conn.enable_remote_audio()",
            MediaKind::Video => "c.conn.enable_remote_video()",
        };
        self.execute(Statement::new(
            // language=JavaScript
            &format!(
                r#"
                async (c) => {{
                    await {enable};
                }}
                "#,
            ),
            [],
        ))
        .await
        .map(drop)
    }

    /// Disables remote media receiving for the provided [`MediaKind`].
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn disable_remote_media(
        &self,
        kind: MediaKind,
    ) -> Result<(), Error> {
        let disable = match kind {
            MediaKind::Audio => "c.conn.disable_remote_audio()",
            MediaKind::Video => "c.conn.disable_remote_video()",
        };
        self.execute(Statement::new(
            // language=JavaScript
            &format!(
                r#"
                async (c) => {{
                    await {disable};
                }}
                "#,
            ),
            [],
        ))
        .await
        .map(drop)
    }

    /// Returns a [`Future`] resolving when `Connection.on_close()` callback is
    /// fired.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    ///
    /// [`Future`]: std::future::Future
    pub async fn wait_for_close(&self) -> Result<(), Error> {
        self.execute(Statement::new(
            // language=JavaScript
            r#"
            async (conn) => {
                await new Promise((resolve) => {
                    if (!conn.closeListener.isClosed) {
                        conn.closeListener.subs.push(resolve);
                    } else {
                        resolve();
                    }
                });
            }
            "#,
            [],
        ))
        .await
        .map(drop)
    }
}
