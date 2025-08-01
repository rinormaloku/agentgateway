use std::fmt::{Display, Formatter};
use std::string::String;
use std::{env, fmt};

const BUILD_VERSION: &str = env!("AGENTGATEWAY_BUILD_buildVersion");
const BUILD_GIT_REVISION: &str = env!("AGENTGATEWAY_BUILD_buildGitRevision");
const BUILD_STATUS: &str = env!("AGENTGATEWAY_BUILD_buildStatus");
const BUILD_TAG: &str = env!("AGENTGATEWAY_BUILD_buildTag");
const BUILD_RUST_VERSION: &str = env!("AGENTGATEWAY_BUILD_RUSTC_VERSION");
const BUILD_RUST_PROFILE: &str = env!("AGENTGATEWAY_BUILD_PROFILE_NAME");

#[derive(serde::Serialize, Clone, Debug, Default)]
pub struct BuildInfo {
	pub version: String,
	pub git_revision: String,
	pub rust_version: String,
	pub build_profile: String,
	pub build_status: String,
	pub git_tag: String,
}

impl BuildInfo {
	pub fn new() -> Self {
		BuildInfo {
			version: BUILD_VERSION.to_string(),
			git_revision: BUILD_GIT_REVISION.to_string(),
			rust_version: BUILD_RUST_VERSION.to_string(),
			build_profile: BUILD_RUST_PROFILE.to_string(),
			build_status: BUILD_STATUS.to_string(),
			git_tag: BUILD_TAG.to_string(),
		}
	}
}

impl Display for BuildInfo {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"version.BuildInfo{{RustVersion:\"{}\", BuildProfile:\"{}\", BuildStatus:\"{}\", GitTag:\"{}\", Version:\"{}\", GitRevision:\"{}\"}}",
			self.rust_version,
			self.build_profile,
			self.build_status,
			self.git_tag,
			self.version,
			self.git_revision
		)
	}
}
