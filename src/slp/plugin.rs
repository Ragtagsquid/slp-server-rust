pub use super::{PeerManager, InPacket, OutPacket, Packet};
pub use async_trait::async_trait;
pub use std::net::SocketAddr;
use downcast_rs::{Downcast, impl_downcast};

pub struct Context<'a> {
    pub peer_manager: &'a PeerManager,
}

impl<'a> Context<'a> {
    pub fn new(peer_manager: &'a PeerManager) -> Self {
        Self {
            peer_manager,
        }
    }
}

pub trait PluginType<T = BoxPlugin> {
    fn name(&self) -> String;
    fn new(&self, context: Context) -> BoxPlugin;
}

pub type BoxPluginType<T = BoxPlugin> = Box<dyn PluginType<T> + Send + Sync + 'static>;

#[async_trait]
pub trait Plugin: Downcast {
    async fn in_packet(&mut self, packet: &InPacket);
    async fn out_packet(&mut self, packet: &Packet, addrs: &[SocketAddr]);
}
impl_downcast!(Plugin);

pub type BoxPlugin = Box<dyn Plugin + Send + 'static>;
