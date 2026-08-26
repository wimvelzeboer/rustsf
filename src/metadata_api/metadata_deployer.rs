//! # Salesforce Metadata Deployer
//!
//! The Metadata Deployer is responsible for packaging the metadata
//! so that it is ready to be deployed to Salesforce.
//!
//! It handles the creation of the deployment zip file, as well as the generation of the
//! package.xml files.
//!
//! ## Supported Endpoints:
//! - **/services/data/vXX.X/metadata/deployRequest**, for deployments, status checks, and cancellation
//!

use std::collections::HashMap;
use std::io::Cursor;
use s_zip::{SZipError, StreamingZipWriter};
use crate::{Client};
use crate::metadata_api::{add_file_to_package, generate_package_xml};
use crate::metadata_api::responses::deploy_options::DeployOptions;
use crate::metadata_api::responses::deploy_request::DeployRequest;


/**
fix me refactor s-zip into zip

use std::io::Write;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;
use std::io::Cursor;

fn create_zip_in_memory() -> Vec<u8> {
    // 1. Create an in-memory buffer
    let mut buf = Cursor::new(Vec::new());

    // 2. Initialize the ZipWriter
    let mut zip = ZipWriter::new(&mut buf);

    // 3. Add a file with options (e.g., compression method)
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflate);
    zip.start_file("hello.txt", options).unwrap();

    // 4. Write the in-memory data to the zip entry
    zip.write_all(b"Hello, World!").unwrap();

    // 5. Finalize the archive
    zip.finish().unwrap();

    // 6. Retrieve the resulting bytes
    buf.into_inner()
}
*/


/// A `RestApi` struct that represents the core component for reparing a deployment
pub struct MetadataDeployer {
    version: String,
    options: DeployOptions,
    writer: StreamingZipWriter<Cursor<Vec<u8>>>,
    package: HashMap<String, Vec<String>>,
    pre_destructive: HashMap<String, Vec<String>>,
    post_destructive: HashMap<String, Vec<String>>,
}

impl MetadataDeployer {
    ///
    /// Creates a new instance of `MetadataDeployer`.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to a `Client` instance used for initializing the `MetadataDeployer`.
    ///   The `Client` provides the Salesforce version number through its `version_number` method.
    ///
    /// # Returns
    ///
    /// Returns a newly instantiated `MetadataDeployer` object.
    /// The following components are initialized:
    /// - `version`: The version number retrieved from the client. Defaults to `"67.0"` if unable to fetch the version.
    /// - `options`: A new instance of `DeployOptions`.
    /// - `writer`: A `StreamingZipWriter` with a cursor-based underlying buffer for writing deployment packages.
    /// - `package`, `pre_destructive`, `post_destructive`: Empty `HashMap` instances to hold deployment metadata.
    ///
    /// # Example
    ///
    /// ```
    /// let client= Client::new(Credentials::new()).await?;
    /// let deployer = MetadataDeployer::new(&client);
    /// ```
    pub fn new(client: &Client) -> Self {
        MetadataDeployer {
            version: client.version_number().unwrap_or("67.0".to_string()),
            options: DeployOptions::new(),
            writer: StreamingZipWriter::from_writer(Cursor::new(Vec::new())).unwrap(),
            package: HashMap::new(),
            pre_destructive: HashMap::new(),
            post_destructive: HashMap::new(),
        }
    }

    /// Sets the deployment options for the current instance.
    ///
    /// This method allows you to configure the deployment behavior by updating the
    /// `options` field with the provided `DeployOptions` instance.
    ///
    /// # Arguments
    ///
    /// * `options` - A `DeployOptions` struct containing the desired configuration
    ///               for deployment.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut deploy_instance = DeployManager::new();
    /// let new_options = DeployOptions::default();
    /// deploy_instance.set_options(new_options);
    /// ```
    ///
    /// After calling this method, the instance will use the newly provided
    /// deployment options.
    ///
    /// # Notes
    ///
    /// This method requires a mutable reference to the instance, as it modifies
    /// the internal state.
    pub fn set_options(&mut self, options: DeployOptions) {
        self.options = options;
    }

    /// Adds a file and its corresponding metadata file to the deployment zip
    /// and registers the file in the deployment package.
    ///
    /// # Parameters
    /// - `metadata_type`: The type of Salesforce metadata (e.g., `"ApexClass"`).
    /// - `folder`: The directory where the file should be stored (e.g., `"classes"`).
    /// - `file_name`: The name of the file to add (e.g., `"MyClass.cls"`).
    /// - `content`: The content of the file being added.
    /// - `file_name_xml`: (optional) The name of an associated XML metadata file (e.g., `"MyClass.cls-meta.xml"`).
    /// - `content_xml`: (optional) The content of the associated XML metadata file.
    ///
    /// # Returns
    /// - `Result<(), Error>`: Returns `Ok(())` on success, or an `Error` variant if any operation (e.g., writing to the zip) fails.
    ///
    /// # Behavior
    /// - Adds the main file to the deployment zip in the specified folder.
    /// - Optionally adds the corresponding XML metadata file to the deployment zip if both `file_name_xml` and `content_xml` are provided.
    /// - Registers the file name (without the extension) in the deployment package under the specified `metadata_type`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Credentials, MetadataApi};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    ///
    ///     let mut api = MetadataApi::new(client);
    ///     let mut request = api.new_deployment_request();
    ///     request.add(
    ///         "ApexClass",
    ///         "classes",
    ///         "MyClass.cls",
    ///         b"public class MyClass {}",
    ///         Some("MyClass.cls-meta.xml"),
    ///         Some(r#"<?xml version="1.0" encoding="UTF-8"?>
    /// <ApexClass xmlns="http://soap.sforce.com/2006/04/metadata">
    ///     <apiVersion>66.0</apiVersion>
    ///     <status>Active</status>
    /// </ApexClass>
    /// "#.as_bytes()))?;
    ///
    ///     let response = api.deploy(request).await?;
    ///     println!("Deploy response {:?}", response);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// **Note:**
    /// - The folder structure (`folder`) should coincide with Salesforce metadata conventions.
    /// - The code assumes that the file name has an extension and leverages `split(".").next()`—a `fixme` note is included as this could panic if the file name is incorrectly formatted.
    ///
    pub fn add(
        &mut self,
        metadata_type: &str,         // e.g. "ApexClass"
        folder: &str, // e.g. "classes"  todo - should should be queried from salesforce
        file_name: &str, // e.g. "MyClass.cls"
        content: &[u8], //
        file_name_xml: Option<&str>, // e.g. "MyClass.cls-meta.xml"
        content_xml: Option<&[u8]>, //
    ) -> Result<&mut Self> {
        self.add_file_to_zip(folder, file_name, content)
            .map_err(|mut e| Error::RequestError(e.to_string()))?;

        // Optionally add the xml file to the zip
        match (file_name_xml, content_xml) {
            (Some(file_name_xml), Some(content_xml)) => {
                self.add_file_to_zip(folder, file_name, content)
                    .map_err(|mut e| Error::RequestError(e.to_string()))?;

                self.add_file_to_zip(folder, file_name_xml, content_xml)
                    .map_err(|mut e| Error::RequestError(e.to_string()))?;
            }
            _ => {}
        }

        add_file_to_package(&mut self.package, metadata_type, file_name);

        Ok(self)
    }

    /// Deletes one or more metadata objects after the deplopment is completed,
    /// by adding them to a "destructive" package file.
    ///
    /// # Parameters
    /// * `metadata_type`: The type of metadata (e.g., "CustomObject", "ApexClass") that is being deleted.
    /// * `objects`: The names of the metadata objects to be deleted.
    ///
    /// # Returns
    /// A mutable reference to `self`, allowing for method chaining.
    ///
    /// # Behavior
    /// For each object name provided in the `objects` vector, this function adds
    /// the object to a "post-destructive" package file using the `add_file_to_package`
    /// helper function. This is commonly used in deployment processes to specify
    /// metadata components to be removed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Credentials, MetadataApi};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    ///
    ///     let mut api = MetadataApi::new(client);
    ///     let mut request = api.new_deployment_request();
    ///     request.delete_post("ApexClass", vec!["MyClass.cls"]);
    ///
    ///     let response = api.deploy(request).await?;
    ///     println!("Deploy response {:?}", response);
    ///     Ok(())
    /// }
    /// ```
    pub fn delete_post(&mut self, metadata_type: &str, objects: Vec<&str>) -> &mut Self {
        for file_name in objects {
            add_file_to_package(&mut self.post_destructive, metadata_type, file_name);
        }
        self
    }

    /// Deletes one or more metadata objects before the deplopment is started,
    /// by adding them to a "destructive" package file.
    ///
    /// # Parameters
    /// * `metadata_type`: The type of metadata (e.g., "CustomObject", "ApexClass") that is being deleted.
    /// * `objects`: The names of the metadata objects to be deleted.
    ///
    /// # Returns
    /// A mutable reference to `self`, allowing for method chaining.
    ///
    /// # Behavior
    /// For each object name provided in the `objects` vector, this function adds
    /// the object to a "post-destructive" package file using the `add_file_to_package`
    /// helper function. This is commonly used in deployment processes to specify
    /// metadata components to be removed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Credentials, MetadataApi};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    ///
    ///     let mut api = MetadataApi::new(client);
    ///     let mut request = api.new_deployment_request();
    ///     request.delete_pre("ApexClass", vec!["MyClass.cls"]);
    ///
    ///     let response = api.deploy(request).await?;
    ///     println!("Deploy response {:?}", response);
    ///     Ok(())
    /// }
    /// ```
    pub fn delete_pre(&mut self, metadata_type: &str, objects: Vec<&str>) -> &mut Self {
        for file_name in objects {
            add_file_to_package(&mut self.pre_destructive, metadata_type, file_name);
        }
        self
    }

    /// Adds a file to the zip-file used in the deployment
    ///
    /// # Parameters
    /// - `folder`: The name of the folder (e.g. "classes")
    /// - `file_name`: The name of the file (e.g. "MyClass.cls")
    /// - `content`: The content of the file
    ///
    /// # Returns
    /// - `Ok(())` on success, indicating that the package was successfully added.
    /// - `Err(Error)`: Returns an error if writing to the storage fails during entry creation or data writing.
    ///
    /// # Errors
    /// This method returns an `SZipError` if something went wrong writing the file to the zip-file
    ///
    fn add_file_to_zip(
        &mut self,
        folder: &str,
        file_name: &str,
        content: &[u8],
    ) -> Result<(), SZipError> {
        let filename = format!("{}/{}", folder, file_name);
        self.writer.start_entry(filename.as_str())?;
        self.writer.write_data(content)?;
        Ok(())
    }

    /// Adds a package to the storage by generating its metadata and writing it to the output stream.
    ///
    /// # Parameters
    /// - `name`: A string slice representing the name of the package (e.g., the file name or package identifier).
    /// - `content`: A reference to a `HashMap` where keys are `String` values representing metadata categories or types,
    ///              and values are `Vec<String>` representing the associated data for those categories.
    ///
    /// # Returns
    /// - `Ok(())` on success, indicating that the package was successfully added.
    /// - `Err(Error)`: Returns an error if writing to the storage fails during entry creation or data writing.
    ///
    /// # Errors
    /// This method returns an `Error::RequestError` if either of the following operations fail:
    /// - `start_entry`: An error occurs while starting the entry creation process for the package.
    /// - `write_data`: An error occurs while writing the generated package metadata to the output stream.
    ///
    fn add_package(
        &mut self,
        name: &str,
        content: HashMap<String, Vec<String>>,
    ) -> Result<()> {
        let package = generate_package_xml(&self.version, &content);
        println!("Adding package: {}\n{}", name, package);

        self.writer
            .start_entry(name)
            .map_err(|mut e| Error::RequestError(e.to_string()))?;

        self.writer
            .write_data(package.as_bytes())
            .map_err(|mut e| Error::RequestError(e.to_string()))?;

        Ok(())
    }

    /// Generates a JSON representation of the `DeployRequest`.
    ///
    /// # Returns
    /// - `Ok(String)`: A string containing the JSON-encoded `DeployRequest`.
    /// - `Err(Error)`: An error if serialization fails.
    ///
    /// # Errors
    /// This function will return an `Error::RequestError` if the `DeployRequest`
    /// struct cannot be serialized to JSON.
    ///
    /// # Example
    /// ```rust
    /// let deploy_request_json = instance.get_deploy_request_json();
    /// match deploy_request_json {
    ///     Ok(json) => println!("DeployRequest JSON: {}", json),
    ///     Err(e) => eprintln!("Error generating JSON: {}", e),
    /// }
    /// ```
    pub(crate) fn get_deploy_request_json(&self) -> Result<String> {
        let request = DeployRequest {
            options: self.options.clone(),
        };
        serde_json::to_string(&request).map_err(|e| Error::RequestError(e.to_string()))
    }

    /// Generates a ZIP file containing the specified package files and returns it as a `Vec<u8>`.
    ///
    /// # Functionality
    /// This method creates a ZIP file by adding a package XML file (`package.xml`)
    /// and optionally including pre- and post-destructive change XML files (`destructiveChangesPre.xml`
    /// and `destructiveChangesPost.xml`) if they are present. It finalizes the ZIP writer
    /// and returns the resulting ZIP file as a byte vector.
    ///
    /// # Parameters
    /// - `self`: Consumes the struct instance, giving ownership of the struct's data to the method.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)`: A `Vec<u8>` containing the bytes of the generated ZIP file upon successful
    ///   creation and finalization of the ZIP writer.
    /// - `Err(Error)`: An error of type `Error` if there is a failure during the ZIP construction process.
    ///
    /// # Errors
    /// - `Error::RequestError`: Returned when the ZIP writer encounters an error during the finalization process.
    ///
    /// # Usage Example
    /// ```rust
    /// let zip_file = my_instance.get_zip_file()?;
    /// // Use the generated `zip_file` (a `Vec<u8>`) as needed
    /// ```
    ///
    /// # Notes
    /// - The function assumes that `self.add_package` properly adds the specified files to the ZIP.
    /// - If `pre_destructive` or `post_destructive` is empty, the corresponding file will not be added to the ZIP.
    /// - The `finish` method on the ZIP writer is expected to finalize the ZIP and produce a writable result.
    ///
    pub(crate) fn get_zip_file(mut self) -> Result<Vec<u8>> {
        // Add the package.xml files to the zip
        self.add_package("package.xml", self.package.clone())?;
        if !self.pre_destructive.is_empty() {
            self.add_package("destructiveChangesPre.xml", self.pre_destructive.clone())?;
        }
        if !self.post_destructive.is_empty() {
            self.add_package("destructiveChangesPost.xml", self.post_destructive.clone())?;
        }

        Ok(self
            .writer
            .finish()
            .map_err(|e| Error::RequestError(e.to_string()))?
            .into_inner())
    }
}