extern crate rustsf;

use anyhow::Result;
use rustsf::metadata_api::responses::deploy_options::{DeployOptions, TestLevel};
use rustsf::metadata_api::responses::deploy_result::DeploymentStatus;

mod common;

#[tokio::test]
#[ignore]
async fn test_deployment_flow() -> Result<()> {
	let api = common::get_metadata_api_client().await?;

	// Create deployment with an apex class
	let mut request = api.new_deployment_request();
	request.set_options(DeployOptions::new().set_run_tests(TestLevel::NoTestRun).to_owned());
	request.add(
		"ApexClass",
		"classes",
		"RustSfClass.cls",
		b"public class RustSfClass {}",
		Some("RustSfClass.cls-meta.xml"),
		Some(
			r#"<?xml version="1.0" encoding="UTF-8"?>
	<ApexClass xmlns="http://soap.sforce.com/2006/04/metadata">
	    <apiVersion>68.0</apiVersion>
	    <status>Active</status>
	</ApexClass>
	"#
			.as_bytes(),
		),
	)?;

	// Deploy Apex class
	let deploy_response = api.deploy(request).await?;
	let deploy_request_id = deploy_response.get_deploy_request_id();

	// Poll the deployment status
	let mut max_tries = 10;
	let deploy_response = loop {
		let deploy_response = api.deploy_status(deploy_request_id, true).await?;

		if *deploy_response.result().status() == DeploymentStatus::Succeeded {
			break deploy_response;
		}
		if max_tries == 0 {
			panic!("Deploy failed");
		}
		max_tries -= 1;
		tokio::time::sleep(std::time::Duration::from_secs(3)).await;
	};

	let deploy_result = deploy_response.result();
	let details = deploy_result.details().expect("Expected deployment details");
	let components = details.component_successes();

	assert_eq!(components.len(), 2);
	let component = components.into_iter().next().unwrap();
	assert_eq!(component.component_type(), "ApexClass");
	assert_eq!(component.file_name(), "classes/RustSfClass.cls");

	// Create destructive deployment for the apex class
	let mut request = api.new_deployment_request();
	request.set_options(DeployOptions::new().set_run_tests(TestLevel::NoTestRun).to_owned());
	request.delete_pre("ApexClass", vec!["RustSfClass"]);

	// Deploy destructive changes
	let deploy_response = api.deploy(request).await?;
	let deploy_request_id = deploy_response.get_deploy_request_id();

	// Poll the deployment status
	let mut max_tries = 10;
	let deploy_response = loop {
		let deploy_response = api.deploy_status(deploy_request_id, true).await?;

		if deploy_response.result().is_success() {
			break deploy_response;
		}
		if max_tries == 0 {
			panic!("Deploy failed");
		}
		max_tries -= 1;
		tokio::time::sleep(std::time::Duration::from_secs(3)).await;
	};

	let deploy_result = deploy_response.result();
	let details = deploy_result.details().expect("Expected deployment details");
	let components = details.component_successes();

	assert_eq!(components.len(), 2);
	let component = components.into_iter().next().unwrap();
	assert_eq!(component.component_type(), "ApexClass");
	assert_eq!(component.file_name(), "classes/RustSfClass.cls");

	Ok(())
}
