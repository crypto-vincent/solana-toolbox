use anyhow::anyhow;
use anyhow::Result;
use serde_json::Value;

use crate::toolbox_idl_instruction::ToolboxIdlInstruction;

impl ToolboxIdlInstruction {
    pub fn encode_payload(
        &self,
        instruction_payload: &Value,
    ) -> Result<Vec<u8>> {
        let mut instruction_data = vec![];
        instruction_data.extend_from_slice(&self.discriminator);
        self.args_type_full_fields.try_serialize(
            instruction_payload,
            &mut instruction_data,
            true,
        )?;
        Ok(instruction_data)
    }

    pub fn decode_payload(&self, instruction_data: &[u8]) -> Result<Value> {
        if !instruction_data.starts_with(&self.discriminator) {
            return Err(anyhow!(
                "Invalid instruction discriminator: expected: {:?}, found: {:?}",
                self.discriminator,
                instruction_data
            ));
        }
        let (_, instruction_payload) = self
            .args_type_full_fields
            .try_deserialize(instruction_data, self.discriminator.len())?;
        Ok(instruction_payload)
    }
}
