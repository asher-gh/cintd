/* TODO:
 * [x] Set up multi-threaded async server
 * [x] connect to DB
 * [x] parse request params to query the DB
 * [ ] Set up routes to schedule a new job
*/

mod prisma;

use anyhow::Result;
use axum::body::Bytes;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use axum::{
	http::StatusCode,
	routing::{get, post},
	Router,
};
use prisma::PrismaClient;
use prisma_client_rust::NewClientError;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};

use tracing::{debug, error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Clone)]
pub struct ServerConfig {
	pub host: String,
	pub port: String,
	pub prisma_client: Arc<Result<PrismaClient, NewClientError>>,
}

#[tokio::main]
async fn main() -> Result<()> {
	let server_config = ServerConfig {
		host: "127.0.0.1".into(),
		port: "3000".into(),
		prisma_client: Arc::new(PrismaClient::_builder().build().await),
	};

	// initialize tracing
	let subscriber = FmtSubscriber::builder()
		.with_max_level(Level::TRACE)
		.finish();
	tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

	run_scheduler(JobScheduler::new().await?).await?;

	let addr: SocketAddr = format!("{}:{}", server_config.host, server_config.port)
		.parse()
		.unwrap();

	// all routes have access the server_config
	let app = Router::new()
		.route("/", get(|| async { "Hello, world!\n" }))
		.route("/echo", post(echo))
		.route("/user", get(get_user))
		.with_state(server_config);

	debug!("listening on {}", addr);

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await?;

	Ok(())
}

async fn get_user(
	State(server_config): State<ServerConfig>,
	Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
	let prisma_client = server_config.prisma_client.clone();
	let prisma_client = prisma_client.as_ref().clone().as_ref().unwrap();

	if let Some(user_id) = params.get("id") {
        debug!("user_id: {}", user_id);
		let user_id = user_id.to_string();

		let posts: Option<prisma::user::Data> = prisma_client
			.user()
			.find_first(vec![prisma::user::id::equals(user_id)])
			.exec()
			.await
			.unwrap();

		if let Some(post) = posts {
			return Json(post).into_response();
		} else {
			return StatusCode::NOT_FOUND.into_response();
		}
	} else {
		return StatusCode::BAD_REQUEST.into_response();
	};
}

async fn echo(body: Bytes) -> Result<String, StatusCode> {
	if let Ok(string) = String::from_utf8(body.to_vec()) {
		Ok(string)
	} else {
		Err(StatusCode::BAD_REQUEST)
	}
}

pub async fn run_scheduler(mut sched: JobScheduler) -> Result<()> {
	// for graceful shutdown using interrupt signal
	sched.set_shutdown_handler(Box::new(|| {
		Box::pin(async move {
			info!("Shut down done");
		})
	}));

	// adding a scheduled job to run every 4 seconds
	sched
		.add(
			Job::new_async("4 * * * * *", |_uuid, _l| {
				Box::pin(async move {
					info!("I run async on the 4th second of every minute");
					// let next_tick = l.next_tick_for_job(uuid).await;
					// match next_tick {
					// 	Ok(Some(ts)) => info!("Next time for 4s is {:?}", ts),
					// 	_ => warn!("Could not get next tick for 4s job"),
					// }
				})
			})
			.unwrap(),
		)
		.await?;

	let start = sched.start().await;

	if start.is_err() {
		error!("Error starting scheduler");
		return Ok(());
	}
	Ok(())
}
