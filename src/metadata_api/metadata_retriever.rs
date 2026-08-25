const METADATA_NAMESPACE: &str = "http://soap.sforce.com/2006/04/metadata";
const ENVELOPE_NAMESPACE: &str = "http://schemas.xmlsoap.org/soap/envelope/";

/*
The endpoint /services/data/vXX.0/metadata/deployRequest is specifically designed for deploying metadata to a Salesforce org, not for retrieving it.
 To retrieve metadata, you should use the /services/data/vXX.0/metadata/retrieveRequest endpoint.

Below is an example of how to retrieve metadata using the correct retrieveRequest endpoint via Apex:

```apex
public class MetadataRetrieveExample {
    public static void retrieveMetadata() {
        // 1. Define the package.xml content specifying what to retrieve
        String packageXml = '<?xml version="1.0" encoding="UTF-8"?>\n' +
                            '<Package xmlns="http://soap.sforce.com/2006/04/metadata">\n' +
                            '    <types>\n' +
                            '        <members>*</members>\n' +
                            '        <name>ApexClass</name>\n' +
                            '    </types>\n' +
                            '    <version>58.0</version>\n' +
                            '</Package>';

        // 2. Prepare the REST request
        HttpRequest req = new HttpRequest();
        req.setEndpoint(URL.getOrgDomainUrl().toExternalForm() + '/services/data/v58.0/metadata/retrieveRequest');
        req.setMethod('POST');
        req.setHeader('Authorization', 'Bearer ' + UserInfo.getSessionId());
        req.setHeader('Content-Type', 'application/json');

        // 3. Set the body with the package.xml encoded as base64
        // The retrieve API expects a JSON object with a "unpackaged" field containing the base64 encoded package.xml
        String body = '{"unpackaged" : "' + EncodingUtil.base64Encode(Blob.valueOf(packageXml)) + '"}';
        req.setBody(body);

        // 4. Send the request
        Http http = new Http();
        HttpResponse res = http.send(req);

        System.debug('Retrieve Request Status: ' + res.getStatusCode());
        System.debug('Retrieve Response: ' + res.getBody());

        // 5. Parse the response to get the asyncResultId
        Map<String, Object> responseMap = (Map<String, Object>) JSON.deserializeUntyped(res.getBody());
        String asyncResultId = (String) responseMap.get('id');

        // 6. Poll for status using the asyncResultId
        checkRetrieveStatus(asyncResultId);
    }

    public static void checkRetrieveStatus(String asyncResultId) {
        HttpRequest req = new HttpRequest();
        req.setEndpoint(URL.getOrgDomainUrl().toExternalForm() + '/services/data/v58.0/metadata/retrieveRequest/' + asyncResultId);
        req.setMethod('GET');
        req.setHeader('Authorization', 'Bearer ' + UserInfo.getSessionId());
        req.setHeader('Content-Type', 'application/json');

        Http http = new Http();
        HttpResponse res = http.send(req);

        Map<String, Object> responseMap = (Map<String, Object>) JSON.deserializeUntyped(res.getBody());
        String status = (String) responseMap.get('status');

        if (status == 'In Progress') {
            // Poll again after a delay
            System.debug('Still retrieving...');
        } else if (status == 'Success') {
            // The response will contain the zip file encoded in base64
            String zipFile = (String) responseMap.get('zipFile');
            System.debug('Retrieved zip file size: ' + zipFile.length());
            // Decode the zipFile to retrieve the metadata components
            Blob zipBlob = EncodingUtil.base64Decode(zipFile);
            // Process the zipBlob as needed
        } else {
            System.debug('Retrieve Failed: ' + res.getBody());
        }
    }
}
```

Key Differences:
DeployRequest: Uses POST to /metadata/deployRequest with a body containing zipFile (the metadata to deploy) and deployOptions.
RetrieveRequest: Uses POST to /metadata/retrieveRequest with a body containing unpackaged (the package.xml defining what to retrieve). The response provides an id to poll for the result, which includes the zipFile of the retrieved metadata.

 */
use std::collections::HashMap;
use crate::Client;

pub struct MetadataRetriever {
    package: HashMap<String, Vec<String>>,
}

impl MetadataRetriever {
    pub fn new() -> Self {
        MetadataRetriever {
            package: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, member: &str) -> &mut Self {
        self.package
            .entry(name.to_string())
            .or_insert_with(|| { vec![] })
            .push(member.to_string());
        self
    }

/*    pub fn get_request_body(&self, session_id: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenv="{}" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns="{}">
  <soapenv:Header><SessionHeader><sessionId>{}</sessionId></SessionHeader></soapenv:Header>
  <soapenv:Body>
    <retrieve>
      <retrieveRequest>
        <apiVersion>67.0</apiVersion>
        <unpackaged>{}</unpackaged>
      </retrieveRequest>
    </retrieve>
  </soapenv:Body>
</soapenv:Envelope>"#,
            ENVELOPE_NAMESPACE,
            METADATA_NAMESPACE,
            escape_xml(session_id),
            self.get_package(),
        )
    }*/

    pub fn get_package(&self) -> String {
        let mut result = "<types>".to_string();
        for (name, members) in &self.package {

            for member in members {
                result.push_str(&format!("<members>{}</members>", member));
            }

            result.push_str(&format!("<name>{}</name>", name));
        }
        result.push_str("</types>");
        result
    }



/*    fn session_id(&self) -> Result<&str, Error> {
        self.client.access_token_value().ok_or(Error::NotLoggedIn)
    }*/
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}