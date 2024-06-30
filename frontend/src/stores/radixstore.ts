
import { defineStore } from 'pinia'
import { RadixDappToolkit, type WalletDataStateAccount } from '@radixdlt/radix-dapp-toolkit'
import { ref } from 'vue'

export const useRadixStore = defineStore('radix', () => {
  let account = ref<undefined|WalletDataStateAccount>()
  let radixDappToolkit = ref<undefined|RadixDappToolkit>()
  
  function setAccount(acc: WalletDataStateAccount) {
    
    account.value = acc;
  }
  function setRdt(rdt: RadixDappToolkit) {
    radixDappToolkit.value = rdt;
  }

  return { account, radixDappToolkit, setAccount, setRdt }
})
