use std::collections::HashMap;

use anyhow::Context;
use anyhow::Result;
use pyo3::prelude::*;

#[pyclass]
struct StablePair {
    inner: flatqube_pair::stable_pair::StablePair,
}

#[pymethods]
impl StablePair {
    #[new]
    pub fn new(
        token_data: Vec<TokenDataInput>,
        token_index: HashMap<String, u8>,
        a: AmplificationCoefficient,
        fee_params: FeeParams,
        lp_supply: u128,
    ) -> Result<Self> {
        let token_data = token_data
            .into_iter()
            .map(|token_data| token_data.into())
            .collect();
        let token_index: Result<HashMap<_, _>> = token_index
            .into_iter()
            .map(|(k, v)| {
                anyhow::ensure!(
                    k.len() == 64,
                    "token index key must be 64 bytes, got: {}",
                    k.len()
                );
                let k = hex::decode(k).context("Failed to decode token index key");
                let k = k.and_then(|k| {
                    if k.len() == 32 {
                        let k: [u8; 32] = k.try_into().unwrap();
                        Ok(k)
                    } else {
                        Err(anyhow::anyhow!(
                            "invalid token index key length: {}",
                            k.len()
                        ))
                    }
                });
                k.map(|k| (k, v))
            })
            .collect();
        let token_index = token_index?;
        let a = a.into();
        let fee_params = fee_params.into();

        Ok(Self {
            inner: flatqube_pair::stable_pair::StablePair::new(
                token_data,
                token_index,
                a,
                fee_params,
                lp_supply,
            )
            .context("Failed to create stable pair")?,
        })
    }

    pub fn expected_exchange(
        &self,
        amount: u128,
        spent_token: &str,
        receive_token: &str,
    ) -> Result<SwapResult> {
        let spent_token = hex::decode(spent_token).context("Failed to decode spent token")?;
        let spent_token = spent_token
            .try_into()
            .map_err(|_| anyhow::anyhow!("Failed to convert spent token"))?;
        let receive_token = hex::decode(receive_token).context("Failed to decode receive token")?;
        let receive_token = receive_token
            .try_into()
            .map_err(|_| anyhow::anyhow!("Failed to convert spent token"))?;

        self.inner
            .expected_exchange_extended(amount, &spent_token, &receive_token)
            .map(|x| SwapResult {
                amount: x.amount,
                fee: x.fee,
            })
            .context("Failed to calculate expected exchange")
    }

    pub fn expected_spend_amount(
        &self,
        receive_amount: u128,
        receive_token_root: &str,
        spent_token_root: &str,
    ) -> Result<SwapResult> {
        let receive_token_root =
            hex::decode(receive_token_root).context("Failed to decode receive token root")?;
        let receive_token_root = receive_token_root
            .try_into()
            .map_err(|_| anyhow::anyhow!("Failed to convert receive token root"))?;
        let spent_token_root =
            hex::decode(spent_token_root).context("Failed to decode spent token root")?;
        let spent_token_root = spent_token_root
            .try_into()
            .map_err(|_| anyhow::anyhow!("Failed to convert spent token root"))?;

        self.inner
            .expected_spend_amount_extended(receive_amount, &receive_token_root, &spent_token_root)
            .map(|x| SwapResult {
                amount: x.amount,
                fee: x.fee,
            })
            .context("Failed to calculate expected spend amount")
    }

    pub fn update_balances(&mut self, balances: Vec<u128>, lp_supply: u128) -> Result<()> {
        self.inner.update_balances(balances, lp_supply)
    }
}

#[pyclass]
#[derive(Clone, Debug, Copy)]
pub struct SwapResult {
    #[pyo3(get, set)]
    pub amount: u128,
    #[pyo3(get, set)]
    pub fee: u128,
}

#[pymethods]
impl SwapResult {
    pub fn __repr__(&self) -> String {
        format!("SwapResult(amount={}, fee={})", self.amount, self.fee)
    }
}

#[pyclass]
#[derive(Clone, Debug, Copy)]
pub struct TokenDataInput {
    #[pyo3(get, set)]
    pub decimals: u8,
    #[pyo3(get, set)]
    pub balance: u128,
}

#[pymethods]
impl TokenDataInput {
    #[new]
    pub fn new(decimals: u8, balance: u128) -> Self {
        Self { decimals, balance }
    }
}

impl From<TokenDataInput> for flatqube_pair::stable_pair::TokenDataInput {
    fn from(token_data: TokenDataInput) -> Self {
        Self {
            decimals: token_data.decimals,
            balance: token_data.balance,
        }
    }
}

#[pyclass]
#[derive(Clone, Debug, Copy)]
pub struct AmplificationCoefficient {
    pub value: u128,
    pub precision: u8,
}

#[pymethods]
impl AmplificationCoefficient {
    #[new]
    pub fn new(value: u128, precision: u8) -> Self {
        Self { value, precision }
    }
}

impl From<AmplificationCoefficient> for flatqube_pair::stable_pair::AmplificationCoefficient {
    fn from(amplification_coefficient: AmplificationCoefficient) -> Self {
        Self {
            value: amplification_coefficient.value.into(),
            precision: amplification_coefficient.precision.into(),
        }
    }
}

#[pyclass]
#[derive(Clone, Debug, Copy)]
pub struct FeeParams {
    pub denominator: u128,
    pub pool_numerator: u128,
    pub beneficiary_numerator: u128,
}

#[pymethods]
impl FeeParams {
    #[new]
    pub fn new(denominator: u128, pool_numerator: u128, beneficiary_numerator: u128) -> Self {
        Self {
            denominator,
            pool_numerator,
            beneficiary_numerator,
        }
    }
}

impl From<FeeParams> for flatqube_pair::stable_pair::FeeParams {
    fn from(fee_params: FeeParams) -> Self {
        Self {
            denominator: fee_params.denominator.into(),
            pool_numerator: fee_params.pool_numerator.into(),
            beneficiary_numerator: fee_params.beneficiary_numerator.into(),
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn flatqube_pair_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<StablePair>()?;
    m.add_class::<SwapResult>()?;
    m.add_class::<TokenDataInput>()?;
    m.add_class::<AmplificationCoefficient>()?;
    m.add_class::<FeeParams>()?;

    Ok(())
}
