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
letsdmmf --help
```

### Basic

You can specify both local path or remote url.

```shell
letsdmmf my/schema.prisma

letsdmmf https://raw.githubusercontent.com/prisma/prisma-examples/latest/databases/sql-server/prisma/schema.prisma
```

### Examples

This will show you minimal examples of `Prisma schema`.

```shell
letsdmmf example
```

- 1-1
- 1-1-self
- 1-1-multi-field
- 1-n
- 1-n-self
- 1-n-multi-field
- m-n-explicit
- m-n-implicit
- m-n-self
- m-n-self-explicit

### Output

```shell
letsdmmf my/schema.prisma > dmmf.json

letsdmmf my/schema.prisma --output dmmf.json
```

### Option

```shell
letsdmmf my/schema.prisma --scrolloff=3 
letsdmmf my/schema.prisma --mode line
```

See [user guide](https://jless.io/user-guide.html) for details.
