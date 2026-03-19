use crate::error::AppError;
use crate::types::TranscriptionResult;
use reqwest::multipart;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct GroqTranscriptionResponse {
    text: String,
}

pub struct GroqClient {
    api_key: String,
    client: reqwest::Client,
}

impl GroqClient {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build reqwest client");

        Self { api_key, client }
    }

    pub async fn transcribe(&self, audio_wav: Vec<u8>) -> Result<TranscriptionResult, AppError> {
        const MAX_RETRIES: u32 = 3;
        let mut attempt = 0;

        loop {
            match self.transcribe_attempt(&audio_wav).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempt += 1;
                    if attempt >= MAX_RETRIES {
                        return Err(e);
                    }
                    
                    let delay = Duration::from_secs(2u64.pow(attempt - 1));
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    async fn transcribe_attempt(&self, audio_wav: &[u8]) -> Result<TranscriptionResult, AppError> {
        let part = multipart::Part::bytes(audio_wav.to_vec())
            .file_name("audio.wav")
            .mime_str("audio/wav")
            .map_err(|e| AppError::GroqApiError(format!("Failed to create multipart part: {}", e)))?;

        let form = multipart::Form::new()
            .text("model", "whisper-large-v3-turbo")
            .part("file", part);

        let response = self
            .client
            .post("https://api.groq.com/openai/v1/audio/transcriptions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    AppError::GroqApiError("Request timeout".to_string())
                } else if e.is_connect() {
                    AppError::GroqApiError("Connection error".to_string())
                } else {
                    AppError::GroqApiError(format!("Network error: {}", e))
                }
            })?;

        let status = response.status();
        
        if status.is_success() {
            let groq_response: GroqTranscriptionResponse = response
                .json()
                .await
                .map_err(|e| AppError::GroqApiError(format!("Failed to parse response: {}", e)))?;

            Ok(TranscriptionResult {
                text: groq_response.text,
                duration_ms: 0,
                language: None,
            })
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            match status.as_u16() {
                429 => Err(AppError::GroqApiError("Rate limit exceeded".to_string())),
                401 => Err(AppError::GroqApiError("Invalid API key".to_string())),
                400 => Err(AppError::GroqApiError(format!("Bad request: {}", error_text))),
                500..=599 => Err(AppError::GroqApiError("Server error".to_string())),
                _ => Err(AppError::GroqApiError(format!(
                    "API error ({}): {}",
                    status.as_u16(),
                    error_text
                ))),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, header};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use serde_json::json;

    #[tokio::test]
    async fn test_transcribe_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/openai/v1/audio/transcriptions"))
            .and(header("authorization", "Bearer test-key"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                        "text": "Hello world"
                    }))
            )
            .mount(&mock_server)
            .await;

        let client = reqwest::Client::new();
        let audio_data = vec![0u8; 1024];
        
        let part = multipart::Part::bytes(audio_data)
            .file_name("audio.wav")
            .mime_str("audio/wav")
            .unwrap();

        let form = multipart::Form::new()
            .text("model", "whisper-large-v3-turbo")
            .part("file", part);

        let response = client
            .post(format!("{}/openai/v1/audio/transcriptions", mock_server.uri()))
            .header("Authorization", "Bearer test-key")
            .multipart(form)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status().as_u16(), 200);
        let groq_response: GroqTranscriptionResponse = response.json().await.unwrap();
        assert_eq!(groq_response.text, "Hello world");
    }

    #[tokio::test]
    async fn test_rate_limit_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/openai/v1/audio/transcriptions"))
            .respond_with(ResponseTemplate::new(429))
            .mount(&mock_server)
            .await;

        let client = reqwest::Client::new();
        let audio_data = vec![0u8; 100];
        
        let part = multipart::Part::bytes(audio_data)
            .file_name("audio.wav")
            .mime_str("audio/wav")
            .unwrap();

        let form = multipart::Form::new()
            .text("model", "whisper-large-v3-turbo")
            .part("file", part);

        let response = client
            .post(format!("{}/openai/v1/audio/transcriptions", mock_server.uri()))
            .header("Authorization", "Bearer test-key")
            .multipart(form)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status().as_u16(), 429);
    }

    #[test]
    fn test_client_creation() {
        let client = GroqClient::new("test-api-key".to_string());
        assert_eq!(client.api_key, "test-api-key");
    }
}