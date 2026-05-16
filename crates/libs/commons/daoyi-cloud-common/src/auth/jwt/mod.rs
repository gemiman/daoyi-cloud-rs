pub mod middleware;

use crate::conf;
use crate::utils::id_utils;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp,
};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::OnceLock;
use std::time::Duration;

static DEFAULT_JWT: OnceLock<JWT> = OnceLock::new();

/// JWT 主体信息
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Principal {
    /// 租户ID
    pub tenant_id: i64,
    /// 用户ID
    pub id: i64,
    /// 用户姓名
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    jti: String,
    sub: String,
    aud: String,
    iss: String,
    iat: u64,
    exp: u64,
}

#[derive(Debug)]
pub struct JwtConfig {
    pub secret: Cow<'static, str>,
    pub expiration: Duration,
    pub audience: String,
    pub issuer: String,
}

impl JwtConfig {
    /// 从配置系统创建 JwtConfig
    pub fn from_config() -> Self {
        let auth_config = conf::get().auth();
        let secret = if !auth_config.jwt.secret.is_empty() {
            auth_config.jwt.secret.clone()
        } else {
            // 回退到环境变量
            std::env::var("APP_AUTH_JWT_SECRET").unwrap_or_default()
        };
        Self {
            secret: Cow::Owned(secret),
            expiration: Duration::from_secs(auth_config.jwt.expiration_secs),
            audience: auth_config.jwt.audience.clone(),
            issuer: auth_config.jwt.issuer.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JWT {
    encode_secret: EncodingKey,
    decode_secret: DecodingKey,
    header: Header,
    validation: Validation,
    expiration: Duration,
    audience: String,
    issuer: String,
}

impl JWT {
    pub fn new(config: JwtConfig) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[&config.audience]);
        validation.set_issuer(&[&config.issuer]);
        validation.set_required_spec_claims(&["jti", "sub", "aud", "iss", "iat", "exp"]);
        let secret = config.secret.as_bytes();
        Self {
            encode_secret: EncodingKey::from_secret(secret),
            decode_secret: DecodingKey::from_secret(secret),
            header: Header::new(Algorithm::HS256),
            validation,
            expiration: config.expiration,
            audience: config.audience,
            issuer: config.issuer,
        }
    }

    pub fn encode(&self, principal: Principal) -> anyhow::Result<String> {
        let current_timestamp = get_current_timestamp();
        let claims = Claims {
            jti: id_utils::xid(),
            sub: format!(
                "{}:{}:{}",
                principal.tenant_id, principal.id, principal.name
            ),
            aud: self.audience.clone(),
            iss: self.issuer.clone(),
            iat: current_timestamp,
            exp: current_timestamp.saturating_add(self.expiration.as_secs()),
        };
        Ok(jsonwebtoken::encode(
            &self.header,
            &claims,
            &self.encode_secret,
        )?)
    }

    pub fn decode(&self, token: &str) -> anyhow::Result<Principal> {
        let claims: Claims =
            jsonwebtoken::decode(token, &self.decode_secret, &self.validation)?.claims;
        let mut parts = claims.sub.splitn(3, ':');
        let principal = Principal {
            tenant_id: parts.next().unwrap().parse::<i64>()?,
            id: parts.next().unwrap().parse::<i64>()?,
            name: parts.next().unwrap().to_string(),
        };
        Ok(principal)
    }
}

/// 获取全局 JWT 实例（从配置系统懒加载初始化）
pub fn default_jwt() -> &'static JWT {
    DEFAULT_JWT.get_or_init(|| {
        let config = JwtConfig::from_config();
        JWT::new(config)
    })
}

/// 在应用启动时初始化 JWT（提前校验配置是否有效）
pub fn init_from_config() -> anyhow::Result<()> {
    let config = JwtConfig::from_config();
    if config.secret.is_empty() {
        anyhow::bail!(
            "JWT secret 未配置！请在配置文件中设置 auth.jwt.secret，或通过环境变量 APP_AUTH_JWT_SECRET 设置"
        );
    }
    let jwt = JWT::new(config);
    DEFAULT_JWT
        .set(jwt)
        .map_err(|_| anyhow::anyhow!("JWT 已初始化，重复调用 init_from_config"))
}
