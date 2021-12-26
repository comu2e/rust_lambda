use ::lib::handler;
use ::lib::LambdaError;
use lambda::handler_fn;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
  println!("execute bootstrap#main");
  let runtime_handler = handler_fn(handler);
  lambda::run(runtime_handler).await?;
  Ok(())
}
