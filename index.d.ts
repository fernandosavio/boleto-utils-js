/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface ConvenioInfo {
  codigo: string
}
export const enum TipoValor {
  Real = 'Real',
  QtdeMoeda = 'QtdeMoeda'
}
export interface ArrecadacaoInfo {
  codBarras: string
  linhaDigitavel: string
  segmento: string
  tipoValor: TipoValor
  digitoVerificador: number
  valor?: number
  convenio: ConvenioInfo
}
export interface BancoInfo {
  codigo: number
}
export const enum Moeda {
  Real = 'Real',
  Outras = 'Outras'
}
export interface CobrancaInfo {
  codigoBarras: string
  linhaDigitavel: string
  banco: BancoInfo
  codMoeda: Moeda
  digitoVerificador: number
  dataVencimento?: string
  valor?: number
}
export const enum TipoBoleto {
  Cobranca = 'Cobranca',
  Arrecadacao = 'Arrecadacao'
}
export interface BoletoData {
  tipoBoleto: TipoBoleto
  info: CobrancaInfo | ArrecadacaoInfo
}
export interface BoletoInfo {
  isValid: boolean
  data?: BoletoData
  error?: string
}
export function validate(barcode: string): BoletoInfo
