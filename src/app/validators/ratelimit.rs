use rocket_governor::{Method, Quota, RocketGovernable};

use crate::AppConfig;

pub struct RateLimitGuard {}

impl<'r> RocketGovernable<'r> for RateLimitGuard {
  fn quota(_method: Method, route_name: &str) -> Quota {
    let config = AppConfig::get();
    match route_name {
      "upload_route" => Quota::per_minute(Self::nonzero(config.rpm_upload)),
      "view_route" => Quota::per_minute(Self::nonzero(config.rpm_view)),
      "download_route" => Quota::per_minute(Self::nonzero(config.rpm_view)),
      _ => Quota::per_second(Self::nonzero(3u32)),
    }
  }
}
