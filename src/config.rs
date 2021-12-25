use std::env;

pub struct Config {
  database_name: String,
  mongodb_uri: String
}

impl Config {
  pub fn new() -> Config {
    Config {
      mongodb_uri: env::var("MONGODB_URI").expect("MONGODB_URI env var should be specified"),
      database_name: env::var("DATABASE_NAME").expect("DATABASE env var should be specified")
    }
  }

  pub fn database_name(&self) -> &String {
    &self.database_name
  }

  pub fn mongodb_uri(&self) -> &String {
    &self.mongodb_uri
  }
}

#[cfg(test)]
mod tests {
  use std::env;
  use crate::config::Config;

  #[test]
  fn new_return_config_with_env_vars() {
    let uri = "SOME URI";
    let database_name = "SOME DATABASE_NAME";

    env::set_var("MONGODB_URI", uri);
    env::set_var("DATABASE_NAME", database_name);

    let config = Config::new();

    assert_eq!(config.database_name(), database_name);
    assert_eq!(config.mongodb_uri(), uri);
  }

  #[test]
  #[should_panic]
  fn panic_if_there_no_database_name_in_env_vars() {
    dotenv::from_filename(".env.test").ok();

    env::remove_var("DATABASE_NAME");

    Config::new();
  }

  #[test]
  #[should_panic]
  fn panic_if_there_no_mongodb_uri_in_env_vars() {
    dotenv::from_filename(".env.test").ok();
    
    env::remove_var("MONGODB_URI");

    Config::new();
  }
}