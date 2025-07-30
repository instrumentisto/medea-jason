//! [`Object`] storing all the [`Connection`]s thrown by
//! `Room.on_new_connection()` callback.

use crate::{
    browser::Statement,
    object::{Error, Object, connection::Connection},
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
    /// `Connected`.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn wait_for_connected_state(
        &self,
        remote_id: String,
    ) -> Result<Object<()>, Error> {
        self.execute_and_fetch(Statement::new(
            // language=JavaScript
            "
            async (store) => {
                const [remoteId] = args;
                const connection = store.connections.get(remoteId);

                if (!connection) {
                    throw new NotFoundError();
                }

                const handle = connection.conn;
                const state = handle.get_state();

                const P2P = window.rust.MemberConnectionStateKind.P2P;
                const CONNECTED = window.rust.PeerConnectionState.Connected;

                const isConnected = state && (
                    state.kind() !== P2P || state.value() === CONNECTED
                );

                if (isConnected) {
                    return;
                }

                return new Promise((resolve) => {
                    connection.stateListener.subs.push((state) => {
                        const isConnected = state.kind() !== P2P ||
                            state.value() === CONNECTED;

                        if (isConnected) {
                            return resolve();
                        }
                    });
                });
            }
            ",
            [remote_id.into()],
        ))
        .await
    }
}
