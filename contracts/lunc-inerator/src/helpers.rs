use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, CosmosMsg, CustomQuery, Querier, QuerierWrapper, StdResult,
    WasmMsg, WasmQuery,
};

use crate::msg::{ExecuteMsg, QueryMsg};

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T, funds: Vec<Coin>) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds,
        }
        .into())
    }

    /// Get Community Addresses
    pub fn query_community_info<Q, T, CQ>(&self, querier: &Q, msg: QueryMsg) -> StdResult<Binary>
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
        .into();
        let res = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }
}
