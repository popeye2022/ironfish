/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
import { ASSET_ID_LENGTH } from '@ironfish/rust-nodejs'
import * as yup from 'yup'
import { CurrencyUtils } from '../../../utils'
import { ValidationError } from '../../adapters'
import { ApiNamespace, router } from '../router'

export type GetAssetRequest = {
  id: string
}

export type GetAssetResponse = {
  createdTransactionHash: string
  id: string
  metadata: string
  name: string
  nonce: number
  owner: string
  supply: string
}

export const GetAssetRequestSchema: yup.ObjectSchema<GetAssetRequest> = yup
  .object()
  .shape({
    id: yup.string(),
  })
  .defined()

export const GetAssetResponse: yup.ObjectSchema<GetAssetResponse> = yup
  .object({
    createdTransactionHash: yup.string().defined(),
    id: yup.string().defined(),
    metadata: yup.string().defined(),
    name: yup.string().defined(),
    nonce: yup.number().defined(),
    owner: yup.string().defined(),
    supply: yup.string().defined(),
  })
  .defined()

router.register<typeof GetAssetRequestSchema, GetAssetResponse>(
  `${ApiNamespace.chain}/getAsset`,
  GetAssetRequestSchema,
  async (request, node): Promise<void> => {
    const id = Buffer.from(request.data.id, 'hex')

    if (id.byteLength !== ASSET_ID_LENGTH) {
      throw new ValidationError(
        `Asset identifier is invalid length, expected ${ASSET_ID_LENGTH} but got ${id.byteLength}`,
      )
    }

    const asset = await node.chain.getAssetById(id)

    if (!asset) {
      throw new ValidationError(`No asset found with identifier ${request.data.id}`)
    }

    request.end({
      createdTransactionHash: asset.createdTransactionHash.toString('hex'),
      id: asset.id.toString('hex'),
      metadata: asset.metadata.toString('hex'),
      name: asset.name.toString('hex'),
      nonce: asset.nonce,
      owner: asset.owner.toString('hex'),
      supply: CurrencyUtils.encode(asset.supply),
    })
  },
)
