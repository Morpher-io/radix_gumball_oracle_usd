<script setup lang="ts">
import { useRadixStore } from '@/stores/radixstore'
import { GatewayProcessor } from '@beaker-tools/typescript-toolkit'
import { RadixNetwork, type WalletDataStateAccount } from '@radixdlt/radix-dapp-toolkit'
import { storeToRefs } from 'pinia'
import { onMounted, ref } from 'vue'
import { getPrice } from '@/axios/prices'
import { updatePublicKey } from '@/radix/manifests/updatePublicKey'
import { buyGumballManifest } from '@/radix/manifests/buy_gumball'
import { priceMsgToString } from '@/radix/utils'

const store = useRadixStore()

const { account, radixDappToolkit } = storeToRefs(store)

const props = defineProps({ subscriptionId: String })
console.log(props.subscriptionId)

const gatewayProcessor = GatewayProcessor.fromNetworkId(RadixNetwork.Stokenet)

const exires = ref<undefined | Date>()
const apiCallsUsed = ref<undefined | number>()
const totalApiCalls = ref<undefined | number>()
const authorizedPubKey = ref<undefined | string>()

function makeOracleCall() {
  if (account.value !== undefined && account.value.address !== undefined) {
    getPrice().then(async (payload) => {
      const { data, status } = payload
      const xrdAmount = 1 / data.price
      const buyManifest = buyGumballManifest(
        xrdAmount,
        account.value?.address || '',
        priceMsgToString(data),
        data.signature || ''
      )
      console.log(buyManifest);
      const result = await radixDappToolkit.value?.walletApi.sendTransaction({
        transactionManifest: buyManifest,
        version: 1,
        message: 'Buy one Gumball Token for ' + data.price + ' XRD'
      })

      if (result?.isErr()) {
        throw result.error
      }

      console.log('Buy Subscription result:', result)
      updateSubscription()
    })
  }
}

async function updatePublicKeyForSubscription() {
  if (authorizedPubKey.value && account.value && props.subscriptionId) {
    const manifest = updatePublicKey(
      account.value?.address,
      props.subscriptionId,
      authorizedPubKey.value
    )
    console.log(manifest)
    const result = await radixDappToolkit.value?.walletApi.sendTransaction({
      transactionManifest: manifest,
      version: 1,
      message: 'Change the Public Key to ' + authorizedPubKey.value
    })

    if (result?.isErr()) {
      throw result.error
    }

    console.log('Buy Subscription result:', result)
    updateSubscription()
  }
}

function updateSubscription() {
  if (props.subscriptionId) {
    gatewayProcessor
      .getNonFungibleItemsFromIds(import.meta.env.VITE_ORACLE_SUBSCRIPTION_RESOURCE_ADDRESS, [
        props.subscriptionId
      ])
      .then((item) => {
        exires.value = item[0].non_fungible_data?.get('expiration_time')
          ? new Date(Number(item[0].non_fungible_data?.get('expiration_time')) * 1000)
          : undefined

        apiCallsUsed.value = Number(item[0].non_fungible_data?.get('cur_nonce'))
        totalApiCalls.value = Number(item[0].non_fungible_data?.get('max_nonce'))
        authorizedPubKey.value = item[0].non_fungible_data?.get('authorized_pub_key')
      })
  }
}

onMounted(() => {
  updateSubscription()
})
</script>
<template>
  <div class="mt-6">
    <div>ID: {{ subscriptionId }}</div>
    <div>
      Authorized Pub Key: <input type="text" v-model="authorizedPubKey" />
      <button v-on:click="updatePublicKeyForSubscription()">💾</button>
    </div>

    <div>Expires: {{ exires?.toISOString() }}</div>
    <div v-if="apiCallsUsed && totalApiCalls">
      Usage: {{ apiCallsUsed }} of {{ totalApiCalls }} ({{
        ((totalApiCalls - apiCallsUsed) / totalApiCalls) * 100
      }}% remaining)
    </div>
     </div>
</template>
