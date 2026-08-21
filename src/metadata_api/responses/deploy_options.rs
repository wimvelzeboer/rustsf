//! # Module containing the structs for deployment options

#[derive(Debug, serde::Deserialize, serde::Serialize, Default, Clone)]
pub struct DeployOptions {

    /// If files that are specified in package.xml aren’t in the .zip file,
    /// specifies whether a deployment can still succeed. Don’t set this argument
    /// for deployment to production orgs.
    #[serde(rename = "allowMissingFiles", default)]
    allow_missing_files : bool,

    /// Reserved for future use.
    #[serde(rename = "autoUpdatePackage", default)]
    auto_update_package : bool,

    /// Defaults to false. Set to true to perform a test deployment (validation) of components
    /// without saving the components in the target org. A validation enables you to verify
    /// the results of tests that would be generated in a deployment, but doesn’t commit any changes.
    /// After a validation finishes with passing tests, it can qualify for deployment without
    /// rerunning tests. See Deploy a Recently Validated Component Set Without Tests.
    #[serde(rename = "checkOnly")]
    check_only : bool,

    /// Indicates whether a deployment is allowed to complete successfully despite one or more
    /// warnings (true) or not (false). Defaults to false.
    /// The DeployMessage object for a warning contains the following values:
    /// - problemType—Warning
    /// - problem—The text of the warning.
    ///
    /// If a warning occurs and ignoreWarnings is set to true, the success field in DeployMessage
    /// is true. If ignoreWarnings is set to false, success is set to false and the warning
    /// is treated like an error.
    #[serde(rename = "ignoreWarnings")]
    ignore_warnings : bool,

    /// Reserved for future use.
    #[serde(rename = "performRetrieve")]
    perform_retrieve : bool,

    /// If true, the deleted components in the destructiveChanges.xml manifest file aren't stored
    /// in the Recycle Bin. Instead, they become immediately eligible for deletion.
    ///
    /// # Note:
    /// This option only works in Developer Edition or sandbox orgs. It doesn't work in production orgs.
    #[serde(rename = "purgeOnDelete")]
    purge_on_delete : bool,

    ///  Indicates whether any failure causes a complete rollback (true) or not (false).
    /// If false, whatever actions can be performed without errors are performed, and errors are
    /// returned for the remaining actions. This parameter must be set to true if you’re deploying
    /// to a production org. The default is false.
    #[serde(rename = "rollbackOnError")]
    rollback_on_error : bool,

    /// A list of Apex tests to run during deployment. Specify the class name, one name per instance.
    /// The class name can also specify a namespace with a dot notation
    ///
    /// # Note
    /// To use this option, set testLevel to RunSpecifiedTests.
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_meta.meta/api_meta/meta_deploy_run_specific_tests.htm>
    #[serde(rename = "runTests")]
    run_tests : Option<Vec<String>>,

    /// Indicates whether the specified .zip file points to a directory structure with a
    /// single package (true) or a set of packages (false).
    #[serde(rename = "singlePackage")]
    single_package : bool,

    /// Specifies which tests are run as part of a deployment.
    /// The test level is enforced regardless of the types of components that are present in
    /// the deployment package.
    ///
    /// # See
    /// [TestLevel]
    #[serde(rename = "testLevel")]
    test_level : Option<TestLevel>,
}

impl DeployOptions {
    pub fn new() -> Self {
        Self {
            allow_missing_files: false,
            auto_update_package: false,
            check_only: false,
            ignore_warnings: false,
            perform_retrieve: false,
            purge_on_delete: false,
            rollback_on_error: true,
            run_tests: None,
            single_package: true,
            test_level: Some(TestLevel::RunLocalTests),
        }
    }
    pub fn set_run_tests(&mut self, test_level: TestLevel) -> &mut Self {
        self.test_level = Some(test_level);
        self
    }
}

/// Specifies the available test levels
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum TestLevel {

    /// No tests are run. This test level applies only to deployments to development environments,
    /// such as sandbox, Developer Edition, or trial organizations.
    /// This test level is the default for development environments.
    NoTestRun,

    /// Only the tests that you specify in the runTests option are run. Code coverage requirements
    /// differ from the default coverage requirements when using this test level.
    /// Each class and trigger in the deployment package must be covered by the executed tests
    /// for a minimum of 75% code coverage. This coverage is computed for each class and triggers
    /// individually and is different than the overall coverage percentage.
    RunSpecifiedTests,

    /// Only tests relevant to the deployment payload are run. Salesforce automatically identifies
    /// the relevant tests based on an analysis of the deployment payload and the payload dependencies.
    /// For fine-grained control, you can annotate test classes so that they either run regardless
    /// of the deployment payload, or run when modified, referenced components are included in the
    /// deployment. See @IsTest Annotation in the Apex Developer Guide. Each class and trigger in
    /// the deployment package must be covered by the executed tests for a minimum of 75% code
    /// coverage. This coverage is computed for each class and trigger individually and is
    /// different from the overall coverage percentage.
    ///
    /// # Note
    /// This is a BETA feature
    RunRelevantTests,

    /// All tests in your org are run, except the ones that originate from installed managed and
    /// unlocked packages. This test level is the default for production deployments that include
    /// Apex classes or triggers.
    RunLocalTests,

    /// All tests are run. The tests include all tests in your org, including tests of managed packages.
    RunAllTestsInOrg,
}