/**
 * Customize this for all the bridge event types
 * If there are any complex objects, create structs on the Rust side too!
 */

//The order of these must match the Rust BridgeEventIndex!
export enum BridgeEvent {
    ToggleAudio,
    Speed,
    WindowSize,
    AssetsLoaded,
    RenderUi
}

export type ValidBridgeEvents = 
    | BridgeEvent.ToggleAudio
    | [BridgeEvent.Speed, number]
    | [BridgeEvent.WindowSize, WindowSize]
    | BridgeEvent.AssetsLoaded
    | BridgeEvent.RenderUi

interface WindowSize{
    width: number;
    height: number;
}
