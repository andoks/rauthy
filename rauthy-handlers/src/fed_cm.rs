use actix_web::http::header;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use rauthy_common::constants::{
    COOKIE_SESSION_FED_CM, COOKIE_USER, EXPERIMENTAL_FED_CM_ENABLE, HEADER_ALLOW_ALL_ORIGINS,
    HEADER_JSON, SESSION_TIMEOUT_FED_CM,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::real_ip_from_req;
use rauthy_models::api_cookie::ApiCookie;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::fed_cm::{
    FedCMAccount, FedCMAccounts, FedCMClientMetadata, FedCMIdPConfig, FedCMLoginStatus,
    FedCMTokenResponse, WebIdentity,
};
use rauthy_models::entity::sessions::Session;
use rauthy_models::entity::users::User;
use rauthy_models::request::{FedCMAssertionRequest, FedCMClientMetadataRequest};
use rauthy_models::ListenScheme;
use rauthy_service::token_set::{AuthCodeFlow, DeviceCodeFlow, TokenNonce, TokenSet};
use tracing::{debug, warn};

const HEADER_ALLOW_CREDENTIALS: (&str, &str) = ("access-control-allow-credentials", "true");

/// GET accounts linked to the users
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    get,
    path = "/fed_cm/accounts",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok", body = FedCMAccounts),
        (status = 400, description = "BadRequest"),
    ),
)]
#[get("/fed_cm/accounts")]
#[tracing::instrument(level = "debug", skip_all)]
pub async fn get_fed_cm_accounts(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;
    is_web_identity_fetch(&req)?;

    let (login_status, user_id) = login_status_from_req(&data, &req).await;
    if login_status == FedCMLoginStatus::LoggedOut {
        return Ok(HttpResponse::Unauthorized()
            .insert_header(FedCMLoginStatus::LoggedOut.as_header_pair())
            .json(FedCMAccounts {
                accounts: Vec::default(),
            }));
    }

    let user = User::find_for_fed_cm_validated(&data, user_id).await?;

    // let clients = Client::find_all(&data)
    //     .await?
    //     .into_iter()
    //     .filter_map(|c| (c.id != "rauthy").then_some(c.id))
    //     .collect::<Vec<String>>();

    let account = FedCMAccount::build(user);
    let accounts = FedCMAccounts {
        accounts: vec![account],
    };
    Ok(HttpResponse::Ok()
        .insert_header(FedCMLoginStatus::LoggedIn.as_header_pair())
        .json(accounts))
}

/// GET metadata for the FedCM client
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    get,
    path = "/fed_cm/client_meta",
    tag = "fed_cm",
    params(FedCMClientMetadataRequest),
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/fed_cm/client_meta")]
#[tracing::instrument(level = "debug", skip_all, fields(client_id = params.client_id))]
pub async fn get_fed_cm_client_meta(
    data: web::Data<AppState>,
    req: HttpRequest,
    params: actix_web_validator::Query<FedCMClientMetadataRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;
    is_web_identity_fetch(&req)?;

    let params = params.into_inner();

    if &params.client_id == "rauthy" {
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("client-forbidden".to_string()),
            "The 'rauthy' client is forbidden to be used with FedCM".to_string(),
        ));
    }

    let client = Client::find_maybe_ephemeral(&data, params.client_id).await?;
    if !client.enabled {
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("client-disabled".to_string()),
            "This client has been disabled".to_string(),
        ));
    }
    let origin_header = client_origin_header(&data, &req, &client)?;

    let meta = FedCMClientMetadata::new();
    let (login_status, _) = login_status_from_req(&data, &req).await;
    Ok(HttpResponse::Ok()
        .insert_header(login_status.as_header_pair())
        .insert_header(HEADER_ALLOW_CREDENTIALS)
        .insert_header(origin_header)
        .json(meta))
}

/// The FedCM IdP configuration
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    get,
    path = "/fed_cm/config",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok", body = FedCMIdPConfig),
        (status = 400, description = "BadRequest"),
    ),
)]
#[get("/fed_cm/config")]
#[tracing::instrument(level = "debug", skip_all)]
pub async fn get_fed_cm_config(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;
    is_web_identity_fetch(&req)?;

    let config = FedCMIdPConfig::get(&data).await?;
    let (login_status, _) = login_status_from_req(&data, &req).await;
    Ok(HttpResponse::Ok()
        .insert_header(login_status.as_header_pair())
        .insert_header(HEADER_JSON)
        .insert_header(HEADER_ALLOW_ALL_ORIGINS)
        .json(config))
}

// /// Disconnect an account
// ///
// /// https://fedidcg.github.io/FedCM/#idp-api
// #[utoipa::path(
//     get,
//     path = "/fed_cm/disconnect",
//     tag = "fed_cm",
//     responses(
//         (status = 200, description = "Ok"),
//     ),
// )]
// #[get("/fed_cm/disconnect")]
// pub async fn post_fed_cm_disconnect(
//     _data: web::Data<AppState>,
// ) -> Result<HttpResponse, ErrorResponse> {
//     is_fed_cm_enabled()?;
//
//     // TODO like it is defined in the spec now, the disconnect endpoint does not really work with
//     // revoking OIDC refresh tokens, since we only get a `client_id` and an `account_id`, but for
//     // these 2 we don't keep any persistent links. Something like an `id_token` hint would be helpful.
//
//     Ok(HttpResponse::Ok().finish())
// }

/// POST ID assertion
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    post,
    path = "/fed_cm/token",
    tag = "fed_cm",
    request_body(
        content = FedCMAssertionRequest,
        content_type = "application/x-www-form-urlencoded"
    ),
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[post("/fed_cm/token")]
#[tracing::instrument(level = "debug", skip_all, fields(client_id = payload.client_id))]
pub async fn post_fed_cm_token(
    req: HttpRequest,
    data: web::Data<AppState>,
    payload: actix_web_validator::Form<FedCMAssertionRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;
    is_web_identity_fetch(&req)?;

    let (login_status, user_id) = login_status_from_req(&data, &req).await;
    if login_status == FedCMLoginStatus::LoggedOut {
        return Ok(HttpResponse::Unauthorized()
            .insert_header(FedCMLoginStatus::LoggedOut.as_header_pair())
            .finish());
    }

    let payload = payload.into_inner();

    // find and check the client
    let client = Client::find_maybe_ephemeral(&data, payload.client_id).await?;
    if !client.enabled {
        debug!("client {} is disabled", client.id);
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("client-disabled".to_string()),
            "This client has been disabled".to_string(),
        ));
    }

    // TODO what about confidential clients? Should we maybe return an auth_code?
    // TODO impl a new `FedCM` flow for client's and reject if not true?

    let origin_header = client_origin_header(&data, &req, &client)?;

    // find and check the user
    let user = User::find_for_fed_cm_validated(&data, user_id).await?;
    if payload.account_id != user.id {
        debug!(
            "payload.account_id != user.id -> {} != {}",
            payload.account_id, user.id
        );
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("invalid-user".to_string()),
            "The `account_id` does not match the `user_id` from the active session".to_string(),
        ));
    }

    // We are good - issue a TokenSet
    let ts = TokenSet::from_user(
        &user,
        &data,
        &client,
        None,
        payload.nonce.map(TokenNonce),
        // TODO add something like `fedcm` to the scopes? Maybe depending on new allowed flow?
        None,
        AuthCodeFlow::No,
        DeviceCodeFlow::No,
    )
    .await?;

    Ok(HttpResponse::Ok()
        .insert_header(HEADER_ALLOW_CREDENTIALS)
        .insert_header(origin_header)
        .insert_header(FedCMLoginStatus::LoggedIn.as_header_pair())
        .json(FedCMTokenResponse {
            token: ts.id_token.unwrap(),
        }))
}

/// The `.well-known` endpoint for FedCM clients
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    get,
    path = "/.well-known/web-identity",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok", body = WebIdentity),
    ),
)]
#[get("/.well-known/web-identity")]
pub async fn get_fed_cm_well_known(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;
    is_web_identity_fetch(&req)?;

    let (login_status, _) = login_status_from_req(&data, &req).await;
    Ok(HttpResponse::Ok()
        .insert_header(login_status.as_header_pair())
        .insert_header(HEADER_JSON)
        .insert_header(HEADER_ALLOW_ALL_ORIGINS)
        .json(WebIdentity::new(&data.issuer)))
}

#[inline(always)]
fn is_fed_cm_enabled() -> Result<(), ErrorResponse> {
    if *EXPERIMENTAL_FED_CM_ENABLE {
        Ok(())
    } else {
        Err(ErrorResponse::new(
            ErrorResponseType::Internal,
            "The FedCM API is disabled on this instance".to_string(),
        ))
    }
}

/// Checks for `Sec-Fetch-Dest: webidentity`
/// Ignores validation of empty origin and referrer headers - not our job
#[inline(always)]
fn is_web_identity_fetch(req: &HttpRequest) -> Result<(), ErrorResponse> {
    if req
        .headers()
        .get("sec-fetch-dest")
        .map(|v| v.to_str().unwrap_or_default())
        == Some("webidentity")
    {
        Ok(())
    } else {
        warn!("`Sec-Fetch-Dest: webidentity` not set`");
        Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Expected header `Sec-Fetch-Dest: webidentity`".to_string(),
        ))
    }
}

fn client_origin_header(
    data: &web::Data<AppState>,
    req: &HttpRequest,
    client: &Client,
) -> Result<(HeaderName, HeaderValue), ErrorResponse> {
    let origin = req
        .headers()
        .get(header::ORIGIN)
        .map(|v| v.to_str().unwrap_or_default())
        .ok_or_else(|| {
            debug!("Origin header is missing");
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Origin header is missing".to_string(),
            )
        })?;
    debug!("Origin header from request: {}", origin);
    let header = (
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_str(origin).unwrap(),
    );

    if client.is_ephemeral() {
        // TODO does this make sense? what if we host the config somewhere else?
        if client.id != origin {
            debug!("client.id != origin -> {} != {}", client.id, origin);
            return Err(ErrorResponse::new(
                ErrorResponseType::WWWAuthenticate("invalid-origin".to_string()),
                "invalid `Origin` header".to_string(),
            ));
        };
    } else {
        if client.allowed_origins.is_none() {
            debug!("Allowed origins is None");
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "The origin is not allowed for this client".to_string(),
            ));
        }

        if let Some(allowed_origins) = &client.allowed_origins {
            for ao in allowed_origins.split(',') {
                debug!("Comparing Allowed Origin '{}' to origin '{}'", ao, origin);
                if (data.listen_scheme == ListenScheme::HttpHttps && ao.ends_with(origin))
                    || ao.eq(origin)
                {
                    return Ok(header);
                }
            }
        }

        // in case we did not have a specific allowed origin, we can validate via allowed
        // `redirect_uri`s
        for uri in client.redirect_uris.split(',') {
            if uri.starts_with(origin) {
                return Ok(header);
            }
        }

        debug!("No match found for allowed origin");
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "The origin is not allowed for this client".to_string(),
        ));
    };

    Ok(header)
}

#[inline(always)]
async fn login_status_from_req(
    data: &web::Data<AppState>,
    req: &HttpRequest,
) -> (FedCMLoginStatus, String) {
    let user_id = match user_id_from_req(req) {
        Ok(uid) => uid,
        Err(_) => return (FedCMLoginStatus::LoggedOut, String::default()),
    };

    match ApiCookie::from_req(req, COOKIE_SESSION_FED_CM) {
        None => {
            debug!(
                "FedCM session cookie not found -> user_id {} is logged-out",
                user_id
            );
            return (FedCMLoginStatus::LoggedOut, user_id);
        }
        Some(sid) => {
            let session = match Session::find(data, sid).await {
                Ok(s) => s,
                Err(_) => {
                    debug!(
                        "FedCM session not found -> user_id {} is logged-out",
                        user_id
                    );
                    return (FedCMLoginStatus::LoggedOut, user_id);
                }
            };
            if !session.is_valid(*SESSION_TIMEOUT_FED_CM, real_ip_from_req(req)) {
                debug!(
                    "FedCM session is invalid -> user_id {} is logged-out",
                    user_id
                );
                return (FedCMLoginStatus::LoggedOut, user_id);
            }

            if session.user_id.as_deref() != Some(&user_id) {
                debug!(
                    "session.user_id.as_deref() != Some(&user_id) -> {:?} != {:?}",
                    session.user_id,
                    Some(&user_id)
                );
                return (FedCMLoginStatus::LoggedOut, user_id);
            }
        }
    };

    (FedCMLoginStatus::LoggedIn, user_id)
}

#[inline(always)]
fn user_id_from_req(req: &HttpRequest) -> Result<String, ErrorResponse> {
    ApiCookie::from_req(req, COOKIE_USER).ok_or_else(|| {
        debug!("Could not extract the RauthyUser cookie");
        ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("user-does-not-exist".to_string()),
            "No Rauthy User cookie found".to_string(),
        )
    })
}
