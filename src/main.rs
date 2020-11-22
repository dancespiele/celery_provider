extern crate celery;
extern crate structopt;
use celery::error::CeleryError;
use celery::{broker::AMQPBroker, Celery, TaskResult};
use dotenv::dotenv;
use sled::{Db, Error as DbError, IVec};
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::str;
use tokio::time::{delay_for, Duration};
use warp::{filters::BoxedFilter, Filter};
use warp::{reject, reply, Rejection, Reply};

#[celery::task]
fn send_text(text: String) -> TaskResult<String> {
    Ok(text)
}

fn init_my_app() -> Result<BoxedFilter<(&'static Celery<AMQPBroker>,)>, Box<dyn Error>> {
    let my_app = celery::app!(
        broker = AMQP { std::env::var("AMPQ_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672".into())},
        tasks = [
            send_text,
        ],
        task_routes = [
            "send_text" => "text_queue",
        ]
    );

    Ok(warp::any().map(move || my_app).boxed())
}

pub fn init_tree() -> Result<Db, DbError> {
    let seld_url = env::var("SLED_URL").expect("SLED_URL must be set");
    sled::open(seld_url)
}

async fn send_hello(my_app: &'static Celery<AMQPBroker>) -> Result<impl Reply, Rejection> {
    let response: IVec;
    let mut index = 0;
    let id_message = my_app
        .send_task(send_text::new("hello consumer".to_string()))
        .await
        .map_err(|_| CeleryError::TaskRegistrationError(String::from("unknown task")))
        .unwrap();

    delay_for(Duration::from_millis(100)).await;

    loop {
        let db_result = init_tree();

        if let Ok(db) = db_result {
            let response_result = db.get(id_message.clone());

            if let Ok(response_option) = response_result {
                if let Some(res) = response_option {
                    response = res;
                    break;
                }
            }
        }

        index += 1;

        if index > 10 {
            return Err(reject::not_found());
        }

        delay_for(Duration::from_millis(100)).await;
    }

    let message_body = str::from_utf8(&response).unwrap();

    Ok(reply::json(&message_body))
}

fn hello_filter(
    my_app: impl Filter<Extract = (&'static Celery<AMQPBroker>,), Error = Rejection> + Clone + Send,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("hello")
        .and(warp::get())
        .and(my_app)
        .and_then(send_hello)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let my_app = init_my_app().unwrap();

    let routes = hello_filter(my_app);

    warp::serve(routes).run(addr).await;
}
