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
    /// `Connected`.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn wait_for_connected_state(
        &self,
        remote_id: String,
    ) -> Result<Object<MemberConnectionState>, Error> {
        self.execute_and_fetch(Statement::new(
            // language=JavaScript
            "
            async (store) => {
                const [remoteId] = args;
                const conn = store.connections.get(remoteId);

                if (!conn) {
                    throw new NotFoundError();
                }

                const state = conn.get_state();

                const P2P = window.rust.MemberConnectionStateKind.P2P;
                const CONNECTED = window.rust.PeerConnectionState.Connected;

                if (state) {
                    if (state.kind() !== P2P) {
                        throw new Error();
                    }

                    return state.value() === CONNECTED;
                }

                return new Promise((resolve) => {
                    conn.stateListener.subs.push(resolve);
                });
            }
            ",
            [remote_id.into()],
        ))
        .await
    }
}
