# AI World Desktop Pet 进度总结

## 已完成

- 已建立完整 Tauri v2 + React + TypeScript + Rust 工程骨架。
- 已实现透明、无边框、置顶的桌面窗口配置。
- 已实现前端桌宠主界面、Chronicle 侧栏、状态面板和 Settings 面板。
- 已实现角色基础状态：等级、XP、Mood、Energy、Traits、Skills、Inventory、Goals、NPC 列表、事件列表。
- 已实现本地离线事件模拟与事件流展示。
- 已实现 8 x 9 Sprite Atlas 的导入入口与尺寸校验逻辑。
- 已实现 JSON 世界导出与导入。
- 已实现 Reset World 二次确认。
- 已实现 Rust World Engine 基础持久化。
- 已实现 SQLite 表结构：world_state、events、personality_evidence、relationships、shared_experiences、inventory、skills、goals。
- 已实现事件验证基础规则：类型白名单、摘要长度、XP 和 Relationship delta clamp。
- 已实现中文优先的 LLM 输出协议。
- 已实现 OpenAI-compatible Provider 调用骨架。
- 已实现 `generate_event` / `test_provider` / `apply_proposal` / `reset_world` / `get_world` Tauri commands。
- 已实现后台调度器基础逻辑与重要事件概率测试。
- 已实现 Markdown 记忆文件与世界规则文件。
- 已补充英文 README 和中文 README。
- 已补充基础静态验证。

## 针对 require 还需要继续完成

- 需要让 Rust 后台调度器真正基于当前世界快照和 Markdown 记忆持续生成候选事件，而不是只做基础 tick 检查。
- 需要把 `generate_event` 的结果完整接入世界状态更新链路，包括：事件落库、关系变化、人格证据、共享经历、重要人物推进。
- 需要实现 Markdown 与 SQLite 的双向同步规则，包含启动时重载和事件后写回。
- 需要补齐更完整的世界快照结构，让前端从 Rust 读取真实状态，而不是主要依赖前端初始内存状态。
- 需要实现系统托盘菜单的更完整窗口控制细节，以及窗口位置/大小持久化。
- 需要实现真正的 OS 密钥存储接入，至少在桌面端落到 Keychain / Credential Manager 的正式实现。
- 需要实现更多事件类型的规则化处理：social、milestone、important、relationship、personality relevant 等。
- 需要实现更完整的 Import / Export：JSON world、Markdown character files、必要时打包导出。
- 需要实现更完整的测试覆盖：事件 JSON 解析、XP/Level、Relationship、Shared Experience、Personality Evolution、Markdown sync、sleep/wake recovery 等。
- 需要完成真实的 Tauri 构建验证与 Windows/macOS 构建验证。

## 当前阻塞

- 当前工作环境缺少 `node`、`npm`、`rustc`、`cargo`，因此无法实际运行安装、构建、测试和 Tauri 启动验证。
- 由于无法执行真实编译，仍需要在具备完整工具链的环境中做一次端到端构建检查。

## 下一步建议

1. 优先补齐世界快照和 Markdown 同步链路。
2. 然后把后台调度器接到真实事件生成和持久化。
3. 最后补全构建验证、跨平台打包和测试覆盖。
