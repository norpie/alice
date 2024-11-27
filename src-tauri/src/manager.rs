use crate::{api::Api, events, prelude::*};

use std::{fmt::Debug, sync::Arc, time::Duration};

use tokio::{
    sync::{
        watch::{self, Sender},
        Mutex,
    },
    task::JoinHandle,
    time,
};

pub struct Manager {
    pub api: Arc<Mutex<dyn Api>>,
    keep_alive: Arc<Mutex<Option<JoinHandle<()>>>>,
    keep_alive_stop_signal: Arc<Mutex<Option<Sender<()>>>>,
}

impl Manager {
    pub fn new(api: Arc<Mutex<dyn Api>>) -> Result<Self> {
        Ok(Self {
            api,
            keep_alive: Arc::new(Mutex::new(None)),
            keep_alive_stop_signal: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn set_api(&mut self, api: Arc<Mutex<dyn Api>>) -> Result<()> {
        self.stop_keep_alive().await?;
        self.api = api;
        self.start_keep_alive().await
    }

    pub async fn start_keep_alive(&mut self) -> Result<()> {
        let api = self.api.clone();
        let (tx, mut rx) = watch::channel(());
        {
            let mut stop_signal = self.keep_alive_stop_signal.lock().await;
            *stop_signal = Some(tx);
        }
        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = rx.changed() => {
                        break;
                    }
                    _ = time::sleep(Duration::from_secs(5)) => {
                        let mut api = api.lock().await;
                        let result = api.is_alive().await;
                        if let Ok(is_alive) = result {
                            if !is_alive {
                                let _ = events::emit_connection_status(false).await;
                            } else {
                                continue;
                            }
                        }
                        let result = api.connect().await;
                        if let Err(e) = result {
                            eprintln!("Failed to reconnect: {}", e);
                        } else {
                            let result = events::emit_connection_status(true).await;
                        }
                    }
                }
            }
        });
        {
            let mut keep_alive = self.keep_alive.lock().await;
            *keep_alive = Some(handle);
        }
        Ok(())
    }

    pub async fn stop_keep_alive(&mut self) -> Result<()> {
        // Notify the current task to stop.
        if let Some(tx) = self.keep_alive_stop_signal.lock().await.take() {
            let _ = tx.send(());
        }

        // Wait for the task to complete.
        if let Some(handle) = self.keep_alive.lock().await.take() {
            let _ = handle.await;
        }
        Ok(())
    }
}

impl Debug for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Manager").finish()
    }
}
