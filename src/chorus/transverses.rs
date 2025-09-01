use crate::data::transverses::ConsulterCrDetailleData;
use crate::enums::ChorusResponse;
use crate::macros::log::debug;
use crate::response::transverses::ConsulterCrDetailleResponse;
use crate::Chorus;
use squared_api_wrapper::response::StringObjectResponse;
use squared_api_wrapper::traits::Api;
use url_builder::URLBuilder;

impl Chorus {
    fn transverses_base_url(&self) -> URLBuilder {
        let mut url = URLBuilder::new();
        url.set_protocol("https")
            .set_host(&self.get_root_url())
            .add_route("cpro")
            .add_route("transverses");

        url
    }

    fn consulter_cr_detaille_url(&self) -> String {
        let mut url = self.transverses_base_url();
        url.add_route("v1").add_route("consulterCRDetaille");

        url.build()
    }

    pub fn consulter_cr_detaille(
        &self,
        data: &ConsulterCrDetailleData,
    ) -> anyhow::Result<StringObjectResponse<ChorusResponse<ConsulterCrDetailleResponse>>> {
        let (mut curl, headers) = self.get_base_data()?;

        curl.url(&self.consulter_cr_detaille_url())?;
        curl.http_headers(headers)?;

        let body = serde_json::to_string(&data)?;

        debug!(
            "Trying to consult flow with {}",
            &self.consulter_cr_detaille_url()
        );

        let upload =
            squared_api_wrapper::post(&mut curl, Some(body.as_bytes()), None)?.to_string_response();

        debug!("Consult got a status {}", u32::from(upload.http_code));

        let object = ChorusResponse::from_json(&upload.raw_data);

        Ok(upload.add_object(object))
    }
}
