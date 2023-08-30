use crate::utils::dir_files::get_wakflo_config;
use crate::utils::types::WakfloExtension;
use anyhow::bail;
use std::path::PathBuf;
use ureq_multipart::MultipartBuilder;

pub struct WakfloUploadApi {
    base_url: String,
    client: ureq::Agent,
}

impl WakfloUploadApi {
    pub fn new(base_url: String, client: ureq::Agent) -> WakfloUploadApi {
        WakfloUploadApi { base_url, client }
    }

    pub(crate) fn upload_file(
        &self,
        wakflo: WakfloExtension,
        file_path: PathBuf,
    ) -> anyhow::Result<()> {
        let url = format!("{}{}", self.base_url, "/registry/upload/test");
        let mut req = self.client.post(url.as_str());

        let config = get_wakflo_config()?;
        if config.auth.is_some() {
            req = req.set(
                "Authorization",
                format!(
                    "Bearer {}",
                    config.auth.expect("missing auth api").access_token
                )
                .as_str(),
            );
        }

        let lang_ref = wakflo.plugin.language.as_ref().to_lowercase();
        let lang = lang_ref.as_str();
        let version = wakflo.plugin.version.as_str();
        let description = wakflo.plugin.description.as_str();
        let name = wakflo.plugin.name.as_str();

        let (content_type, data) = MultipartBuilder::new()
            .add_file("file", file_path)
            .unwrap()
            .add_text("name", name)
            .unwrap()
            .add_text("language", lang)
            .unwrap()
            .add_text("version", version)
            .unwrap()
            .add_text("description", description)
            .unwrap()
            .finish()
            .unwrap();

        match req
            .set("Content-Type", &content_type)
            .send_bytes(data.as_slice())
        {
            Ok(_) => Ok(()),
            Err(e) => {
                if let Some(rsp) = e.into_response() {
                    let status = rsp.status();
                    let msg = rsp.into_string().unwrap();

                    #[allow(unreachable_code)]
                    return if status == 409 {
                        bail!(format!(
                            "{name} plugin with version {version} already deployed"
                        ))
                    } else {
                        bail!(msg)
                    };
                }

                Ok(())
            }
        }
    }
}
