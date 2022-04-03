/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export interface Uri {
  scheme: string
  authority: string
  path: string
  query: string
  fragment: string
}
export function parse(source: string): Uri | undefined | null
