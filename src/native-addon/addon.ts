const { existsSync } = require("fs");
const { arch, platform } = require("os");
const { join } = require("path");
/* @internal */
namespace ts {
    export const platformArchTriples: Record<string, any> = Object.freeze({
        darwin: {
            arm64: [
                {
                    platform: "darwin",
                    arch: "arm64",
                    platformArchABI: "darwin-arm64",
                    raw: "aarch64-apple-darwin",
                },
            ],
            x64: [
                {
                    platform: "darwin",
                    arch: "x64",
                    platformArchABI: "darwin-x64",
                    raw: "x86_64-apple-darwin",
                },
            ],
        },
        ios: {
            arm64: [
                {
                    platform: "ios",
                    arch: "arm64",
                    platformArchABI: "ios-arm64",
                    raw: "aarch64-apple-ios",
                },
            ],
            x64: [
                {
                    platform: "ios",
                    arch: "x64",
                    platformArchABI: "ios-x64",
                    raw: "x86_64-apple-ios",
                },
            ],
        },
        fuchsia: {
            arm64: [
                {
                    platform: "fuchsia",
                    arch: "arm64",
                    platformArchABI: "fuchsia-arm64",
                    raw: "aarch64-fuchsia",
                },
            ],
            x64: [
                {
                    platform: "fuchsia",
                    arch: "x64",
                    platformArchABI: "fuchsia-x64",
                    raw: "x86_64-fuchsia",
                },
            ],
        },
        android: {
            arm64: [
                {
                    platform: "android",
                    arch: "arm64",
                    platformArchABI: "android-arm64",
                    raw: "aarch64-linux-android",
                },
            ],
            ia32: [
                {
                    platform: "android",
                    arch: "ia32",
                    platformArchABI: "android-ia32",
                    raw: "i686-linux-android",
                },
            ],
            x64: [
                {
                    platform: "android",
                    arch: "x64",
                    platformArchABI: "android-x64",
                    raw: "x86_64-linux-android",
                },
            ],
        },
        win32: {
            arm64: [
                {
                    platform: "win32",
                    arch: "arm64",
                    abi: "msvc",
                    platformArchABI: "win32-arm64-msvc",
                    raw: "aarch64-pc-windows-msvc",
                },
            ],
            i586: [
                {
                    platform: "win32",
                    arch: "i586",
                    abi: "msvc",
                    platformArchABI: "win32-i586-msvc",
                    raw: "i586-pc-windows-msvc",
                },
            ],
            ia32: [
                {
                    platform: "win32",
                    arch: "ia32",
                    abi: "gnu",
                    platformArchABI: "win32-ia32-gnu",
                    raw: "i686-pc-windows-gnu",
                },
                {
                    platform: "win32",
                    arch: "ia32",
                    abi: "msvc",
                    platformArchABI: "win32-ia32-msvc",
                    raw: "i686-pc-windows-msvc",
                },
            ],
            x64: [
                {
                    platform: "win32",
                    arch: "x64",
                    abi: "gnu",
                    platformArchABI: "win32-x64-gnu",
                    raw: "x86_64-pc-windows-gnu",
                },
                {
                    platform: "win32",
                    arch: "x64",
                    abi: "msvc",
                    platformArchABI: "win32-x64-msvc",
                    raw: "x86_64-pc-windows-msvc",
                },
            ],
        },
        linux: {
            arm64: [
                {
                    platform: "linux",
                    arch: "arm64",
                    abi: "gnu",
                    platformArchABI: "linux-arm64-gnu",
                    raw: "aarch64-unknown-linux-gnu",
                },
                {
                    platform: "linux",
                    arch: "arm64",
                    abi: "musl",
                    platformArchABI: "linux-arm64-musl",
                    raw: "aarch64-unknown-linux-musl",
                },
            ],
            arm: [
                {
                    platform: "linux",
                    arch: "arm",
                    abi: "gnueabi",
                    platformArchABI: "linux-arm-gnueabi",
                    raw: "arm-unknown-linux-gnueabi",
                },
                {
                    platform: "linux",
                    arch: "arm",
                    abi: "gnueabihf",
                    platformArchABI: "linux-arm-gnueabihf",
                    raw: "arm-unknown-linux-gnueabihf",
                },
                {
                    platform: "linux",
                    arch: "arm",
                    abi: "musleabi",
                    platformArchABI: "linux-arm-musleabi",
                    raw: "arm-unknown-linux-musleabi",
                },
                {
                    platform: "linux",
                    arch: "arm",
                    abi: "musleabihf",
                    platformArchABI: "linux-arm-musleabihf",
                    raw: "arm-unknown-linux-musleabihf",
                },
                {
                    platform: "linux",
                    arch: "arm",
                    abi: "gnueabi",
                    platformArchABI: "linux-arm-gnueabi",
                    raw: "armv7-unknown-linux-gnueabi",
                },
                {
                    platform: "linux",
                    arch: "arm",
                    abi: "gnueabihf",
                    platformArchABI: "linux-arm-gnueabihf",
                    raw: "armv7-unknown-linux-gnueabihf",
                },
                {
                    platform: "linux",
                    arch: "arm",
                    abi: "musleabi",
                    platformArchABI: "linux-arm-musleabi",
                    raw: "armv7-unknown-linux-musleabi",
                },
                {
                    platform: "linux",
                    arch: "arm",
                    abi: "musleabihf",
                    platformArchABI: "linux-arm-musleabihf",
                    raw: "armv7-unknown-linux-musleabihf",
                },
            ],
            armv5te: [
                {
                    platform: "linux",
                    arch: "armv5te",
                    abi: "gnueabi",
                    platformArchABI: "linux-armv5te-gnueabi",
                    raw: "armv5te-unknown-linux-gnueabi",
                },
                {
                    platform: "linux",
                    arch: "armv5te",
                    abi: "musleabi",
                    platformArchABI: "linux-armv5te-musleabi",
                    raw: "armv5te-unknown-linux-musleabi",
                },
            ],
            i586: [
                {
                    platform: "linux",
                    arch: "i586",
                    abi: "gnu",
                    platformArchABI: "linux-i586-gnu",
                    raw: "i586-unknown-linux-gnu",
                },
                {
                    platform: "linux",
                    arch: "i586",
                    abi: "musl",
                    platformArchABI: "linux-i586-musl",
                    raw: "i586-unknown-linux-musl",
                },
            ],
            ia32: [
                {
                    platform: "linux",
                    arch: "ia32",
                    abi: "gnu",
                    platformArchABI: "linux-ia32-gnu",
                    raw: "i686-unknown-linux-gnu",
                },
                {
                    platform: "linux",
                    arch: "ia32",
                    abi: "musl",
                    platformArchABI: "linux-ia32-musl",
                    raw: "i686-unknown-linux-musl",
                },
            ],
            mips: [
                {
                    platform: "linux",
                    arch: "mips",
                    abi: "gnu",
                    platformArchABI: "linux-mips-gnu",
                    raw: "mips-unknown-linux-gnu",
                },
                {
                    platform: "linux",
                    arch: "mips",
                    abi: "musl",
                    platformArchABI: "linux-mips-musl",
                    raw: "mips-unknown-linux-musl",
                },
            ],
            mips64: [
                {
                    platform: "linux",
                    arch: "mips64",
                    abi: "gnuabi64",
                    platformArchABI: "linux-mips64-gnuabi64",
                    raw: "mips64-unknown-linux-gnuabi64",
                },
                {
                    platform: "linux",
                    arch: "mips64",
                    abi: "muslabi64",
                    platformArchABI: "linux-mips64-muslabi64",
                    raw: "mips64-unknown-linux-muslabi64",
                },
            ],
            mips64el: [
                {
                    platform: "linux",
                    arch: "mips64el",
                    abi: "gnuabi64",
                    platformArchABI: "linux-mips64el-gnuabi64",
                    raw: "mips64el-unknown-linux-gnuabi64",
                },
                {
                    platform: "linux",
                    arch: "mips64el",
                    abi: "muslabi64",
                    platformArchABI: "linux-mips64el-muslabi64",
                    raw: "mips64el-unknown-linux-muslabi64",
                },
            ],
            mipsel: [
                {
                    platform: "linux",
                    arch: "mipsel",
                    abi: "gnu",
                    platformArchABI: "linux-mipsel-gnu",
                    raw: "mipsel-unknown-linux-gnu",
                },
                {
                    platform: "linux",
                    arch: "mipsel",
                    abi: "musl",
                    platformArchABI: "linux-mipsel-musl",
                    raw: "mipsel-unknown-linux-musl",
                },
            ],
            powerpc: [
                {
                    platform: "linux",
                    arch: "powerpc",
                    abi: "gnu",
                    platformArchABI: "linux-powerpc-gnu",
                    raw: "powerpc-unknown-linux-gnu",
                },
            ],
            powerpc64: [
                {
                    platform: "linux",
                    arch: "powerpc64",
                    abi: "gnu",
                    platformArchABI: "linux-powerpc64-gnu",
                    raw: "powerpc64-unknown-linux-gnu",
                },
            ],
            powerpc64le: [
                {
                    platform: "linux",
                    arch: "powerpc64le",
                    abi: "gnu",
                    platformArchABI: "linux-powerpc64le-gnu",
                    raw: "powerpc64le-unknown-linux-gnu",
                },
            ],
            riscv64gc: [
                {
                    platform: "linux",
                    arch: "riscv64gc",
                    abi: "gnu",
                    platformArchABI: "linux-riscv64gc-gnu",
                    raw: "riscv64gc-unknown-linux-gnu",
                },
            ],
            s390x: [
                {
                    platform: "linux",
                    arch: "s390x",
                    abi: "gnu",
                    platformArchABI: "linux-s390x-gnu",
                    raw: "s390x-unknown-linux-gnu",
                },
            ],
            sparc64: [
                {
                    platform: "linux",
                    arch: "sparc64",
                    abi: "gnu",
                    platformArchABI: "linux-sparc64-gnu",
                    raw: "sparc64-unknown-linux-gnu",
                },
            ],
            thumbv7neon: [
                {
                    platform: "linux",
                    arch: "thumbv7neon",
                    abi: "gnueabihf",
                    platformArchABI: "linux-thumbv7neon-gnueabihf",
                    raw: "thumbv7neon-unknown-linux-gnueabihf",
                },
            ],
            x64: [
                {
                    platform: "linux",
                    arch: "x64",
                    abi: "gnu",
                    platformArchABI: "linux-x64-gnu",
                    raw: "x86_64-unknown-linux-gnu",
                },
                {
                    platform: "linux",
                    arch: "x64",
                    abi: "gnux32",
                    platformArchABI: "linux-x64-gnux32",
                    raw: "x86_64-unknown-linux-gnux32",
                },
                {
                    platform: "linux",
                    arch: "x64",
                    abi: "musl",
                    platformArchABI: "linux-x64-musl",
                    raw: "x86_64-unknown-linux-musl",
                },
            ],
        },
        none: {
            arm64: [
                {
                    platform: "none",
                    arch: "arm64",
                    platformArchABI: "none-arm64",
                    raw: "aarch64-unknown-none",
                },
                {
                    platform: "none",
                    arch: "arm64",
                    abi: "softfloat",
                    platformArchABI: "none-arm64-softfloat",
                    raw: "aarch64-unknown-none-softfloat",
                },
            ],
            riscv32i: [
                {
                    platform: "none",
                    arch: "riscv32i",
                    abi: "elf",
                    platformArchABI: "none-riscv32i-elf",
                    raw: "riscv32i-unknown-none-elf",
                },
            ],
            riscv32imac: [
                {
                    platform: "none",
                    arch: "riscv32imac",
                    abi: "elf",
                    platformArchABI: "none-riscv32imac-elf",
                    raw: "riscv32imac-unknown-none-elf",
                },
            ],
            riscv32imc: [
                {
                    platform: "none",
                    arch: "riscv32imc",
                    abi: "elf",
                    platformArchABI: "none-riscv32imc-elf",
                    raw: "riscv32imc-unknown-none-elf",
                },
            ],
            riscv64gc: [
                {
                    platform: "none",
                    arch: "riscv64gc",
                    abi: "elf",
                    platformArchABI: "none-riscv64gc-elf",
                    raw: "riscv64gc-unknown-none-elf",
                },
            ],
            riscv64imac: [
                {
                    platform: "none",
                    arch: "riscv64imac",
                    abi: "elf",
                    platformArchABI: "none-riscv64imac-elf",
                    raw: "riscv64imac-unknown-none-elf",
                },
            ],
        },
        androideabi: {
            arm: [
                {
                    platform: "androideabi",
                    arch: "arm",
                    platformArchABI: "androideabi-arm",
                    raw: "arm-linux-androideabi",
                },
                {
                    platform: "androideabi",
                    arch: "arm",
                    platformArchABI: "androideabi-arm",
                    raw: "armv7-linux-androideabi",
                },
            ],
            thumbv7neon: [
                {
                    platform: "androideabi",
                    arch: "thumbv7neon",
                    platformArchABI: "androideabi-thumbv7neon",
                    raw: "thumbv7neon-linux-androideabi",
                },
            ],
        },
        eabi: {
            "armebv7r": [
                {
                    platform: "eabi",
                    arch: "armebv7r",
                    platformArchABI: "eabi-armebv7r",
                    raw: "armebv7r-none-eabi",
                },
            ],
            "armv7a": [
                {
                    platform: "eabi",
                    arch: "armv7a",
                    platformArchABI: "eabi-armv7a",
                    raw: "armv7a-none-eabi",
                },
            ],
            "armv7r": [
                {
                    platform: "eabi",
                    arch: "armv7r",
                    platformArchABI: "eabi-armv7r",
                    raw: "armv7r-none-eabi",
                },
            ],
            "thumbv6m": [
                {
                    platform: "eabi",
                    arch: "thumbv6m",
                    platformArchABI: "eabi-thumbv6m",
                    raw: "thumbv6m-none-eabi",
                },
            ],
            "thumbv7em": [
                {
                    platform: "eabi",
                    arch: "thumbv7em",
                    platformArchABI: "eabi-thumbv7em",
                    raw: "thumbv7em-none-eabi",
                },
            ],
            "thumbv7m": [
                {
                    platform: "eabi",
                    arch: "thumbv7m",
                    platformArchABI: "eabi-thumbv7m",
                    raw: "thumbv7m-none-eabi",
                },
            ],
            "thumbv8m.base": [
                {
                    platform: "eabi",
                    arch: "thumbv8m.base",
                    platformArchABI: "eabi-thumbv8m.base",
                    raw: "thumbv8m.base-none-eabi",
                },
            ],
            "thumbv8m.main": [
                {
                    platform: "eabi",
                    arch: "thumbv8m.main",
                    platformArchABI: "eabi-thumbv8m.main",
                    raw: "thumbv8m.main-none-eabi",
                },
            ],
        },
        eabihf: {
            "armebv7r": [
                {
                    platform: "eabihf",
                    arch: "armebv7r",
                    platformArchABI: "eabihf-armebv7r",
                    raw: "armebv7r-none-eabihf",
                },
            ],
            "armv7r": [
                {
                    platform: "eabihf",
                    arch: "armv7r",
                    platformArchABI: "eabihf-armv7r",
                    raw: "armv7r-none-eabihf",
                },
            ],
            "thumbv7em": [
                {
                    platform: "eabihf",
                    arch: "thumbv7em",
                    platformArchABI: "eabihf-thumbv7em",
                    raw: "thumbv7em-none-eabihf",
                },
            ],
            "thumbv8m.main": [
                {
                    platform: "eabihf",
                    arch: "thumbv8m.main",
                    platformArchABI: "eabihf-thumbv8m.main",
                    raw: "thumbv8m.main-none-eabihf",
                },
            ],
        },
        emscripten: {
            asmjs: [
                {
                    platform: "emscripten",
                    arch: "asmjs",
                    platformArchABI: "emscripten-asmjs",
                    raw: "asmjs-unknown-emscripten",
                },
            ],
        },
        freebsd: {
            ia32: [
                {
                    platform: "freebsd",
                    arch: "ia32",
                    platformArchABI: "freebsd-ia32",
                    raw: "i686-unknown-freebsd",
                },
            ],
            x64: [
                {
                    platform: "freebsd",
                    arch: "x64",
                    platformArchABI: "freebsd-x64",
                    raw: "x86_64-unknown-freebsd",
                },
            ],
        },
        cuda: {
            nvptx64: [
                {
                    platform: "cuda",
                    arch: "nvptx64",
                    platformArchABI: "cuda-nvptx64",
                    raw: "nvptx64-nvidia-cuda",
                },
            ],
        },
        solaris: {
            sparcv9: [
                {
                    platform: "solaris",
                    arch: "sparcv9",
                    platformArchABI: "solaris-sparcv9",
                    raw: "sparcv9-sun-solaris",
                },
            ],
            x64: [
                {
                    platform: "solaris",
                    arch: "x64",
                    platformArchABI: "solaris-x64",
                    raw: "x86_64-sun-solaris",
                },
            ],
        },
        unknown: {
            x64: [
                {
                    platform: "unknown",
                    arch: "x64",
                    abi: "sgx",
                    platformArchABI: "unknown-x64-sgx",
                    raw: "x86_64-fortanix-unknown-sgx",
                },
            ],
        },
        netbsd: {
            x64: [
                {
                    platform: "netbsd",
                    arch: "x64",
                    platformArchABI: "netbsd-x64",
                    raw: "x86_64-rumprun-netbsd",
                },
                {
                    platform: "netbsd",
                    arch: "x64",
                    platformArchABI: "netbsd-x64",
                    raw: "x86_64-unknown-netbsd",
                },
            ],
        },
        illumos: {
            x64: [
                {
                    platform: "illumos",
                    arch: "x64",
                    platformArchABI: "illumos-x64",
                    raw: "x86_64-unknown-illumos",
                },
            ],
        },
        redox: {
            x64: [
                {
                    platform: "redox",
                    arch: "x64",
                    platformArchABI: "redox-x64",
                    raw: "x86_64-unknown-redox",
                },
            ],
        },
    });

    export function loadBinding(
        dirname: string,
        filename = "index",
        packageName?: string
    ) {
        const platformName = platform();
        const archName = arch();
        const triples = platformArchTriples[platformName][archName];
        for (const triple of triples) {
            // resolve in node_modules
            if (packageName) {
                return require(require.resolve(
                    `${packageName}-${triple.platformArchABI}`,
                    { paths: [dirname] }
                ));
                // eslint-disable-next-line no-empty
            }
            const localFilePath = join(
                dirname,
                `${filename}.${triple.platformArchABI}.node`
            );
            if (existsSync(localFilePath)) {
                return require(localFilePath);
            }
        }

        const errorMsg = `Can not find node binding files from ${
            packageName
                ? triples
                      .map(
                          (triple: Record<string, string>) =>
                              `${packageName}-${triple.platformArchABI}`
                      )
                      .join(", ")
                : ""
        } ${packageName ? "and " : ""}${triples
            .map((triple: Record<string, string>) =>
                join(dirname, `${filename}.${triple.platformArchABI}.node`)
            )
            .join(", ")}`;

        throw new TypeError(errorMsg);
    }

    // exported addon properties, function, classes, values
    export interface AddonExports {
        lookupInUnicodeMap(code: number, map: readonly number[]): boolean;
    }
    export const native: AddonExports = loadBinding(
        join(__dirname, "..", "..", "src-native"),
        "typescript",
        "typescript"
    );
}
