// import {
//   morpherSubscriptionAddress,
//   oracleComponent,
//   xrdAddress,
// } from "../constants";

// export const renewSubscription = (
//   accountAddress: string,
//   subscriptionId: string,
//   xrdAmount: number,
//   months: number,
// ) => `
// CALL_METHOD
//     Address("${accountAddress}")
//     "create_proof_of_non_fungibles"
//     Address("${morpherSubscriptionAddress}")
//     Array<NonFungibleLocalId>(
//         NonFungibleLocalId("${subscriptionId}")
//     )
// ;
// CREATE_PROOF_FROM_AUTH_ZONE_OF_NON_FUNGIBLES
//     Address("${morpherSubscriptionAddress}")
//     Array<NonFungibleLocalId>(
//         NonFungibleLocalId("${subscriptionId}")
//     )
//     Proof("proof")
// ;

// CALL_METHOD
//     Address("${accountAddress}")
//     "withdraw"
//     Address("${xrdAddress}")
//     Decimal("${xrdAmount}")
// ;
// TAKE_FROM_WORKTOP
//     Address("${xrdAddress}")
//     Decimal("${xrdAmount}")
//     Bucket("bucket")
// ;
// CALL_METHOD
//     Address("${oracleComponent}")
//     "renew_subscription"
//     Proof("proof")
//     ${months}u64
//     Bucket("bucket")
// ;
// CALL_METHOD
//     Address("${accountAddress}")
//     "deposit_batch"
//     Expression("ENTIRE_WORKTOP")
// ;`;
