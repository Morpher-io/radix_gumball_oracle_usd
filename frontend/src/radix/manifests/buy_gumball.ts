export const buyGumballManifest = (
    xrdAmount: number,
    accountAddress: string,
    priceMsg: string,
    signature: string
  ) => `
  CALL_METHOD
      Address("${accountAddress}")
      "withdraw"
      Address("${import.meta.env.VITE_XRD_ARRESS}")
      Decimal("${xrdAmount}")
  ;
  TAKE_FROM_WORKTOP
      Address("${import.meta.env.VITE_XRD_ARRESS}")
      Decimal("${xrdAmount}")
      Bucket("bucket_of_xrd")
  ;
  CALL_METHOD
      Address("${import.meta.env.VITE_GUMBALL_COMPONENT_ADDRESS}")
      "buy_gumball"
      Bucket("bucket_of_xrd")
      "${priceMsg}"
      "${signature}"
  ;
  CALL_METHOD
      Address("${accountAddress}")
      "deposit_batch"
      Expression("ENTIRE_WORKTOP")
  ;`;