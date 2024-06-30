<script setup lang="ts">
import WelcomeItem from './WelcomeItem.vue'
import DocumentationIcon from './icons/IconDocumentation.vue'
import ToolingIcon from './icons/IconTooling.vue'
import EcosystemIcon from './icons/IconEcosystem.vue'
import CommunityIcon from './icons/IconCommunity.vue'
import SupportIcon from './icons/IconSupport.vue'
import Subscription from './Oracle/Subscription.vue'
import { useRadixStore } from '@/stores/radixstore'
import { storeToRefs } from 'pinia'
import { GatewayProcessor } from '@beaker-tools/typescript-toolkit'
import { RadixNetwork, type WalletDataStateAccount } from '@radixdlt/radix-dapp-toolkit'
import { ref } from 'vue'

import { newSubscription } from '@/radix/manifests/newSubscription'

const store = useRadixStore()
const { account, radixDappToolkit } = storeToRefs(store)
const gatewayProcessor = GatewayProcessor.fromNetworkId(RadixNetwork.Stokenet)

const subscriptionIds = ref<undefined | string[]>();

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
  updateSubscriptions();
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
    updateSubscriptions();
    }
}
</script>

<template>
  <h2 class="text-4xl font-extrabold dark:text-white">Manage Oracle Subscription</h2>
  <p class="my-4 text-lg text-gray-500">
    Start here for a new subscription or extending an existing subscription. Follow the steps to get
    started.
  </p>

  <ol class="relative border-s border-gray-200 dark:border-gray-700">
    <WelcomeItem>
      <template #icon>
        <DocumentationIcon />
      </template>
      <template #heading>Documentation</template>

      Using the Morpher Oracle is simple: Connect, Buy, Use.
    </WelcomeItem>

    <WelcomeItem>
      <template #icon>
        <ToolingIcon />
      </template>
      <template #heading>Wallet</template>

      Connect your Radix Wallet to get started.
      <br />
      <br />

      <radix-connect-button />
    </WelcomeItem>

    <WelcomeItem v-if="account">
      <template #icon>
        <EcosystemIcon />
      </template>
      <template #heading>Your Account</template>

      <div class="flex flex-col gap-3">
        <div>
          {{ account.address }}
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
    </WelcomeItem>

    <WelcomeItem>
      <template #icon>
        <CommunityIcon />
      </template>
      <template #heading>Community</template>

      Got stuck? Ask your question on
      <a href="https://chat.vuejs.org" target="_blank" rel="noopener">Vue Land</a>, our official
      Discord server, or
      <a href="https://stackoverflow.com/questions/tagged/vue.js" target="_blank" rel="noopener"
        >StackOverflow</a
      >. You should also subscribe to
      <a href="https://news.vuejs.org" target="_blank" rel="noopener">our mailing list</a> and
      follow the official
      <a href="https://twitter.com/vuejs" target="_blank" rel="noopener">@vuejs</a>
      twitter account for latest news in the Vue world.
    </WelcomeItem>

    <WelcomeItem>
      <template #icon>
        <SupportIcon />
      </template>
      <template #heading>Support Vue</template>

      As an independent project, Vue relies on community backing for its sustainability. You can
      help us by
      <a href="https://vuejs.org/sponsor/" target="_blank" rel="noopener">becoming a sponsor</a>.
    </WelcomeItem>
  </ol>
</template>
