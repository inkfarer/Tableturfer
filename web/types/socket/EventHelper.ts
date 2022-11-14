// Takes a map of event names and parameters and creates a union type of them
// For example: { A: string, B: number } becomes { event: 'A', detail: string } | { event: 'B', detail: number }
export type AnyMessage<MessageMap> = {
    [K in keyof MessageMap]: { event: K, detail: MessageMap[K] }
}[keyof MessageMap];
