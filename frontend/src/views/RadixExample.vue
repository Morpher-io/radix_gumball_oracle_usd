<script setup lang="ts">
import DocumentationIcon from '@/components/icons/IconDocumentation.vue'
import ToolingIcon from '@/components/icons/IconTooling.vue'
import EcosystemIcon from '@/components/icons/IconEcosystem.vue'
import Subscription from '@/components/Oracle/Subscription.vue'
import { useRadixStore } from '@/stores/radixstore'
import { storeToRefs } from 'pinia'
import { GatewayProcessor } from '@beaker-tools/typescript-toolkit'
import { RadixNetwork, type WalletDataStateAccount } from '@radixdlt/radix-dapp-toolkit'
import { ref } from 'vue'

import { newSubscription } from '@/radix/manifests/newSubscription'

const store = useRadixStore()
const { account, radixDappToolkit } = storeToRefs(store)
const gatewayProcessor = GatewayProcessor.fromNetworkId(RadixNetwork.Stokenet)

const subscriptionIds = ref<undefined | string[]>()

function updateSubscriptions() {
  if (account.value !== undefined) {
    console.log('onmounted finished', account.value)
    gatewayProcessor
      .getNonFungibleIdsHeldBy(
        account.value.address,
        import.meta.env.VITE_ORACLE_SUBSCRIPTION_RESOURCE_ADDRESS
      )
      .then((ids) => {
        subscriptionIds.value = ids
      })
  } else {
    console.log('account value is undefined', account.value)
  }
}
store.$subscribe((mutation, state) => {
  updateSubscriptions()
})

async function buySubscription() {
  // const manifest = subscriptionId.value
  //   ? renewSubscription(account.address, subscription_id, 300, 1)
  //   : newSubscription(account.address, 300, 1);
  if (account.value) {
    const manifest = newSubscription(account.value.address, 1400, 1)

    const result = await radixDappToolkit.value?.walletApi.sendTransaction({
      transactionManifest: manifest,
      version: 1,
      message:
        subscriptionIds.value && subscriptionIds.value.length > 0
          ? 'Renewing your subscription for another month'
          : 'Buying a one-month subscription'
    })

    if (result?.isErr()) {
      throw result.error
    }

    console.log('Buy Subscription result:', result)
    updateSubscriptions()
  }
}
</script>

<template>
  <article class="max-w-4xl">
    <h2 class="text-4xl font-extrabold dark:text-white">Manage Oracle Subscription</h2>
    <p class="my-4 text-lg text-gray-500">
      Start here for a new subscription or extending an existing subscription. Follow the steps to
      get started.
    </p>

    <ol class="relative border-s border-gray-200 dark:border-gray-700">
      <li class="mb-10 ms-6">
        <span
          class="absolute flex items-center justify-center w-6 h-6 rounded-full -start-3 ring-8 ring-white dark:ring-gray-900 dark:bg-blue-900"
        >
          <DocumentationIcon />
        </span>
        <h3 class="flex items-center mb-1 text-lg font-semibold text-gray-900 dark:text-white">
          Documentation
        </h3>
        <p class="mb-4 text-base font-normal text-gray-500 dark:text-gray-400">
          Using the Morpher Oracle is simple: Connect, Buy, Use.
        </p>
      </li>

      <li class="mb-10 ms-6">
        <span
          class="absolute flex items-center justify-center w-6 h-6 rounded-full -start-3 ring-8 ring-white dark:ring-gray-900 dark:bg-blue-900"
        >
          <ToolingIcon />
        </span>
        <h3 class="flex items-center mb-1 text-lg font-semibold text-gray-900 dark:text-white">
          Wallet
        </h3>
        <p class="mb-4 text-base font-normal text-gray-500 dark:text-gray-400">
          Connect your Radix Wallet to get started.

          <radix-connect-button />
        </p>
      </li>

      <li class="mb-10 ms-6">
        <span
          class="absolute flex items-center justify-center w-6 h-6 rounded-full -start-3 ring-8 ring-white dark:ring-gray-900 dark:bg-blue-900"
        >
          <EcosystemIcon />
        </span>
        <h3 class="flex items-center mb-1 text-lg font-semibold text-gray-900 dark:text-white">Your Account</h3>
        <p class="mb-4 text-base font-normal text-gray-500 dark:text-gray-400">

        <div class="flex flex-col gap-3">
          <div>
            {{ account?.address }}
          </div>

          <Subscription
            v-for="subscriptionId in subscriptionIds"
            :key="subscriptionId"
            :subscriptionId
          />
          <div>
            <button
              class="relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-cyan-500 to-blue-500 group-hover:from-cyan-500 group-hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-cyan-200 dark:focus:ring-cyan-800"
              v-on:click="buySubscription()"
            >
              <span
                class="relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0"
              >
                Buy Subscription
              </span>
            </button>
          </div>
        </div>
    </p>
</li>

    
    </ol>
  </article>
</template>
