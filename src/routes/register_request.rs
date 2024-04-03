use axum::extract::{Path, State};
use compositor_pipeline::pipeline::Port;

use crate::{
    state::{ApiState, Pipeline, Response},
    error::ApiError,
    routes::Json,
    types::{
        ImageSpec, InputId, Mp4, OutputId, RegisterOutputRequest, RendererId, RtpInputStream,
        ShaderSpec, WebRendererSpec,
    },
};

pub(super) async fn handle_rtp_input_stream(
    State(api): State<ApiState>,
    Path(input_id): Path<InputId>,
    Json(request): Json<RtpInputStream>,
) -> Result<Response, ApiError> {
    let api = api.clone();
    tokio::task::spawn_blocking(move || {
        let result = Pipeline::register_input(&api.pipeline, input_id.into(), request.try_into()?)?;
        match result {
            Some(Port(port)) => Ok(Response::RegisteredPort { port }),
            None => Ok(Response::Ok {}),
        }
    })
    .await
    // `unwrap()` panics only when the task panicked or `response.abort()` was called
    .unwrap()
}

pub(super) async fn handle_mp4(
    State(api): State<ApiState>,
    Path(input_id): Path<InputId>,
    Json(request): Json<Mp4>,
) -> Result<Response, ApiError> {
    let api = api.clone();
    tokio::task::spawn_blocking(move || {
        let result = Pipeline::register_input(&api.pipeline, input_id.into(), request.try_into()?)?;
        match result {
            Some(Port(port)) => Ok(Response::RegisteredPort { port }),
            None => Ok(Response::Ok {}),
        }
    })
    .await
    .unwrap()
}

pub(super) async fn handle_rtp_output_stream(
    State(api): State<ApiState>,
    Path(output_id): Path<OutputId>,
    Json(request): Json<RegisterOutputRequest>,
) -> Result<Response, ApiError> {
    let api = api.clone();
    tokio::task::spawn_blocking(move || {
        let result = api
            .pipeline()
            .register_output(output_id.into(), request.try_into()?)?;
        match result {
            Some(Port(port)) => Ok(Response::RegisteredPort { port }),
            None => Ok(Response::Ok {}),
        }
    })
    .await
    .unwrap()
}

pub(super) async fn handle_shader(
    State(api): State<ApiState>,
    Path(shader_id): Path<RendererId>,
    Json(request): Json<ShaderSpec>,
) -> Result<Response, ApiError> {
    let api = api.clone();
    tokio::task::spawn_blocking(move || {
        Pipeline::register_renderer(&api.pipeline, shader_id.into(), request.try_into()?)?;
        Ok(Response::Ok {})
    })
    .await
    .unwrap()
}

pub(super) async fn handle_web_renderer(
    State(api): State<ApiState>,
    Path(instance_id): Path<RendererId>,
    Json(request): Json<WebRendererSpec>,
) -> Result<Response, ApiError> {
    let api = api.clone();
    tokio::task::spawn_blocking(move || {
        Pipeline::register_renderer(&api.pipeline, instance_id.into(), request.try_into()?)?;
        Ok(Response::Ok {})
    })
    .await
    .unwrap()
}

pub(super) async fn handle_image(
    State(api): State<ApiState>,
    Path(image_id): Path<RendererId>,
    Json(request): Json<ImageSpec>,
) -> Result<Response, ApiError> {
    let api = api.clone();
    tokio::task::spawn_blocking(move || {
        Pipeline::register_renderer(&api.pipeline, image_id.into(), request.try_into()?)?;
        Ok(Response::Ok {})
    })
    .await
    .unwrap()
}
