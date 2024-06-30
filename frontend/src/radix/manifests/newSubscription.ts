export const newSubscription = (
  accountAddress: string,
  xrdAmount: number,
  months: number,
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
    Address("${import.meta.env.VITE_ORACLE_COMPONENT_ADDRESS}")
    "new_subscription"
    ${months}u64
    Bucket("bucket")
;
CALL_METHOD
    Address("${accountAddress}")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;`;
