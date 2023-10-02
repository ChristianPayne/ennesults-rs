// Helpers
use std::sync::Mutex;
use std::sync::Arc;

// IRC
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::transport::tcp::{TCPTransport, TLS};

use serde::ser::{Serialize, Serializer};

// BOT 
#[derive(Debug)]
pub struct Bot {
  pub client : Mutex<Client>
}

impl Default for Bot {
    fn default() -> Self { 
        Bot {
          client: Mutex::new(Client(None))
        }
    }
}

// CLIENT
#[derive(Debug)]
pub struct Client (pub Option<Arc<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>>>);
impl Client {
  pub fn new(client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>) -> Self {
    Client(Some(Arc::new(client)))
  }
}
impl Default for Client {
  fn default() -> Self { 
    Client (None)
  }
}
pub fn get_client (state: &tauri::State<'_, Bot>) -> Option<Arc<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>>> {
  let mutex_result = &state.client.lock();
  match mutex_result {
      Ok(guard) => guard.0.clone(),
      Err(_) => {
          println!("Error getting client out of mutex!");
          None
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