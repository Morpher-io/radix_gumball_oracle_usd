export const updatePublicKey = (
    accountAddress: string,
    nonFungibleId: string,
    newPublicKey: string,
  ) => `
CALL_METHOD
    Address("${accountAddress}")
    "create_proof_of_non_fungibles"
    Address("${import.meta.env.VITE_ORACLE_SUBSCRIPTION_RESOURCE_ADDRESS}")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId("${nonFungibleId}")
    )
;
POP_FROM_AUTH_ZONE
    Proof("Sub Proof")
;
CALL_METHOD
    Address("${import.meta.env.VITE_ORACLE_COMPONENT_ADDRESS}")
    "update_subscription_pub_key"
    "${newPublicKey}"
    Proof("Sub Proof")
;
CALL_METHOD
    Address("${accountAddress}")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
`