export const buildMode = process.env['NODE_ENV'];
export const buildVersion =  process.env['BUILD_VERSION'];
export const isProduction = buildMode === "production" ? true : false;

interface DebugSettings {
    skipStart: boolean;
}

const devDebugSettings:DebugSettings = {
    //skipStart: true 
    skipStart: false
}

const prodDebugSettings:DebugSettings = {
    skipStart: false
}

export const debug_settings = isProduction ? prodDebugSettings : devDebugSettings;