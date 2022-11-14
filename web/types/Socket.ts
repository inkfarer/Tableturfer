export interface SocketActionMap {
    CreateRoom: never
}

export interface SocketUser {
    joinedAt: string
}

export interface SocketMessageMap {
    Error: string
    Welcome: {
        id: string,
        roomCode: string,
        users: Record<string, SocketUser>,
        owner: string
    }
    UserJoin: { id: string, user: SocketUser }
    UserLeave: string
    OwnerChange: string
}

type IncomingSocketMessageMap = {
    [K in keyof SocketMessageMap]: { event: K, detail: SocketMessageMap[K] }
};
export type AnySocketMessage = IncomingSocketMessageMap[keyof IncomingSocketMessageMap];
