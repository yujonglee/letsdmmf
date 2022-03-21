# Let's DMMF
[![release](https://github.com/yujong-lee/letsdmmf/actions/workflows/ci.yml/badge.svg)](https://github.com/yujong-lee/letsdmmf/actions/workflows/ci.yml)

Traverse `DMMF` of `Prisma schema`, in your terminal.

Powered by [`jless`](https://github.com/PaulJuliusMartinez/jless).

## Installation
```shell
brew tap yujong-lee/tap
brew install letsdmmf
```

## Usage

```shell
# letsdmmf --help

letsdmmf example

letsdmmf https://raw.githubusercontent.com/prisma/prisma-examples/latest/databases/sql-server/prisma/schema.prisma

letsdmmf my/schema.prisma

letsdmmf my/schema.prisma -m=line
letsdmmf my/schema.prisma -M data

letsdmmf my/schema.prisma --scrolloff=3
letsdmmf my/schema.prisma --scrolloff 3

```
See [user guide](https://jless.io/user-guide.html) for details.
