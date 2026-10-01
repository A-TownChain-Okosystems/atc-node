// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! P0-NET-001 / P0-ID-001 enforcement boundary.
//! Network reachability never grants protocol authority.

use crate::peers::{PeerState, PeerTable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorityError {
    UnknownPeer,
    PeerNotVerified,
    PeerBanned,
    MissingCapability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkAction {
    Discover,
    Authenticate,
    PropagateTransaction,
    PropagateBlock,
    SynchronizeState,
    RequestState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    NetworkProtocol,
    StateRead,
    StatePropagation,
}

pub fn authorize_network_action(
    peers: &PeerTable,
    peer_id: u64,
    action: NetworkAction,
    capability: Option<Capability>,
) -> Result<(), AuthorityError> {
    if peers.state(peer_id) == Some(PeerState::Banned) {
        return Err(AuthorityError::PeerBanned);
    }
    if peers.state(peer_id).is_none() {
        return Err(AuthorityError::UnknownPeer);
    }
    if matches!(action, NetworkAction::PropagateBlock | NetworkAction::SynchronizeState | NetworkAction::RequestState)
        && peers.state(peer_id) != Some(PeerState::Verified)
    {
        return Err(AuthorityError::PeerNotVerified);
    }
    let required = match action {
        NetworkAction::Discover | NetworkAction::Authenticate => Capability::NetworkProtocol,
        NetworkAction::PropagateTransaction | NetworkAction::PropagateBlock => Capability::NetworkProtocol,
        NetworkAction::SynchronizeState | NetworkAction::RequestState => Capability::StateRead,
    };
    if capability != Some(required) {
        return Err(AuthorityError::MissingCapability);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reachability_does_not_create_authority() {
        let mut peers = PeerTable::new();
        assert!(peers.add(7, "127.0.0.1:1"));
        assert_eq!(
            authorize_network_action(&peers, 7, NetworkAction::PropagateBlock, Some(Capability::NetworkProtocol)),
            Err(AuthorityError::PeerNotVerified)
        );
        assert!(peers.verify(7));
        assert!(authorize_network_action(&peers, 7, NetworkAction::PropagateBlock, Some(Capability::NetworkProtocol)).is_ok());
    }

    #[test]
    fn banned_peer_is_denied() {
        let mut peers = PeerTable::new();
        peers.add(7, "127.0.0.1:1");
        peers.verify(7);
        peers.ban(7);
        assert_eq!(
            authorize_network_action(&peers, 7, NetworkAction::PropagateBlock, Some(Capability::NetworkProtocol)),
            Err(AuthorityError::PeerBanned)
        );
    }
}
