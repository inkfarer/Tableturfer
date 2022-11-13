export interface SocketActionMap {
    CreateRoom: never
}

interface SocketMessageMap {
    Error: string
    Welcome: { roomCode: string }
}

type IncomingSocketMessageMap = {
    [K in keyof SocketMessageMap]: { event: K, detail: SocketMessageMap[K] }
};
export type AnySocketMessage = IncomingSocketMessageMap[keyof IncomingSocketMessageMap];
