import type { AxiosResponse } from "axios";
import http from "./api";
import type { Nonce, PriceMessage } from "./types";

export async function getPrice(): Promise<AxiosResponse<PriceMessage>> {
    return await http.get<PriceMessage>(`example/getPrice`)
}

// export async function getNonce(nftId: string) {
//     return await http.get<APIResponse<Nonce>>(`nonce/${nftId}`);
// }