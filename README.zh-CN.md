# The You Beyond

透明 AI 桌宠，拥有可自定义角色、精灵图动画、世界事件、连续记忆与动态成长。

## 功能

- 透明、无边框、置顶桌宠窗口
- 可独立显示或隐藏的事件栏
- 桌宠窗口支持拖动和缩放
- 可自定义人物名称、性格、经历、兴趣、技能和物品
- 支持透明 PNG 精灵图
- 可配置精灵图切分网格和动画帧率
- 支持能量、心情、体力、智力、好奇心、社交、创造力和勇气
- 五维属性雷达图和长期成长
- 基于当前世界状态生成 AI 日常事件
- 后端控制事件类型、地点、效果和事件连续性
- 主事件与嵌套子进展
- 事件去重、JSON 类型修复、冷却和规则校验
- 人物、人际、地图、物品、技能、目标和记忆管理
- 支持自定义 OpenAI-compatible API、模型和 API Key
- SQLite 本地保存，Markdown 文件可读
- 支持生成 Windows NSIS 和 MSI 安装包

## 核心架构

```text
真实时间
   |
世界调度器
   |
后端事件导演
   |  决定事件类型、连续性、地点和持续时间
AI 内容生成器
   |  生成具体事件文本和效果
后端校验
   |  校验 JSON、参与者、效果、地点和重复内容
世界引擎
   |
SQLite + Markdown + React 界面
```

后端是修改世界状态的唯一 authority。AI 只能在后端给定的条件下生成内容。

## 环境要求

- Windows 10/11 64 位
- Node.js 18 或更高版本
- npm
- Rust stable 和 Cargo
- WebView2
- Visual Studio C++ Build Tools

## 开发运行

在项目根目录执行：

```powershell
npm.cmd install
npm.cmd run dev
```

运行 Tauri 桌面应用：

```powershell
npm.cmd run tauri dev
```

运行检查：

```powershell
npm.cmd run build
npm.cmd test
cargo test --manifest-path src-tauri/Cargo.toml
```

## AI 配置

在 Settings 中配置：

- `Base URL`，例如 `https://example.com/v1`
- `Model`
- `API Key`
- 输出语言

Provider 使用兼容 OpenAI 的接口：

```text
POST /chat/completions
```

请求超时时间为 120 秒。应用会归一化常见的 Provider 响应格式，并在写入世界前校验事件结构。

不要将 API Key 提交到 Git 仓库。发布软件或公开截图时，请隐藏 API Key。

## 精灵图

从 Settings 导入透明 PNG 精灵图。

当前支持的网格配置包括：

- 8 列 × 9 行
- 10 列 × 9 行
- 10 列 × 10 行
- 12 列 × 10 行

精灵图按照整数坐标切分：

```text
单帧宽度 = floor(图片宽度 / 列数)
单帧高度 = floor(图片高度 / 行数)
```

每行可以对应一个动作。动画控制器支持待机、行走、快速动作、特殊动作、
Ping-Pong 播放、可变帧时长、冷却和非无限循环动作。

## 事件系统

世界每 10 分钟执行一次 Tick。Tick 代表一次世界更新机会，并不意味着每次都要创建新的顶层事件。

- 普通日常事件占绝大多数
- 特殊事件和重大事件保持低概率
- 学习、工作、阅读、散步、购物等持续活动使用一个主事件和多个子进展
- 一个主事件最多包含 4 个子进展
- 单个主事件的预计持续时间最多 40 分钟
- 与最近事件高度相似的内容会被拒绝并重新生成
- 只有真实发生 NPC 互动时，NPC 才会加入参与者
- 所有效果由世界引擎校验并执行
- 地点切换由停留时间和当前世界状态控制
- 自动 Tick 使用唯一 Tick ID 防止重复生成
- 手动“AI 立即生成”可用于测试

事件结构：

```text
主事件
  ├── 子进展 1
  ├── 子进展 2
  ├── 子进展 3
  └── 子进展 4
```

## 世界数据

主要配置文件位于：

```text
world/
character/
config/
```

重要文件包括：

```text
world/rules.md
world/world.json
world/locations.json
world/event_probabilities.json
character/character.md
character/personality.md
character/relationships.md
character/important_people/
config/config.example.json
```

`world/event_probabilities.json` 保存可编辑的事件概率策略，包括事件类别、
连续事件时长分布和地点切换概率。

结构化运行状态保存在 SQLite 中。安装版本的数据目录由应用数据目录配置决定。

## 构建 Windows 安装包

执行：

```powershell
npm.cmd run build
npm.cmd run tauri build
```

生成文件位于：

```text
src-tauri/target/release/bundle/nsis/
src-tauri/target/release/bundle/msi/
```

推荐上传到 GitHub Release：

```text
The You Beyond_0.1.0_x64-setup.exe
```

NSIS `.exe` 安装包适合大多数用户。`.msi` 文件适合企业部署或受管控安装。

## 常见问题

### 找不到 Cargo

安装 Rust 后重新打开终端：

```powershell
cargo --version
rustc --version
```

### 打包时找不到图标

确认图标文件存在：

```text
src-tauri/icons/icon.ico
```

然后重新执行：

```powershell
npm.cmd run tauri build
```

### AI 事件生成失败

检查 Base URL、Model、API Key 和接口兼容性。接口必须支持：

```text
POST /chat/completions
```

可以在 Settings 中打开日志查看请求、响应、JSON 解析和事件校验错误。

## License

公开发布前请补充项目许可证。
