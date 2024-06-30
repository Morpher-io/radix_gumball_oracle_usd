
export const buyStableCoin = (
  accountAddress: string,
  xrdAmount: number,
  tokensToBuy: number,
  message: string,
  signature: string,
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
    Bucket("bucket")
;
CALL_METHOD
    Address("${import.meta.env.VITE_STABLECOIN_COMPONENT_ADDRESS}")
    "buy"
    Decimal("${tokensToBuy}")
    Bucket("bucket")
    "${message}"
    "${signature}"
;
CALL_METHOD
    Address("${accountAddress}")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
`;
