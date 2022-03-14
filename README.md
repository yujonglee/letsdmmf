# Let's DMMF
[![release](https://github.com/yujong-lee/letsdmmf/actions/workflows/ci.yml/badge.svg)](https://github.com/yujong-lee/letsdmmf/actions/workflows/ci.yml)

Traverse `DMMF` of your `Prisma schema`, in your terminal.

Powered by [`jless`](https://github.com/PaulJuliusMartinez/jless).

## Installation
```shell
brew tap yujong-lee/tap
brew install letsdmmf
```

## Usage
See [user guide](https://jless.io/user-guide.html) for details.

```shell
# letsdmmf --help
letsdmmf my/schema.prisma
letsdmmf my/schema.prisma -m=line
letsdmmf my/schema.prisma -M data
letsdmmf my/schema.prisma --scrolloff=3
letsdmmf my/schema.prisma --scrolloff 3
```
