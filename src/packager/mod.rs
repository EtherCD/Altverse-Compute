use crate::proto::Package;
use crate::resources::utils::input::Input;

pub struct NetworkClient {
  pub packages: Vec<Package>,
  pub input: Input,
}
