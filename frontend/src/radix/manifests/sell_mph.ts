
export const sellMph = (
  accountAddress: string,
  tokenAmount: number,
  message: string,
  signature: string,
) => `
CALL_METHOD
    Address("${accountAddress}")
    "withdraw"
    Address("${import.meta.env.VITE_STABLECOIN_RESOURCE_ADDRESS}")
    Decimal("${tokenAmount}")
;
TAKE_FROM_WORKTOP
    Address("${import.meta.env.VITE_STABLECOIN_RESOURCE_ADDRESS}")
    Decimal("${tokenAmount}")
    Bucket("bucket")
;
CALL_METHOD
    Address("${import.meta.env.VITE_STABLECOIN_COMPONENT_ADDRESS}")
    "sell"
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

