use crate::response::ApiResponse;
use salvo::http::StatusCode;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("服务器迷路了~")]
    NotFound,
    #[error("请求方法不被允许")]
    MethodNotAllowed,
    #[error("{0}")]
    Biz(String),
    #[error("错误: {0}")]
    Internal(#[from] anyhow::Error),
    #[error("数据库异常: {0}")]
    DbErr(#[from] sea_orm::DbErr),
    #[error("参数校验失败: {0}")]
    Validation(String),
    #[error("密码错误: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("认证失败：{0}")]
    JWT(#[from] jsonwebtoken::errors::Error),
    #[error("未授权：{0}")]
    Unauthenticated(String),
    #[error("Glob异常: {0}")]
    Glob(#[from] wax::BuildError),
}

/// 将 validator 的 ValidationErrors 格式化为易读的字段级错误信息
pub fn format_validation_errors(errors: &validator::ValidationErrors) -> String {
    format_to_vec(errors).join("; ")
}

fn format_to_vec(errors: &validator::ValidationErrors) -> Vec<String> {
    use validator::ValidationErrorsKind;
    errors
        .errors()
        .iter()
        .flat_map(|(field, errors_kind)| match errors_kind {
            ValidationErrorsKind::Field(field_errors) => field_errors
                .iter()
                .map(|error| {
                    let message = error
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "验证失败".to_string());
                    format!("[{}]{}", field, message)
                })
                .collect::<Vec<_>>(),
            ValidationErrorsKind::Struct(struct_errors) => format_to_vec(struct_errors),
            ValidationErrorsKind::List(list_errors) => list_errors
                .iter()
                .flat_map(|(_index, errors)| format_to_vec(errors))
                .collect::<Vec<_>>(),
        })
        .collect()
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Biz(_) => StatusCode::OK,
            ApiError::Internal(_)
            | ApiError::DbErr(_)
            | ApiError::Bcrypt(_)
            | ApiError::Glob(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Validation(_) => StatusCode::BAD_REQUEST,
            ApiError::JWT(_) | ApiError::Unauthenticated(_) => StatusCode::UNAUTHORIZED,
        }
    }

    pub fn to_api_response(&self) -> ApiResponse<()> {
        ApiResponse::<()>::err_msg(self.to_string())
    }
}
