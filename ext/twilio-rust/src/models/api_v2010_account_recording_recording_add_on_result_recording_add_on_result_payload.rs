/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.25.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiV2010AccountRecordingRecordingAddOnResultRecordingAddOnResultPayload {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The SID of the Add-on configuration
    #[serde(rename = "add_on_configuration_sid", skip_serializing_if = "Option::is_none")]
    pub add_on_configuration_sid: Option<String>,
    /// The SID of the AddOnResult to which the payload belongs
    #[serde(rename = "add_on_result_sid", skip_serializing_if = "Option::is_none")]
    pub add_on_result_sid: Option<String>,
    /// The SID of the Add-on to which the result belongs
    #[serde(rename = "add_on_sid", skip_serializing_if = "Option::is_none")]
    pub add_on_sid: Option<String>,
    /// The MIME type of the payload
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The string that describes the payload
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The SID of the recording to which the AddOnResult resource that contains the payload belongs
    #[serde(rename = "reference_sid", skip_serializing_if = "Option::is_none")]
    pub reference_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// A list of related resources identified by their relative URIs
    #[serde(rename = "subresource_uris", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<serde_json::Value>,
}

impl ApiV2010AccountRecordingRecordingAddOnResultRecordingAddOnResultPayload {
    pub fn new() -> ApiV2010AccountRecordingRecordingAddOnResultRecordingAddOnResultPayload {
        ApiV2010AccountRecordingRecordingAddOnResultRecordingAddOnResultPayload {
            account_sid: None,
            add_on_configuration_sid: None,
            add_on_result_sid: None,
            add_on_sid: None,
            content_type: None,
            date_created: None,
            date_updated: None,
            label: None,
            reference_sid: None,
            sid: None,
            subresource_uris: None,
        }
    }
}


