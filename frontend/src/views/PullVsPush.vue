<template>
  <div class="container mx-auto p-4">
    <h1 class="text-3xl font-bold mb-4">Pull vs Push Blockchain Oracles</h1>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="bg-blue-100 p-4 rounded-lg">
        <h2 class="text-xl font-semibold mb-2 text-blue-700">Pull Oracles</h2>
        <ul class="list-disc pl-5">
          <li class="mb-2">DApps or Smart contracts actively request data from the oracle</li>
          <li class="mb-2">Data is fetched on-demand</li>
          <li class="mb-2">More control over when and how data is updated</li>
          <li>Better suited for real-time or time-sensitive applications</li>
        </ul>
      </div>

      <div class="bg-green-100 p-4 rounded-lg">
        <h2 class="text-xl font-semibold mb-2 text-green-700">Push Oracles</h2>
        <ul class="list-disc pl-5">
          <li class="mb-2">Oracles proactively send data to smart contracts</li>
          <li class="mb-2">Data is updated at predefined intervals or thresholds</li>
          <li class="mb-2">More efficient for data that changes at known intervals</li>
          <li>Less suitable for immediate or real-time data needs</li>
        </ul>
      </div>
    </div>

    <p class="mt-4 text-gray-600">
      The choice between Pull and Push oracles depends on the specific requirements of the
      blockchain application, particularly the frequency and timing of data updates needed.
    </p>

    <div class="mt-4 text-gray-600">
      The Morpher Oracle on Radix is a Pull Oracle. You fetch Data off-chain and then use it
      on-chain including an attestation service.

      <ul>
        <li>The process starts with the Smart Contract</li>
        <li>It sends a Data Request to the Oracle</li>
        <li>The Oracle fetches data from the Data Source</li>
        <li>The Data Response is sent back to the Smart Contract</li>
      </ul>
    </div>
  </div>

</template>
