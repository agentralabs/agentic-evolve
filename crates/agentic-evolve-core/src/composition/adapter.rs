//! AdapterGenerator — generates adapters between patterns.

use crate::types::error::EvolveResult;
use crate::types::pattern::Pattern;

/// Generates adapter code between incompatible patterns.
#[derive(Debug, Default)]
pub struct AdapterGenerator;

impl AdapterGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_adapter(
        &self,
        source: &Pattern,
        target: &Pattern,
    ) -> EvolveResult<AdapterCode> {
        let needs_type_conversion = source.signature.return_type
            != target
                .signature
                .params
                .first()
                .map(|p| p.param_type.clone());
        let needs_async_bridge = source.signature.is_async != target.signature.is_async;

        let code =
            self.build_adapter_code(source, target, needs_type_conversion, needs_async_bridge);

        Ok(AdapterCode {
            code,
            source_pattern_id: source.id.as_str().to_string(),
            target_pattern_id: target.id.as_str().to_string(),
            needs_type_conversion,
            needs_async_bridge,
        })
    }

    fn build_adapter_code(
        &self,
        source: &Pattern,
        target: &Pattern,
        needs_type_conversion: bool,
        needs_async_bridge: bool,
    ) -> String {
        let mut code = String::new();

        if needs_async_bridge && source.signature.is_async && !target.signature.is_async {
            code.push_str("// Async-to-sync bridge\n");
            code.push_str(&format!(
                "let result = tokio::runtime::Handle::current().block_on({}_result);\n",
                source.signature.name
            ));
        }

        if needs_type_conversion {
            code.push_str(&format!(
                "// Type conversion: {} output -> {} input\n",
                source.signature.name, target.signature.name
            ));
            code.push_str(&format!(
                "let adapted = {}_output.into();\n",
                source.signature.name
            ));
        }

        if code.is_empty() {
            code.push_str("// Direct connection - no adapter needed\n");
        }

        code
    }
}

/// Generated adapter code.
#[derive(Debug, Clone)]
pub struct AdapterCode {
    pub code: String,
    pub source_pattern_id: String,
    pub target_pattern_id: String,
    pub needs_type_conversion: bool,
    pub needs_async_bridge: bool,
}
