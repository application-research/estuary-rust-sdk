/*
 * Estuary API
 *
 * This is the API for the Estuary application.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UtilContentAddIpfsBody {
    #[serde(rename = "collection", skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,
    #[serde(rename = "collectionPath", skip_serializing_if = "Option::is_none")]
    pub collection_path: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "peers", skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<String>>,
    #[serde(rename = "root", skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
}

impl UtilContentAddIpfsBody {
    pub fn new() -> UtilContentAddIpfsBody {
        UtilContentAddIpfsBody {
            collection: None,
            collection_path: None,
            name: None,
            peers: None,
            root: None,
        }
    }
}


