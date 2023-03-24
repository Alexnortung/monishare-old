pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterWithEmail>,
) -> Result<String, StatusCode> {
    let db = &state.db;
    let val = match db
        .query_required_single::<i64, _>("select 5 + 6", &())
        .await
    {
        Ok(val) => val,
        Err(e) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(val.to_string())
}
