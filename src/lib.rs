#![deny(clippy::all)]

use boleto_utils::{
    arrecadacao::Arrecadacao,
    cobranca::{Cobranca, CodigoMoeda},
    Boleto, BoletoError,
};
use napi::bindgen_prelude::*;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct ConvenioInfo {
    pub codigo: String,
}

#[napi(string_enum)]
pub enum TipoValor {
    Real,
    QtdeMoeda,
}

#[napi(object)]
pub struct ArrecadacaoInfo {
    pub cod_barras: String,
    pub linha_digitavel: String,
    pub segmento: String,
    pub tipo_valor: TipoValor,
    pub digito_verificador: u8,
    pub valor: Option<f64>,
    pub convenio: ConvenioInfo,
}

#[napi(object)]
pub struct BancoInfo {
    pub codigo: u16,
}

#[napi(string_enum)]
pub enum Moeda {
    Real,
    Outras,
}

#[napi(object)]
pub struct CobrancaInfo {
    pub codigo_barras: String,
    pub linha_digitavel: String,
    pub banco: BancoInfo,
    pub cod_moeda: Moeda,
    pub digito_verificador: u8,
    pub data_vencimento: Option<String>,
    pub valor: Option<f64>,
}

#[napi(string_enum)]
pub enum TipoBoleto {
    Cobranca,
    Arrecadacao,
}

#[napi(object)]
pub struct BoletoData {
    pub tipo_boleto: TipoBoleto,
    pub info: Either<CobrancaInfo, ArrecadacaoInfo>,
}

impl BoletoData {
    fn from_cobranca(cobranca: Cobranca) -> Self {
        Self {
            tipo_boleto: TipoBoleto::Cobranca,
            info: Either::A(CobrancaInfo {
                codigo_barras: cobranca.cod_barras.to_string(),
                linha_digitavel: cobranca.linha_digitavel.to_string(),
                banco: BancoInfo {
                    codigo: cobranca.cod_banco.0,
                },
                cod_moeda: match cobranca.cod_moeda {
                    CodigoMoeda::Real => Moeda::Real,
                    CodigoMoeda::Outras => Moeda::Outras,
                },
                digito_verificador: cobranca.digito_verificador,
                data_vencimento: cobranca
                    .data_vencimento
                    .and_then(|date| Some(date.format("%Y-%m-%d").to_string())),
                valor: cobranca.valor,
            }),
        }
    }

    fn from_arrecadacao(arrecadacao: Arrecadacao) -> Self {
        Self {
            tipo_boleto: TipoBoleto::Cobranca,
            info: Either::B(ArrecadacaoInfo {
                cod_barras: arrecadacao.cod_barras.to_string(),
                linha_digitavel: arrecadacao.linha_digitavel.to_string(),
                segmento: arrecadacao.segmento.to_string(),
                tipo_valor: match arrecadacao.tipo_valor {
                    boleto_utils::arrecadacao::TipoValor::QtdeMoedaMod10
                    | boleto_utils::arrecadacao::TipoValor::QtdeMoedaMod11 => TipoValor::QtdeMoeda,
                    boleto_utils::arrecadacao::TipoValor::ValorReaisMod10
                    | boleto_utils::arrecadacao::TipoValor::ValorReaisMod11 => TipoValor::Real,
                },
                digito_verificador: arrecadacao.digito_verificador,
                valor: arrecadacao.valor,
                convenio: ConvenioInfo {
                    codigo: arrecadacao.convenio.to_string(),
                },
            }),
        }
    }
}

#[napi(object)]
pub struct BoletoInfo {
    pub is_valid: bool,
    pub data: Option<BoletoData>,
    pub error: Option<String>,
}

impl BoletoInfo {
    fn from_error(error: BoletoError) -> BoletoInfo {
        BoletoInfo {
            is_valid: false,
            data: None,
            error: Some(error.to_string()),
        }
    }

    fn from_info(boleto: Boleto) -> BoletoInfo {
        BoletoInfo {
            is_valid: true,
            data: Some(match boleto {
                Boleto::Arrecadacao(data) => BoletoData::from_arrecadacao(data),
                Boleto::Cobranca(data) => BoletoData::from_cobranca(data),
            }),
            error: None,
        }
    }
}

#[napi]
pub fn validate(barcode: String) -> BoletoInfo {
    match Boleto::new(barcode.as_bytes()) {
        Ok(boleto) => BoletoInfo::from_info(boleto),
        Err(error) => BoletoInfo::from_error(error),
    }
}
