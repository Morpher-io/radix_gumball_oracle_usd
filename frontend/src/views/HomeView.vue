<script setup lang="ts">
import Oracle from '../components/Oracle.vue'
</script>

<template>
  <article class="max-w-4xl">
    <header class="">
      <p class="text-base font-medium text-slate-500">Getting started</p>

      <h1 class="text-3xl font-bold tracking-tight text-slate-900">Quick start guide</h1>
    </header>

    <p class="mt-8 text-xl text-slate-600">
      What is the
      <span class="font-bold">Morpher <span class="text-primary dark:text-white">Oracle</span></span
      >?
    </p>
    <div class="mt-2">
      In a nutshell, the Oracle Service we provide is a
      <ul class="max-w-xl space-y-1 text-gray-500 list-inside dark:text-gray-400 mt-2">
        <li class="flex items-center">
          <svg
            class="w-3.5 h-3.5 me-2 text-green-500 dark:text-green-400 flex-shrink-0"
            aria-hidden="true"
            xmlns="http://www.w3.org/2000/svg"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z"
            />
          </svg>
          Low Latency Real-Time
        </li>
        <li class="flex items-center">
          <svg
            class="w-3.5 h-3.5 me-2 text-green-500 dark:text-green-400 flex-shrink-0"
            aria-hidden="true"
            xmlns="http://www.w3.org/2000/svg"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z"
            />
          </svg>
          Pull Based On-Chain Attestation Data-Service
        </li>
        <li class="flex items-center">
          <svg
            class="w-3.5 h-3.5 me-2 text-green-500 dark:text-green-400 flex-shrink-0"
            aria-hidden="true"
            xmlns="http://www.w3.org/2000/svg"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z"
            />
          </svg>
          Including Data Feeds for US Equities, Commodities, Forex, Crypto
        </li>
      </ul>
    </div>

    <p class="mt-8 text-xl text-slate-600">Need to get started quickly?</p>

    <a
      href="#"
      class="flex flex-col w-full mt-4 overflow-hidden rounded-lg shadow-xl md:flex-row md:w-4/5"
    >
      <div
        class="flex items-center justify-center w-full px-4 py-16 text-sm font-normal md:w-1/3 text-slate-900/20 bg-gradient-to-br from-indigo-200 via-sky-100 to-indigo-100"
      >
        [Screenshot Image]
      </div>

      <div class="w-full px-6 py-4 border-t border-b border-r md:w-2/3 border-slate-200">
        <h3 class="text-xl font-semibold tracking-tight text-slate-900">Video walkthrough</h3>

        <p class="mt-1 text-lg text-slate-600">
          Watch this 5 minutes video-walkthrough of Spinal. You quickly learn how to set up your
          dashboard, invite team members, set permissions and how to schedule and publish content.
        </p>
      </div>
    </a>

    <h3 class="mt-8 text-base font-bold tracking-tight text-slate-900">Get set up quickly</h3>

    <ul class="mt-4 grid grid-cols-1 gap-6 sm:grid-cols-2">
      <li>
        <a
          href="#"
          class="block px-6 py-4 border rounded bg-slate-50 border-slate-100 hover:bg-white"
        >
          <h4 class="font-bold tracking-tight text-slate-900">Radix DApp Example</h4>

          <p class="text-slate-600">
            We provide an example implementation of a DApp using the Oracle Services
          </p>
        </a>
      </li>

      <li>
        <a
          href="#"
          class="block px-6 py-4 border rounded bg-slate-50 border-slate-100 hover:bg-white"
        >
          <h4 class="font-bold tracking-tight text-slate-900">Subscribe to the Oracle</h4>

          <p class="text-slate-600">Get your Oracle NFT and enroll your Public Key</p>
        </a>
      </li>
    </ul>

    <h3 class="mt-8 text-base font-bold tracking-tight text-slate-900">Have questions?</h3>
    <div class="flex w-full flex-col items-center rounded-lg p-2 md:flex-row text-slate-900">
      <img
        alt="Telegram QR Code Invite"
        src="@/assets/telegram_qr.webp"
        width="757"
        height="757"
        decoding="async"
        data-nimg="future"
        class="h-60 w-60 object-cover p-5 transition hover:drop-shadow-lg"
        loading="lazy"
        style="color: transparent"
      />
      <div class="flex flex-col items-center justify-center pl-2">
        <h4 class="text-xl">Join our Telegram!</h4>
        <p class="mt-3 leading-normal">
          If you want to get in touch with us directly, join our Telegram. Or subscribe to our
          YouTube channel, where we post regular Dev Session Updates.
        </p>
        <a
          href="https://t.me/+p1m8jfBibIU4MGE0"
          target="_blank"
          rel="noreferrer"
          class="mt-6 rounded bg-blue-500 px-4 py-2 text-sm text-white shadow transition hover:bg-blue-900"
          >Join Telegram</a
        >
      </div>
    </div>
  </article>
</template>
