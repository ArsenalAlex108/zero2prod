pub async fn traverse_result_future<T, E>(
    i: Result<impl Future<Output = T>, E>,
) -> Result<T, E> {
    Ok(i?.await)
}

pub async fn traverse_result_future_result<T, E>(
    i: Result<impl Future<Output = Result<T, E>>, E>,
) -> Result<T, E> {
    traverse_result_future(i).await.and_then(|i| i)
}
