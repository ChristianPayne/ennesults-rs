// Helpers
use std::sync::Mutex;

// IRC
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::transport::tcp::{TCPTransport, TLS};

use serde::ser::{Serialize, Serializer};

#[derive(Debug)]
pub struct Bot {
  pub client: Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>>
}

impl Default for Bot {
    fn default() -> Self { 
        Bot {
          client: None
        }
    }
}

// impl Clone for Bot {
//   fn clone(&self) -> Self {
//     Bot {
//       client: &self.client.lock().unwrap().clone()
//     }
//   }
// }

// #[derive(Debug)]
// pub struct Connection (Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>>);

// impl Clone for Connection {
//   fn clone(&self) -> Self {
//     self.clone()
//   }
// }

// impl Connection {
//     pub fn test () {
//       println!("Testing");
//     }
//     pub async fn initialize (mut self) -> tokio::task::JoinHandle<()> {
//         tokio::spawn(async {
//           println!("Connection connecting!");
//           // default configuration is to join chat as anonymous.
//           let config = ClientConfig::default();
//           let (mut incoming_messages, client) =
//               TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
      
//           // first thing you should do: start consuming incoming messages,
//           // otherwise they will back up.
//           let join_handle = tokio::spawn(async move {
//               while let Some(message) = incoming_messages.recv().await {
//                   println!("Received message: {:?}", message);
//               }
//           });
      
//           // join a channel
//           let _ = client.join("ennegineer".to_owned());
      
//           // keep the tokio executor alive.
//           // If you return instead of waiting the background task will exit.
//           join_handle.await.unwrap();
//         })
//     }
// }