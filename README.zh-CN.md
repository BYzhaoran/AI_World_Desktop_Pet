# Aoi's World 中文说明

这是一个长期运行的 AI 桌面角色世界。Aoi 会按照真实本地时间生活，逐渐积累事件、记忆、NPC 关系、共同经历、物品、技能、目标、XP、等级和人格证据。程序不是随机故事生成器，也不是聊天机器人。

## 技术架构

- Tauri v2：透明、无边框、置顶的 Windows / macOS 桌面窗口。
- Rust：World Engine、事件验证、SQLite 持久化、调度器和 LLM Provider。
- React + TypeScript + Vite：桌宠渲染、状态面板、Chronicle 和 Settings。
- SQLite：结构化状态的唯一事实来源。
- Markdown：用户可以直接编辑的人物长期记忆。

核心链路：

```text
真实时间 -> Scheduler -> Event Candidate -> LLM Event Director
-> JSON Parse -> Schema / Rule / Cooldown Validation
-> World Engine -> SQLite + Markdown -> React Renderer
```

LLM 只能观察上下文并提出 `EventProposal`，不能直接访问数据库或覆盖人物文件。

## 安装与开发

需要 Node.js 18+、npm、Rust stable。macOS 需要 Xcode Command Line Tools，Windows 需要 WebView2 和 Visual Studio C++ Build Tools。

```sh
npm install
npm run dev
npm run build
npm run test
npm run tauri dev
```

发布构建：

```sh
npm run tauri build
cargo test --manifest-path src-tauri/Cargo.toml
```

构建产物位于 `src-tauri/target/release/bundle/`。

## 中文优先的模型输出

Settings 中可填写：

- Base URL，例如 `https://example.com/v1`
- Model，例如 `qwen3-max`
- API Key
- Output language：`中文优先` 或 `English`

默认语言是中文。Rust Provider 会向模型发送明确约束：

- 只返回 JSON
- 默认使用简体中文
- 普通事件的 `summary` 保持简短
- 重要事件必须有长期意义
- LLM 不得直接修改持久化状态

如果选择 English，模型可以返回英文事件；如果没有选择 English，中文是优先语言。模型无论返回中文还是英文，都必须符合相同的结构化 EventProposal schema。

请求接口：`POST /chat/completions`。

错误包括 timeout、HTTP 非 2xx、invalid JSON、empty choices 和 schema mismatch。错误不会导致应用崩溃，程序会切换到离线事件。

## API Key 安全

API Key 不会写入 Git，也不会写入浏览器 localStorage，不会出现在日志中。桌面正式版应该将 Key 接入 macOS Keychain 或 Windows Credential Manager。当前浏览器预览只在当前页面内存中使用 Key。

## 世界循环

普通事件检查间隔为 20-90 分钟。重要事件每 4-8 小时进入概率候选窗口，不采用每 12 小时机械触发。概率会参考：

- 当天已发生的重要事件数量
- 最近重要事件时间
- 目标压力
- NPC 关系机会
- 最近普通事件数量
- 角色状态和随机因素

离线模式允许普通日常继续运行，但不会产生重大的关系、人格或人物升级变化。

## 人物 Markdown 文件

可编辑文件：

```text
character/character.md
character/personality.md
character/relationships.md
character/important_people/*.md
world/rules.md
```

应用启动和生成事件前会重新读取这些文件。普通事件不会更新人格；只有重要事件或人格相关事件才允许产生带事件 ID 的 Personality Evidence。人格变化会被限制在小幅 delta 内。

## SQLite 数据

数据库默认位于 `data/world.sqlite3`，也可以通过 `AI_WORLD_DATA` 指定数据目录。当前迁移包含：

- `world_state`
- `events`
- `personality_evidence`
- `relationships`
- `shared_experiences`
- `inventory`
- `skills`
- `goals`

原始事件永久保留。Markdown 是人类可读记忆，SQLite 是结构化状态。

## 导入、导出与桌面托盘

Settings 支持 JSON 世界快照导入导出、事件数量、动画 FPS、实时模式和重置世界。重置必须二次确认。Tauri 桌面端包含系统托盘，可显示窗口或退出程序。

## Sprite Sheet

精灵系统支持透明 PNG atlas。默认是 8 列 x 9 行，但不会写死尺寸。程序会根据实际图片宽高和配置计算 frame width / height，并校验是否整除。动画控制器支持 idle、walking、thinking、happy、sleepy、important_event、social、celebrating 等状态，以及 FPS、loop 和 ping-pong。

## 故障排查

- API 不可用：检查 Base URL 是否包含正确的 OpenAI-compatible 路径，程序会自动使用离线事件。
- 桌面窗口打不开：先运行 `npm run build`，再检查 Rust、WebView2 或 Xcode Command Line Tools。
- 数据丢失：确认 `AI_WORLD_DATA` 没有被切换到新的目录，并检查 `data/world.sqlite3`。
- 中文输出异常：Settings 选择 `中文优先`，并确认模型支持中文和 JSON response format。
