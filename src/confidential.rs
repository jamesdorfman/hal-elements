use bitcoin::Network;
use bitcoin_hashes::sha256d;
use elements::confidential::{Asset, Nonce, Value};
use serde::{Deserialize, Serialize};

use hal::{GetInfo, HexBytes};

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfidentialType {
	Null,
	Explicit,
	Confidential,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct ConfidentialValueInfo {
	#[serde(rename = "type")]
	pub type_: ConfidentialType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub commitment: Option<HexBytes>,
}

impl GetInfo<ConfidentialValueInfo> for Value {
	fn get_info(&self, _network: Network) -> ConfidentialValueInfo {
		ConfidentialValueInfo {
			type_: match self {
				Value::Null => ConfidentialType::Null,
				Value::Explicit(_) => ConfidentialType::Explicit,
				Value::Confidential(_, _) => ConfidentialType::Confidential,
			},
			value: match self {
				Value::Explicit(v) => Some(*v),
				_ => None,
			},
			commitment: match self {
				Value::Confidential(p, c) => {
					let mut commitment = Vec::new();
					commitment.push(*p);
					commitment.extend(c);
					Some(commitment[..].into())
				}
				_ => None,
			},
		}
	}
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct ConfidentialAssetInfo {
	#[serde(rename = "type")]
	pub type_: ConfidentialType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub asset: Option<sha256d::Hash>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub commitment: Option<HexBytes>,
}

impl GetInfo<ConfidentialAssetInfo> for Asset {
	fn get_info(&self, _network: Network) -> ConfidentialAssetInfo {
		ConfidentialAssetInfo {
			type_: match self {
				Asset::Null => ConfidentialType::Null,
				Asset::Explicit(_) => ConfidentialType::Explicit,
				Asset::Confidential(_, _) => ConfidentialType::Confidential,
			},
			asset: match self {
				Asset::Explicit(a) => Some(*a),
				_ => None,
			},
			commitment: match self {
				Asset::Confidential(p, c) => {
					let mut commitment = Vec::new();
					commitment.push(*p);
					commitment.extend(c);
					Some(commitment[..].into())
				}
				_ => None,
			},
		}
	}
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct ConfidentialNonceInfo {
	#[serde(rename = "type")]
	pub type_: ConfidentialType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nonce: Option<sha256d::Hash>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub commitment: Option<HexBytes>,
}

impl GetInfo<ConfidentialNonceInfo> for Nonce {
	fn get_info(&self, _network: Network) -> ConfidentialNonceInfo {
		ConfidentialNonceInfo {
			type_: match self {
				Nonce::Null => ConfidentialType::Null,
				Nonce::Explicit(_) => ConfidentialType::Explicit,
				Nonce::Confidential(_, _) => ConfidentialType::Confidential,
			},
			nonce: match self {
				Nonce::Explicit(n) => Some(*n),
				_ => None,
			},
			commitment: match self {
				Nonce::Confidential(p, c) => {
					let mut commitment = Vec::new();
					commitment.push(*p);
					commitment.extend(c);
					Some(commitment[..].into())
				}
				_ => None,
			},
		}
	}
}
