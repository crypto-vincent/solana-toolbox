use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_hash_discriminator_from_string;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64;

impl ToolboxIdlAccount {
    pub fn try_parse(
        idl_account_name: &str,
        idl_account: &Value,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlAccount, ToolboxIdlError> {
        let idl_account =
            idl_as_object_or_else(idl_account, &breadcrumbs.idl())?;
        let discriminator = ToolboxIdlAccount::try_parse_discriminator(
            idl_account_name,
            idl_account,
            &breadcrumbs.with_idl("discriminator"),
        )?;
        let docs = idl_account.get("docs").cloned();
        let space = idl_object_get_key_as_u64(idl_account, "space")
            .map(usize::try_from)
            .transpose()
            .map_err(|error| ToolboxIdlError::InvalidInteger {
                conversion: error,
                context: breadcrumbs.idl(),
            })?;
        let content_type_flat = ToolboxIdlAccount::try_parse_data_type_flat(
            idl_account_name,
            idl_account,
            breadcrumbs,
        )?;
        let content_type_full = content_type_flat.try_hydrate(
            &HashMap::new(),
            typedefs,
            breadcrumbs,
        )?;
        Ok(ToolboxIdlAccount {
            name: idl_account_name.to_string(),
            docs,
            space,
            discriminator,
            content_type_flat,
            content_type_full: content_type_full.into(),
        })
    }

    fn try_parse_discriminator(
        idl_account_name: &str,
        idl_account: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        if let Some(idl_account_discriminator) =
            idl_object_get_key_as_array(idl_account, "discriminator")
        {
            return idl_as_bytes_or_else(
                idl_account_discriminator,
                &breadcrumbs.idl(),
            );
        }
        Ok(idl_hash_discriminator_from_string(&format!(
            "account:{}",
            idl_account_name
        )))
    }

    fn try_parse_data_type_flat(
        idl_account_name: &str,
        idl_account: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        if idl_account.contains_key("type")
            || idl_account.contains_key("defined")
            || idl_account.contains_key("generic")
            || idl_account.contains_key("option")
            || idl_account.contains_key("option32")
            || idl_account.contains_key("vec") // TODO (MEDIUM) - should we support vec8/variants32 ??
            || idl_account.contains_key("array")
            || idl_account.contains_key("fields")
            || idl_account.contains_key("variants")
            || idl_account.contains_key("padded")
        {
            return ToolboxIdlTypeFlat::try_parse_object(
                idl_account,
                breadcrumbs,
            );
        }
        Ok(ToolboxIdlTypeFlat::Defined {
            name: idl_account_name.to_string(),
            generics: vec![],
        })
    }
}
