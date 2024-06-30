


// export type APIResponse<T> = {
//     success: boolean
//     data: T;
//     status?: number;
// }

export type Nonce = {
    nonce: number;
};

export type PriceMessage = {
    marketId: string;
    price: number;
    nonce: string;
    createdAt: number;
    signature?: string;
}