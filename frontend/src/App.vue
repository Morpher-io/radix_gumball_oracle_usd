<script setup lang="ts">
import { RouterLink, RouterView, useRoute, useRouter } from 'vue-router'

import { DataRequestBuilder, RadixDappToolkit, RadixNetwork } from '@radixdlt/radix-dapp-toolkit'

import { useRadixStore } from '@/stores/radixstore'
import { onMounted, ref, watch, type Ref } from 'vue'
import { initFlowbite } from 'flowbite'

import MorpherLogo from '@/components/icons/MorpherLogo.vue'

onMounted(() => {
  initFlowbite()

  const dappConfig = {
    networkId: RadixNetwork.Stokenet,
    applicationVersion: '1.0.0',
    applicationName: 'Morpher',
    applicationDappDefinitionAddress:
      'account_tdx_2_12xmevme9ujzqe3yuyq37ampaa2dw633luw8446gumfycltqe5qty66'
  }

  const store = useRadixStore()

  // Instantiate Radix Dapp Toolkit to connect to the Radix wallet
  const rdt = RadixDappToolkit(dappConfig)

  // Instantiate Gateway processor client to query the Radix network
  rdt.walletApi.setRequestData(DataRequestBuilder.accounts().atLeast(1))

  // Subscribe to updates to the user's shared wallet data, then display the account name and address.
  rdt.walletApi.walletData$.subscribe((walletData) => {
    // Set the account variable to the first and only connected account from the wallet
    const acc = walletData.accounts[0]

    store.setAccount(acc)
  })

  store.setRdt(rdt)
})
</script>

<template>
  <nav
    class="sticky top-0 z-50 px-3 py-2 border-b shadow-lg bg-white/90 backdrop-blur-sm border-slate-400/40"
  >
    <div class="flex flex-wrap items-center justify-between mx-auto p-4">
      <a href="/" class="flex items-center space-x-3 rtl:space-x-reverse">
        <MorpherLogo />
        <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white"
          >Oracle</span
        >
      </a>
      <button
        data-collapse-toggle="navbar-default"
        type="button"
        class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
        aria-controls="navbar-default"
        aria-expanded="false"
      >
        <span class="sr-only">Open main menu</span>
        <svg
          class="w-5 h-5"
          aria-hidden="true"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 17 14"
        >
          <path
            stroke="currentColor"
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M1 1h15M1 7h15M1 13h15"
          />
        </svg>
      </button>
      <div class="hidden w-full md:block md:w-auto" id="navbar-default">
        <ul
          class="font-medium flex flex-col p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700"
        >
          <li>
            <RouterLink
              to="/"
              class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
              activeClass="text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white md:dark:text-blue-500"
              aria-current="page"
            >
              Home
            </RouterLink>
          </li>
          <li>
            <RouterLink
              to="/radix/setup"
              class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
              activeClass="text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white md:dark:text-blue-500"
              aria-current="page"
            >
              Radix
            </RouterLink>
          </li>
          <li>
            <a
              href="#"
              class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
              >Pricing</a
            >
          </li>
          <li>
            <a
              href="#"
              class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
              >Contact</a
            >
          </li>
        </ul>
      </div>
    </div>
  </nav>

  <main class="relative flex justify-center mx-auto max-w-8xl sm:px-2 lg:px-8 xl:px-12">
    <label
      for="navigation"
      class="fixed bottom-0 left-0 z-50 flex items-center justify-center w-12 h-12 mb-4 ml-4 bg-white border rounded-full shadow-lg cursor-pointer text-slate-600 border-slate-300 lg:hidden transition duration-200 ease-in-out active:scale-95"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="w-6 h-6"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        stroke-width="2"
      >
        <path stroke-linecap="round" stroke-linejoin="round" d="M4 8h16M4 16h16" />
      </svg>
    </label>

    <input type="checkbox" name="navigation" id="navigation" class="hidden peer" />
    <div
      class="fixed top-[3.5rem] h-screen shadow-xl px-4 left-0 hidden peer-checked:block lg:relative lg:top-0 lg:h-auto lg:px-0 lg:block lg:flex-none lg:shadow-none"
    >
      <div class="absolute inset-y-0 right-0 w-full lg:w-[50vw] bg-white lg:bg-slate-50"></div>

      <nav class="sticky top-[4.5rem] w-64 pr-8 text-base lg:text-sm xl:w-72 xl:pr-16">
        <ul
          role="list"
          class="-ml-0.5 h-[calc(100vh-4.5rem)] overflow-y-auto py-7 pl-0.5 space-y-8"
        >
          <li>
            <h3 class="font-semibold tracking-tight text-slate-900">Getting started</h3>

            <ul role="list" class="pl-3 mt-3 space-y-2">
              <li>
                <RouterLink
                  to="/"
                  class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
                  activeClass="text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white md:dark:text-blue-500"
                  aria-current="page"
                >
                  What is the Morpher Oracle and how does it work?
                </RouterLink>
                <a href="#" class="text-slate-900 hover:text-slate-800"> </a>
              </li>

              <li>
                <RouterLink
                  to="/pull-vs-push-oracles"
                  class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
                  activeClass="text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white md:dark:text-blue-500"
                  aria-current="page"
                >
                  Pull vs Push vs Injection Oracles
                </RouterLink>
              </li>
            </ul>
          </li>

          <li>
            <h3 class="font-semibold tracking-tight text-slate-900">Radix</h3>

            <ul role="list" class="pl-3 mt-3 space-y-2">
              <li>
                <RouterLink
                  to="/radix/setup"
                  class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
                  activeClass="text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white md:dark:text-blue-500"
                  aria-current="page"
                >
                  Getting Started with Radix
                </RouterLink>
              </li>
              <li>
                <RouterLink
                  to="/radix/example"
                  class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
                  activeClass="text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white md:dark:text-blue-500"
                  aria-current="page"
                >
                  Gumball Machine in USD
                </RouterLink>
              </li>
            </ul>
          </li>

          <li>
            <h3 class="font-semibold tracking-tight text-slate-900">Content</h3>

            <ul role="list" class="pl-3 mt-3 space-y-2">
              <li>
                <a href="#" class="text-slate-600 hover:text-slate-800">
                  What kind of content can I create and edit?
                </a>
              </li>

              <li>
                <a href="#" class="text-slate-600 hover:text-slate-800"> Previewing content </a>
              </li>
            </ul>
          </li>
        </ul>
      </nav>
    </div>

    <div class="flex-auto max-w-2xl min-w-0 px-4 py-7 lg:max-w-none lg:pr-0 lg:pl-8 xl:px-16">
      <router-view v-slot="{ Component }">
        <transition name="slide-fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>

      <dl class="flex pt-6 mt-6 border-t border-slate-200">
        <!-- <div class="mr-auto text-left"> -->
        <!--   <dt class="text-sm font-normal tracking-tight text-slate-600"> -->
        <!--     Previous -->
        <!--   </dt> -->

        <!--   <dd class="mt-1"> -->
        <!--     <a href="#" class="text-base font-semibold text-slate-900 hover:underline"> -->
        <!--       Blah -->
        <!--     </a> -->
        <!--   </dd> -->
        <!-- </div> -->

        <!-- <div class="ml-auto text-right">
          <dt class="text-sm font-normal tracking-tight text-slate-600">Next</dt>

          <dd class="mt-1">
            <a href="#" class="text-base font-semibold text-slate-900 hover:underline">
              How does Spinal work?
            </a>
          </dd>
        </div> -->
      </dl>
    </div>
  </main>
</template>

<style lang="css">
/*
  Enter and leave animations can use different
  durations and timing functions.
*/
.slide-fade-enter-active {
  transition: all 0.3s ease-out;
}

.slide-fade-leave-active {
  transition: all 0.3s cubic-bezier(1, 0.5, 0.8, 1);
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateX(20px);
  opacity: 0;
}
</style>
