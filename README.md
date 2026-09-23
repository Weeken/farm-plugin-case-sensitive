# farm-plugin-case-sensitive

用于 [Farm](https://github.com/farm-fe/farm) 的 Rust 插件，用来检查模块文件路径和目录名的大小写是否与文件系统中的实际名称一致。

当项目中出现以下情况时，插件会让编译失败：

```ts
// 实际文件名：src/utils/Button.tsx
import Button from "./utils/button";
```

错误信息会包含请求路径和文件系统中的实际路径，方便修正导入路径。

## 安装

```bash
pnpm add -D farm-plugin-case-sensitive
```

或：

```bash
npm install -D farm-plugin-case-sensitive
```

## 用法

在 `farm.config.ts` 中引入并注册插件：

```ts
import { defineConfig } from "@farmfe/core";
import caseSensitive from "farm-plugin-case-sensitive";

export default defineConfig({
  plugins: [
    caseSensitive(),
  ],
});
```

插件不需要配置项，调用 `caseSensitive()` 即可启用。

## 检查范围

插件在 Farm 加载模块时检查模块的绝对路径，包括：

- 目录名称大小写
- 文件名称大小写
- JavaScript、TypeScript、CSS、JSON 和静态资源等由 Farm 加载的模块

如果路径大小写正确，插件不会修改模块内容，也不会影响正常构建流程。

## 错误示例

假设磁盘上的文件为：

```text
src/components/Header.tsx
```

代码中使用了错误大小写：

```ts
import Header from "./components/header";
```

构建时会报错：

```text
文件路径大小写不匹配: 请求 '.../src/components/header.tsx', 实际 '.../src/components/Header.tsx'. 请修正 import/require 路径的大小写。
```

## 开发

安装 Rust 工具链后，可以使用以下命令检查和构建插件：

```bash
cargo check
cargo fmt --check
pnpm build
```

## 注意事项

- 插件依赖 Farm 的模块解析结果，因此应在正常的 Farm 配置中使用。
- 建议在大小写不敏感的 macOS 或 Windows 文件系统上启用该插件，以便提前发现部署到 Linux 后可能出现的路径问题。
