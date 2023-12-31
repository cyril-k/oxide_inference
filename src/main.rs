use anyhow::Result;
use inference_server::{Predictor, PredictorConfig};

fn main() -> Result<()> {
    let model_id = Some("intfloat/multilingual-e5-base".to_string());
    let revision = Some("main".to_string());

    let model_config = PredictorConfig::load(model_id, revision)?;

    let predictor = Predictor::load(&model_config)?;

    let input = "test string".to_string();

    let embeddings = predictor.process_single(input)?;

    println!("{embeddings}");

    Ok(())
}
