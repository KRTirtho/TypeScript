/* @internal */
namespace ts{
    // exported addon properties, function, classes, values
    export interface AddonExports{
        lookupInUnicodeMap(code: number, map: readonly number[]): boolean;
    }
    export const native: AddonExports = require("../../index.node");

}
