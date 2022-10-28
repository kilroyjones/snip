use pyo3::prelude::*;
use pyo3::types::PyTuple;

pub struct Code {
    pub language: String,
    pub probability: f32,
}

pub async fn guess(snippet: String) -> PyResult<(Code)> {
    let code = Python::with_gil(|py| -> PyResult<Code> {
        let guess = py.import("guesslang")?;
        let res = guess.getattr("Guess")?;
        let code = PyModule::from_code(
            py,
            "
class Code:
    def __init__(self, snippet):
        from guesslang import Guess
        guess = Guess()
        probs = guess.probabilities(snippet)
        self.language = probs[0][0]
        self.probability = probs[0][1] 
    
    def getLanguage(self):
        return self.language

    def getProbability(self):
        return self.probability",
            "",
            "",
        )
        .unwrap();

        let code_class = code.getattr("Code").unwrap();
        let code = code_class.call1((snippet,)).unwrap();
        let lang = code.call_method0("getLanguage").unwrap();
        let prob = code.call_method0("getProbability").unwrap();

        // let prob: f32 = code.call_method0("getProbability").unwrap();
        Ok(Code {
            language: lang.to_string(),
            probability: prob.to_string().parse::<f32>().unwrap(),
        })
    })
    .unwrap();
    Ok(code)
}
