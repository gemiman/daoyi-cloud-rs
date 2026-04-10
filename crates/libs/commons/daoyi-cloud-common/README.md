```rust
impl<S, T> FromRequestParts<S> for ValidQuery<T>
where
    S: Send + Sync,
    Valid<Query<T>>: FromRequestParts<S, Rejection=ApiError>,
{
    type Rejection = ApiError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(ValidQuery(
            Valid::from_request_parts(parts, state).await?.0.0,
        ))
    }
}

impl<S, T> FromRequest<S> for ValidJson<T>
where
    S: Send + Sync,
    Valid<Json<T>>: FromRequest<S, Rejection=ApiError>,
{
    type Rejection = ApiError;
    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        Ok(ValidJson(Valid::from_request(request, state).await?.0.0))
    }
}

impl<S, T> FromRequestParts<S> for ValidPath<T>
where
    S: Send + Sync,
    Valid<Path<T>>: FromRequestParts<S, Rejection=ApiError>,
{
    type Rejection = ApiError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(ValidPath(
            Valid::from_request_parts(parts, state).await?.0.0,
        ))
    }
}
```