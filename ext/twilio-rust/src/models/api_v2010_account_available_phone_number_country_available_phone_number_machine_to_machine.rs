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
pub struct ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberMachineToMachine {
    /// The type of Address resource the phone number requires
    #[serde(rename = "address_requirements", skip_serializing_if = "Option::is_none")]
    pub address_requirements: Option<String>,
    /// Whether the phone number is new to the Twilio platform
    #[serde(rename = "beta", skip_serializing_if = "Option::is_none")]
    pub beta: Option<bool>,
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Box<crate::models::ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities>>,
    /// A formatted version of the phone number
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The ISO country code of this phone number
    #[serde(rename = "iso_country", skip_serializing_if = "Option::is_none")]
    pub iso_country: Option<String>,
    /// The LATA of this phone number
    #[serde(rename = "lata", skip_serializing_if = "Option::is_none")]
    pub lata: Option<String>,
    /// The latitude of this phone number's location
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f32>,
    /// The locality or city of this phone number's location
    #[serde(rename = "locality", skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// The longitude of this phone number's location
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f32>,
    /// The phone number in E.164 format
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The postal or ZIP code of this phone number's location
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The rate center of this phone number
    #[serde(rename = "rate_center", skip_serializing_if = "Option::is_none")]
    pub rate_center: Option<String>,
    /// The two-letter state or province abbreviation of this phone number's location
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberMachineToMachine {
    pub fn new() -> ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberMachineToMachine {
        ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberMachineToMachine {
            address_requirements: None,
            beta: None,
            capabilities: None,
            friendly_name: None,
            iso_country: None,
            lata: None,
            latitude: None,
            locality: None,
            longitude: None,
            phone_number: None,
            postal_code: None,
            rate_center: None,
            region: None,
        }
    }
}


