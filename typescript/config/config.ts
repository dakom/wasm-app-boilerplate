export const buildMode = process.env['NODE_ENV'];
export const buildVersion =  process.env['BUILD_VERSION'];
export const isProduction = buildMode === "production" ? true : false;

interface DebugSettings {
    skipStart: boolean;
    muteAudio: boolean;
    maxFractalIterations: number;
}

const devDebugSettings:DebugSettings = {
    skipStart: true,
    //skipStart: false,
    muteAudio: true,
    maxFractalIterations: 25
}

const prodDebugSettings:DebugSettings = {
    skipStart: false,
    muteAudio: false,
    maxFractalIterations: 25 
}

export const debug_settings = isProduction ? prodDebugSettings : devDebugSettings;