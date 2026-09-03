# The You Beyond

透明 AI 桌宠，拥有可自定义角色、精灵图动画、世界事件、连续记忆与动态成长。

A transparent AI desktop pet with customizable characters, sprite animations, world events, persistent memory, and dynamic growth.

## Features

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

## 功能简介

The You Beyond is a persistent desktop character world. The pet continues to
develop through time, events, relationships, items, skills, memories, and
long-term attribute growth.

The You Beyond 是一个持续运行的桌面角色世界。桌宠会随着时间产生事件、
建立关系、获得物品和技能，并逐步积累记忆与属性成长。

## Architecture

```text
Real time
   |
World Scheduler
   |
Backend Event Director
   |  decides type, continuity, location and duration
LLM Content Generator
   |  generates concrete event text and effects
Backend Validation
   |  validates schema, participants, effects, location and duplicates
World Engine
   |
SQLite + Markdown + React UI
```

The backend is the only authority that mutates world state. The AI generates
content inside the conditions provided by the backend.

后端负责决定事件结构、地点、连续性和是否合法；AI 只负责在给定条件下生成具体内容。

## Requirements

- Windows 10/11 64-bit
- Node.js 18 or newer
- npm
- Rust stable and Cargo
- WebView2
- Visual Studio C++ Build Tools

## Development

```powershell
npm.cmd install
npm.cmd run dev
```

Run the Tauri desktop application:

```powershell
npm.cmd run tauri dev
```

Run checks:

```powershell
npm.cmd run build
npm.cmd test
cargo test --manifest-path src-tauri/Cargo.toml
```

## AI Provider

Open Settings and configure:

- `Base URL`, for example `https://example.com/v1`
- `Model`
- `API Key`
- Output language

The provider uses an OpenAI-compatible `POST /chat/completions` endpoint.
Provider requests use a 120-second timeout. Common provider response formats
are normalized before event validation.

不要将 API Key 提交到 Git 仓库。发布软件或公开截图时，请隐藏 API Key。

## Sprite Sheets

从 Settings 导入透明 PNG 精灵图。

当前支持的网格配置包括：

- 8 列 × 9 行
- 10 列 × 9 行
- 10 列 × 10 行
- 12 列 × 10 行

精灵图按照整数坐标切分：

```text
frame width  = floor(image width / columns)
frame height = floor(image height / rows)
```

每行可以对应一个动作。动画控制器支持待机、行走、快速动作、特殊动作、
Ping-Pong 播放、可变帧时长、冷却和非无限循环动作。

## Event System

The world runs on a ten-minute Tick. A Tick is an opportunity to update the
world, not a requirement to create a new top-level event.

- Ordinary daily events are the majority.
- Special and major events remain rare.
- Long activities use one Event Thread with nested Progress updates.
- A thread can contain at most four Progress updates.
- A thread is limited to a maximum estimated duration of 40 minutes.
- Similar recent events are rejected and regenerated.
- Only actual NPC interaction may add an NPC to participants.
- Effects are validated and applied by the World Engine.
- Location changes are controlled by elapsed location time and world state.
- Duplicate automatic Tick requests are ignored by Tick ID.
- Manual `AI 立即生成` requests remain available for testing.

事件结构：

```text
Event Thread
  ├── Progress 1
  ├── Progress 2
  ├── Progress 3
  └── Progress 4
```

## World Data

主要世界配置文件位于：

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

## Build Windows Installers

构建前端和 Tauri 安装包：

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

## Troubleshooting

### Cargo is not found

安装 Rust 后重新打开终端：

```powershell
cargo --version
rustc --version
```

### Installer cannot find an icon

确认图标文件存在：

```text
src-tauri/icons/icon.ico
```

然后重新执行：

```powershell
npm.cmd run tauri build
```

### AI event generation fails

检查 Base URL、Model、API Key 和接口兼容性。接口必须支持：

```text
POST /chat/completions
```

可以在 Settings 中打开日志查看请求、响应、JSON 解析和事件校验错误。

## License

Add the project license here before public distribution.
