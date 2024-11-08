use chrono::Utc;
use hiqlite::params;
use rauthy_common::constants::{
    DYN_CLIENT_CLEANUP_INTERVAL, DYN_CLIENT_CLEANUP_MINUTES, DYN_CLIENT_REG_TOKEN,
    ENABLE_DYN_CLIENT_REG,
};
use rauthy_common::is_hiqlite;
use rauthy_models::database::DB;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::clients_dyn::ClientDyn;
use sqlx::query_as;
use std::time::Duration;
use tracing::{debug, error, info};

/// Cleans up unused dynamically registered clients
pub async fn dyn_client_cleanup() {
    if !*ENABLE_DYN_CLIENT_REG {
        info!(
            "Dynamic client registration is not enabled - exiting dynamic_client_cleanup scheduler"
        );
        return;
    }
    if DYN_CLIENT_REG_TOKEN.is_some() {
        info!("Dynamic client registration is private - exiting dynamic_client_cleanup scheduler");
        return;
    }

    let mut interval = tokio::time::interval(Duration::from_secs(
        DYN_CLIENT_CLEANUP_INTERVAL.saturating_mul(60),
    ));

    loop {
        interval.tick().await;

        if !DB::client().is_leader_cache().await {
            debug!(
                    "Running HA mode without being the leader - skipping dynamic_client_cleanup scheduler"
                );
            continue;
        }
        debug!("Running dynamic_client_cleanup scheduler");

        let clients_res = if is_hiqlite() {
            DB::client()
                .query_as(
                    "SELECT * FROM clients_dyn WHERE last_used = null",
                    params!(),
                )
                .await
                .map_err(|err| err.to_string())
        } else {
            query_as!(
                ClientDyn,
                "SELECT * FROM clients_dyn WHERE last_used = null"
            )
            .fetch_all(DB::conn())
            .await
            .map_err(|err| err.to_string())
        };
        let clients: Vec<ClientDyn> = match clients_res {
            Ok(c) => c,
            Err(err) => {
                error!("{}", err);
                continue;
            }
        };

        let threshold = Utc::now().timestamp() - *DYN_CLIENT_CLEANUP_MINUTES;
        let mut cleaned_up = 0;
        for client in clients {
            if client.created < threshold {
                info!("Cleaning up unused dynamic client {}", client.id);
                match Client::find(client.id).await {
                    Ok(c) => {
                        if let Err(err) = c.delete().await {
                            error!("Error deleting unused client: {:?}", err);
                            continue;
                        }

                        cleaned_up += 1;
                    }
                    Err(err) => {
                        error!("Client does not exist for ClientDyn when it should. This should never happen.\
                        Please report this issue: {:?}", err);
                        continue;
                    }
                }
            }
        }

        if cleaned_up > 0 {
            info!("Cleaned up {} unused dynamic clients", cleaned_up);
        }
    }
}
