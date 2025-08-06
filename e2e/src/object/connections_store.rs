//! [`Object`] storing all the [`Connection`]s thrown by
//! `Room.on_new_connection()` callback.

use crate::{
    browser::Statement,
    object::{
        Error, Object,
        connection::{Connection, MemberConnectionState},
    },
};

/// Storage for [`Connection`]s thrown by `Room.on_new_connection()` callback.
#[derive(Clone, Copy, Debug)]
pub struct ConnectionStore;

impl Object<ConnectionStore> {
    /// Returns a [`Connection`] of the provided remote member.
    ///
    /// Returns [`None`] if it doesn't exist.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn get(
        &self,
        remote_id: String,
    ) -> Result<Option<Object<Connection>>, Error> {
        let connection = self
            .execute_and_fetch(Statement::new(
                // language=JavaScript
                "
                async (store) => {
                    const [id] = args;
                    return store.connections.get(id);
                }
                ",
                [remote_id.into()],
            ))
            .await?;

        Ok((!connection.is_undefined().await?).then_some(connection))
    }

    /// Returns a [`Connection`] for the provided remote member, awaiting for it
    /// if it doesn't exists at the moment.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn wait_for_connection(
        &self,
        remote_id: String,
    ) -> Result<Object<Connection>, Error> {
        self.execute_and_fetch(Statement::new(
            // language=JavaScript
            "
            async (store) => {
                const [remoteId] = args;
                let conn = store.connections.get(remoteId);
                if (conn !== undefined) {
                    return conn;
                } else {
                    let waiter = new Promise((resolve) => {
                        store.subs.set(remoteId, resolve);
                    });
                    return await waiter;
                }
            }
            ",
            [remote_id.into()],
        ))
        .await
    }

    /// Returns promise that resolves when [`MemberConnectionState`] is
    /// in specific state.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn wait_for_state(
        &self,
        remote_id: String,
        state: Option<MemberConnectionState>,
    ) -> Result<Object<()>, Error> {
        self.execute_and_fetch(Statement::new(
            // language=JavaScript
            "
            async (store) => {
                const kinds = {
                    P2P: window.rust.MemberConnectionStateKind.P2P,
                };
                const values = {
                    P2P: {
                        New: window.rust.PeerConnectionState.New,
                        Connecting: window.rust.PeerConnectionState.Connecting,
                        Connected: window.rust.PeerConnectionState.Connected,
                        Disconnected:
                            window.rust.PeerConnectionState.Disconnected,
                        Failed: window.rust.PeerConnectionState.Failed,
                        Closed: window.rust.PeerConnectionState.Closed,
                    },
                };

                const [remoteId, expected] = args;
                const connection = store.connections.get(remoteId);

                if (!connection) {
                    throw new NotFoundError();
                }

                const handle = connection.conn;
                const state = handle.get_state();

                if (state !== undefined) {
                    if (expected === null) {
                        throw new Error('State must not be set');
                    }

                    if (state.kind() !== kinds[expected.kind]) {
                        throw new Error('Wrong MemberConnectionStateKind');
                    }

                    const value = values[expected.kind][expected.value];
                    if (state.value() === value) {
                        return;
                    }
                } else if (expected === null) {
                    return;
                }

                return new Promise((resolve) => {
                    connection.stateListener.subs.push((state) => {
                        if (state.kind() !== kinds[expected.kind]) {
                            return reject();
                        }

                        const value = values[expected.kind][expected.value];
                        if (state.value() === value) {
                            return resolve();
                        }
                    });
                });
            }
            ",
            [
                remote_id.into(),
                serde_json::to_value(state).map_err(|_| Error::TypeCast)?,
            ],
        ))
        .await
    }
}
