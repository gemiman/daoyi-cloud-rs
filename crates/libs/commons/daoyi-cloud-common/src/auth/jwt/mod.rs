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
static REFRESH_EXPIRATION_SECS: u64 = 7 * 24 * 3600; // 7 days

/// JWT 主体信息（含 RBAC）
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Principal {
    pub tenant_id: i64,
    pub id: i64,
    pub name: String,
    /// 角色列表，如 `["admin", "user"]`
    #[serde(default)]
    pub roles: Vec<String>,
    /// 权限列表，如 `["user:create", "user:delete"]`
    #[serde(default)]
    pub permissions: Vec<String>,
}

impl Principal {
    /// 检查是否拥有指定权限
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|p| p == permission)
    }

    /// 检查是否拥有指定角色
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    jti: String,
    sub: String,
    aud: String,
    iss: String,
    iat: u64,
    exp: u64,
    /// 刷新令牌的 jti（仅在 access token 中存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    rtj: Option<String>,
    /// 角色列表
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    roles: Vec<String>,
    /// 权限列表
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    perms: Vec<String>,
}

#[derive(Debug)]
pub struct JwtConfig {
    pub secret: Cow<'static, str>,
    pub expiration: Duration,
    pub audience: String,
    pub issuer: String,
}

impl JwtConfig {
    pub fn from_config() -> Self {
        let auth_config = conf::get().auth();
        let secret = if !auth_config.jwt.secret.is_empty() {
            auth_config.jwt.secret.clone()
        } else {
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

    /// 编码 Access Token (含 RBAC claims)
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
            rtj: None,
            roles: principal.roles,
            perms: principal.permissions,
        };
        Ok(jsonwebtoken::encode(
            &self.header,
            &claims,
            &self.encode_secret,
        )?)
    }

    /// 编码 Refresh Token（长期，仅含基本标识）
    pub fn encode_refresh(&self, principal: &Principal) -> anyhow::Result<String> {
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
            exp: current_timestamp.saturating_add(REFRESH_EXPIRATION_SECS),
            rtj: None,
            roles: vec![],
            perms: vec![],
        };
        Ok(jsonwebtoken::encode(
            &self.header,
            &claims,
            &self.encode_secret,
        )?)
    }

    /// 解码 Access Token（返回 Principal + jti）
    pub fn decode(&self, token: &str) -> anyhow::Result<Principal> {
        let claims: Claims =
            jsonwebtoken::decode(token, &self.decode_secret, &self.validation)?.claims;
        let mut parts = claims.sub.splitn(3, ':');
        let principal = Principal {
            tenant_id: parts.next().unwrap().parse::<i64>()?,
            id: parts.next().unwrap().parse::<i64>()?,
            name: parts.next().unwrap().to_string(),
            roles: claims.roles,
            permissions: claims.perms,
        };
        Ok(principal)
    }

    /// 解码 Refresh Token（仅提取 subject）
    pub fn decode_refresh(&self, token: &str) -> anyhow::Result<(String, u64)> {
        let validation = self.validation.clone();
        // Refresh Token 过期时间更长，单独校验
        let data = jsonwebtoken::decode::<Claims>(token, &self.decode_secret, &validation)?;
        Ok((data.claims.sub, data.claims.exp))
    }
}

pub fn default_jwt() -> &'static JWT {
    DEFAULT_JWT.get_or_init(|| {
        let config = JwtConfig::from_config();
        JWT::new(config)
    })
}

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
