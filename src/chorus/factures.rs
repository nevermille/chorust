use crate::data::factures::DeposerFluxData;
use crate::enums::ChorusResponse;
use crate::response::factures::DeposerFluxResponse;
use crate::Chorus;
use squared_api_wrapper::response::StringObjectResponse;
use squared_api_wrapper::traits::Api;
use url_builder::URLBuilder;

impl Chorus {
    fn factures_base_url(&self) -> URLBuilder {
        let mut url = URLBuilder::new();
        url.set_protocol("https")
            .set_host(&self.get_root_url())
            .add_route("cpro")
            .add_route("factures");

        url
    }

    fn deposer_flux_url(&self) -> String {
        let mut url = self.factures_base_url();
        url.add_route("v1").add_route("deposer").add_route("flux");

        url.build()
    }

    pub fn deposer_flux(
        &self,
        data: &DeposerFluxData,
    ) -> anyhow::Result<StringObjectResponse<ChorusResponse<DeposerFluxResponse>>> {
        let (mut curl, headers) = self.get_base_data()?;

        curl.url(&self.deposer_flux_url())?;
        curl.http_headers(headers)?;

        let body = serde_json::to_string(&data)?;

        let upload =
            squared_api_wrapper::post(&mut curl, Some(body.as_bytes()), None)?.to_string_response();
        let object = ChorusResponse::from_json(&upload.raw_data);

        Ok(upload.add_object(object))
    }
}
