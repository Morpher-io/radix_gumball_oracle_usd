import type { PriceMessage } from "@/axios/types";

export function priceMsgToString(msg: PriceMessage): string {
    return msg.marketId+"-"+msg.price+"-"+msg.nonce+"-"+msg.createdAt;
}