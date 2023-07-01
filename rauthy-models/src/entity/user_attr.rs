use crate::app_state::AppState;
use crate::entity::scopes::Scope;
use crate::entity::users::User;
use crate::request::{UserAttrConfigRequest, UserAttrValuesUpdateRequest};
use actix_web::web;
use rauthy_common::constants::{CACHE_NAME_12HR, DB_TYPE, IDX_USER_ATTR_CONFIG};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::DbType;
use redhac::{cache_del, cache_get, cache_get_from, cache_get_value, cache_put};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use std::collections::HashSet;
use utoipa::ToSchema;

/// Additional custom attributes for users. These can be set for every user and then mapped to a
/// scope, to include them in JWT tokens.
#[derive(Clone, Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct UserAttrConfigEntity {
    /// Unique name
    pub name: String,
    /// Description for the attribute
    pub desc: Option<String>,
}

/// CRUD
impl UserAttrConfigEntity {
    pub async fn create(
        data: &web::Data<AppState>,
        new_attr: UserAttrConfigRequest,
    ) -> Result<Self, ErrorResponse> {
        if Self::find(data, new_attr.name.clone()).await.is_ok() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "User attribute config does already exist".to_string(),
            ));
        }

        match *DB_TYPE {
            DbType::Sqlite => {
                sqlx::query("insert into user_attr_config (name, desc) values ($1, $2)")
            }
            DbType::Postgres => {
                sqlx::query("insert into user_attr_config (name, \"desc\") values ($1, $2)")
            }
        }
        .bind(&new_attr.name)
        .bind(&new_attr.desc)
        .execute(&data.db)
        .await?;

        let mut attrs = UserAttrConfigEntity::find_all(data).await?;
        let slf = Self {
            name: new_attr.name.clone(),
            desc: new_attr.desc.clone(),
        };
        attrs.push(slf.clone());
        cache_put(
            CACHE_NAME_12HR.to_string(),
            IDX_USER_ATTR_CONFIG.to_string(),
            &data.caches.ha_cache_config,
            &attrs,
        )
        .await?;

        Ok(slf)
    }

    pub async fn delete(data: &web::Data<AppState>, name: String) -> Result<(), ErrorResponse> {
        // we do this empty check beforehand to avoid much more unnecessary work if it does not exist anyway
        Self::find(data, name.clone()).await?;

        let mut txn = data.db.begin().await?;

        // delete all possible scope mappings
        let scopes = Scope::find_all(data).await?;
        for s in scopes {
            let mut needs_update = false;

            let attr_include_access = if let Some(access) = s.attr_include_access {
                if access.contains(&name) {
                    needs_update = true;
                    let a = access.replace(&format!("{},", name), "").replace(&name, "");
                    if a.is_empty() {
                        None
                    } else {
                        Some(a)
                    }
                } else {
                    Some(access)
                }
            } else {
                None
            };

            let attr_include_id = if let Some(id) = s.attr_include_id {
                if id.contains(&name) {
                    needs_update = true;
                    let i = id.replace(&format!("{},", name), "").replace(&name, "");
                    if i.is_empty() {
                        None
                    } else {
                        Some(i)
                    }
                } else {
                    Some(id)
                }
            } else {
                None
            };

            if needs_update {
                Scope::update_mapping_only(
                    data,
                    &s.id,
                    attr_include_access,
                    attr_include_id,
                    &mut txn,
                )
                .await?;
            }
        }

        cache_del(
            CACHE_NAME_12HR.to_string(),
            name.clone(),
            &data.caches.ha_cache_config,
        )
        .await?;

        UserAttrValueEntity::delete_all_by_key(data, &name, &mut txn).await?;

        sqlx::query("delete from user_attr_config where name  = $1")
            .bind(&name)
            .execute(&mut txn)
            .await?;

        txn.commit().await?;

        // DATA_STORE.del(Cf::UserAttrConfig, name.clone()).await?;

        let attrs = Self::find_all(data)
            .await?
            .into_iter()
            .filter(|a| a.name != name)
            .collect::<Vec<Self>>();
        cache_put(
            CACHE_NAME_12HR.to_string(),
            IDX_USER_ATTR_CONFIG.to_string(),
            &data.caches.ha_cache_config,
            &attrs,
        )
        .await?;

        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, name: String) -> Result<Self, ErrorResponse> {
        let attr_opt = cache_get!(
            Self,
            CACHE_NAME_12HR.to_string(),
            name.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if attr_opt.is_some() {
            return Ok(attr_opt.unwrap());
        }

        let attr = sqlx::query_as::<_, Self>("select * from user_attr_config where name = $1")
            .bind(&name)
            .fetch_one(&data.db)
            .await?;

        cache_put(
            CACHE_NAME_12HR.to_string(),
            name,
            &data.caches.ha_cache_config,
            &attr,
        )
        .await?;
        Ok(attr)
    }

    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let attrs = cache_get!(
            Vec<Self>,
            CACHE_NAME_12HR.to_string(),
            IDX_USER_ATTR_CONFIG.to_string(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if attrs.is_some() {
            return Ok(attrs.unwrap());
        }

        let res = sqlx::query_as::<_, Self>("select * from user_attr_config")
            .fetch_all(&data.db)
            .await?;

        cache_put(
            CACHE_NAME_12HR.to_string(),
            IDX_USER_ATTR_CONFIG.to_string(),
            &data.caches.ha_cache_config,
            &res,
        )
        .await?;
        Ok(res)
    }

    pub async fn update(
        data: &web::Data<AppState>,
        name: String,
        req_data: UserAttrConfigRequest,
    ) -> Result<Self, ErrorResponse> {
        let mut slf = Self::find(data, name.clone()).await?;

        slf.name = req_data.name.clone();
        slf.desc = req_data.desc;

        let is_name_update = name != req_data.name;

        // collect all current value IDs with the setting for cache clear on success
        let cache_idxs = if is_name_update {
            let idx = sqlx::query_as::<_, UserAttrValueEntity>(
                "select * from user_attr_values where key = $1",
            )
            .bind(&name)
            .fetch_all(&data.db)
            .await?
            .into_iter()
            .map(|a| UserAttrValueEntity::cache_idx(&a.user_id))
            .collect::<Vec<String>>();
            Some(idx)
        } else {
            None
        };

        let mut txn = data.db.begin().await?;

        match *DB_TYPE {
            DbType::Sqlite => {
                sqlx::query("update user_attr_config set name  = $1, desc = $2 where name = $3")
            }
            DbType::Postgres => {
                sqlx::query("update user_attr_config set name  = $1, \"desc\" = $2 where name = $3")
            }
        }
        .bind(&slf.name)
        .bind(&slf.desc)
        .bind(&name)
        .execute(&mut txn)
        .await?;

        if is_name_update {
            // update all possible scope mappings
            let scopes = Scope::find_all(data).await?;
            for s in scopes {
                let mut needs_update = false;

                let attr_include_access = if let Some(access) = s.attr_include_access {
                    if access.contains(&name) {
                        needs_update = true;
                        let a = access.replace(&name, &req_data.name);
                        Some(a)
                    } else {
                        Some(access)
                    }
                } else {
                    None
                };

                let attr_include_id = if let Some(id) = s.attr_include_id {
                    if id.contains(&name) {
                        needs_update = true;
                        let i = id.replace(&name, &req_data.name);
                        Some(i)
                    } else {
                        Some(id)
                    }
                } else {
                    None
                };

                if needs_update {
                    Scope::update_mapping_only(
                        data,
                        &s.id,
                        attr_include_access,
                        attr_include_id,
                        &mut txn,
                    )
                    .await?;
                }
            }
        }

        txn.commit().await?;

        let attrs = Self::find_all(data)
            .await?
            .into_iter()
            .map(|mut attr| {
                if attr.name == name {
                    attr.name = slf.name.clone();
                    attr.desc = slf.desc.clone();
                }
                attr
            })
            .collect::<Vec<Self>>();

        cache_put(
            CACHE_NAME_12HR.to_string(),
            IDX_USER_ATTR_CONFIG.to_string(),
            &data.caches.ha_cache_config,
            &attrs,
        )
        .await?;

        if let Some(idxs) = cache_idxs {
            for idx in idxs {
                cache_del(
                    CACHE_NAME_12HR.to_string(),
                    idx,
                    &data.caches.ha_cache_config,
                )
                .await?;
            }
        }

        Ok(slf)
    }
}

impl UserAttrConfigEntity {
    pub async fn find_all_as_set(
        data: &web::Data<AppState>,
    ) -> Result<HashSet<String>, ErrorResponse> {
        let attrs = Self::find_all(data).await?;

        let mut set = HashSet::with_capacity(attrs.len());
        for a in attrs {
            set.insert(a.name);
        }

        Ok(set)
    }

    pub fn names_hash_set(mut slf: Vec<Self>) -> HashSet<String> {
        let mut res = HashSet::with_capacity(slf.len());
        slf.drain(..).for_each(|s| {
            res.insert(s.name);
        });
        res
    }
}

/// The value for a pre-defined UserAttrConfig with all `serde_json::Value` being valid values.
/// Important: There is no further input validation / restriction
#[derive(Clone, Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct UserAttrValueEntity {
    pub user_id: String,
    pub key: String,
    pub value: Vec<u8>,
}

impl UserAttrValueEntity {
    pub async fn delete_all_by_key(
        data: &web::Data<AppState>,
        key: &str,
        txn: &mut sqlx::Transaction<'_, sqlx::Any>,
    ) -> Result<(), ErrorResponse> {
        let cache_idxs = sqlx::query_as::<_, Self>("select * from user_attr_values where key = $1")
            .bind(key)
            .fetch_all(&data.db)
            .await?
            .into_iter()
            .map(|a| Self::cache_idx(&a.user_id))
            .collect::<Vec<String>>();

        for idx in cache_idxs {
            cache_del(
                CACHE_NAME_12HR.to_string(),
                idx,
                &data.caches.ha_cache_config,
            )
            .await?;
        }

        sqlx::query("delete from user_attr_values where key = $1")
            .bind(key)
            .execute(txn)
            .await?;

        Ok(())
    }

    pub async fn find_for_user(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let idx = Self::cache_idx(user_id);
        let attrs = cache_get!(
            Vec<Self>,
            CACHE_NAME_12HR.to_string(),
            idx.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if attrs.is_some() {
            return Ok(attrs.unwrap());
        }

        let res = sqlx::query_as::<_, Self>("select * from user_attr_values where user_id = $1")
            .bind(user_id)
            .fetch_all(&data.db)
            .await?;

        cache_put(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &res,
        )
        .await?;

        Ok(res)
    }

    pub async fn update_for_user(
        data: &web::Data<AppState>,
        user_id: &str,
        req_data: UserAttrValuesUpdateRequest,
    ) -> Result<Vec<Self>, ErrorResponse> {
        // Not necessary for the operation and correctness, but look up the user first and return
        // an error, if it does not exist at all, for a better user experience.
        User::exists(data, user_id.to_string()).await?;

        for value in req_data.values {
            let del = if let Some(s) = value.value.as_str() {
                s.is_empty()
            } else {
                false
            };

            if del || value.value == Value::Null {
                sqlx::query("delete from user_attr_values where user_id = $1 and key = $2")
                    .bind(user_id)
                    .bind(&value.key)
                    .execute(&data.db)
                    .await?;
            } else {
                match *DB_TYPE {
                    DbType::Sqlite => sqlx::query(
                        r#"insert or replace into user_attr_values (user_id, key, value)
                        values ($1, $2, $3)"#,
                    ),
                    DbType::Postgres => sqlx::query(
                        r#"insert into user_attr_values (user_id, key, value)
                        values ($1, $2, $3)
                        on conflict(user_id, key) do update set value = $3"#,
                    ),
                }
                .bind(user_id)
                .bind(&value.key)
                .bind(serde_json::to_vec(&value.value).unwrap())
                .execute(&data.db)
                .await?;
            }
        }

        // 2nd query again to have more compatibility
        let res = sqlx::query_as::<_, Self>("select * from user_attr_values where user_id = $1")
            .bind(user_id)
            .fetch_all(&data.db)
            .await?;

        let idx = Self::cache_idx(user_id);
        cache_put(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &res,
        )
        .await?;

        Ok(res)
    }
}

impl UserAttrValueEntity {
    fn cache_idx(user_id: &str) -> String {
        format!("{}{}", IDX_USER_ATTR_CONFIG, user_id)
    }
}