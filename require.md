你现在是一名资深的：

    Desktop Application Engineer
    Tauri/Rust Engineer
    React/TypeScript Engineer
    LLM Application Engineer
    World Simulation Engineer
    Game AI Engineer
    Character AI Engineer

请直接在当前目录从零建立一个完整、可运行、可维护的：

    AI World Desktop Pet

项目。

不要只输出方案、架构图或者代码片段。

你的任务是：

    实际创建完整工程
    安装依赖
    编写代码
    运行测试
    修复错误
    启动应用
    验证核心功能

最终得到一个可以在：

    Windows
    macOS

上运行的 AI 桌宠。

============================================================
一、项目最终目标
============================================================

我要制作的不是普通 Desktop Pet。

也不是：

    点击桌宠 -> AI 回复一句话

更不是：

    每隔几分钟随机调用 LLM -> 生成一句小说

而是一个：

    Persistent AI Character World

也就是：

    一个长期生活在用户桌面上的 AI Character。

这个角色拥有：

    人物设定
    性格
    真实时间
    世界
    地点
    NPC
    关系
    事件
    记忆
    物品
    技能
    XP
    Level
    Goals
    Mood
    Energy

角色会随着现实时间自然生活。

即使用户不操作：

    角色也会自己生活
    自己经历事件
    与 NPC 产生关系
    获得物品
    学会技能
    完成目标
    发生重要事件
    逐渐形成更加完整的性格

最终体验应该是：

    Desktop Pet
        +
    AI Character
        +
    Persistent World
        +
    Memory
        +
    Event Simulation
        +
    Character Growth
        +
    Personality Evolution
        +
    Chronicle

============================================================
二、必须参考的开源项目
============================================================

在开始正式编码之前：

必须主动搜索、阅读并分析以下 GitHub 项目。

不要直接复制代码。

需要理解它们的设计，然后选择适合本项目的架构。

------------------------------------------------------------
1. xiangking/agent-pet
------------------------------------------------------------

GitHub：

https://github.com/xiangking/agent-pet

重点参考：

    Tauri v2
    Rust
    React
    Vite
    transparent desktop window
    always-on-top
    sprite atlas
    8×9 sprite sheet
    desktop pet
    settings
    system tray
    Windows
    macOS

该项目明确支持：

    Windows
    macOS

并且使用：

    1536 × 1872
    8 × 9
    192 × 208

的 Codex-compatible sprite atlas。

主要参考：

    Desktop Pet Renderer
    Sprite System
    Window System
    Cross-platform packaging

不要依赖其 AI Agent integration。

本项目不需要 OpenCode / Codex / Claude Code 等 Agent 集成。

:contentReference[oaicite:0]{index=0}

------------------------------------------------------------
2. asklar/agent-terrarium
------------------------------------------------------------

GitHub：

https://github.com/asklar/agent-terrarium

重点参考：

    Tauri v2
    Rust Simulation
    React Renderer
    World State
    Agent
    Awareness
    Event System
    AI Backend
    OpenAI-compatible API
    autonomous behavior

特别参考：

    Simulation Engine
    Event Dispatcher
    Awareness System
    Agent State

该项目的设计中：

    Simulation 在 Rust 后端运行
    Frontend 主要负责 Renderer

这个思想非常适合本项目。

本项目也应该：

    Rust = World Simulation

    React = UI / Renderer

而不是把所有世界逻辑塞进 React。

:contentReference[oaicite:1]{index=1}

------------------------------------------------------------
3. gukosowa/agents-in-the-office
------------------------------------------------------------

GitHub：

https://github.com/gukosowa/agents-in-the-office

重点参考：

    Event-driven architecture
    NPC
    Character
    Event persistence
    File watcher
    Activity/Event model
    Character state

尤其参考：

    Event -> Character State
    Event -> Animation
    Event -> Persistent History

但是：

本项目不需要接入 Claude Code / Gemini CLI。

只借鉴其：

    Event Architecture

:contentReference[oaicite:2]{index=2}

------------------------------------------------------------
4. nemoaigc/hermes-quest
------------------------------------------------------------

GitHub：

https://github.com/nemoaigc/hermes-quest

重点参考：

    Character
    XP
    Level
    Skills
    Inventory
    NPC
    Relationship
    Quest
    Chronicle
    persistent state

尤其参考：

    Character Progression
    Adventure Chronicle
    NPC
    Inventory
    XP

:contentReference[oaicite:3]{index=3}

============================================================
三、平台要求
============================================================

必须同时支持：

    Windows 10 / 11
    macOS

架构：

    macOS Apple Silicon:
        arm64

    macOS Intel:
        x86_64

    Windows:
        x86_64

如果架构允许：

未来预留：

    Windows ARM64

但第一版：

    Windows x64
    macOS arm64
    macOS x64

必须能够构建。

推荐：

    Tauri v2
    Rust
    React
    TypeScript
    Vite

不要使用 Electron。

除非遇到无法通过 Tauri 合理解决的核心功能，否则不得切换 Electron。

============================================================
四、项目核心架构
============================================================

最终架构：

                         REAL WORLD TIME
                                │
                                ▼
                       ┌─────────────────┐
                       │  World Engine   │
                       └────────┬────────┘
                                │
              ┌─────────────────┼─────────────────┐
              ▼                 ▼                 ▼
        Character            World               NPC
              │                 │                 │
              └─────────────────┼─────────────────┘
                                ▼
                       Event Scheduler
                                │
                                ▼
                       Candidate Events
                                │
                                ▼
                       LLM Event Director
                                │
                                ▼
                         Event Proposal
                                │
                                ▼
                        JSON Validation
                                │
                                ▼
                         Rule Validation
                                │
                                ▼
                          World Engine
                                │
              ┌─────────────────┼─────────────────┐
              ▼                 ▼                 ▼
           SQLite            Memory          Personality
                                │                 │
                                └────────┬────────┘
                                         ▼
                                  Character Update
                                         │
                                         ▼
                                      React
                                   ┌─────┴─────┐
                                   ▼           ▼
                                 Pet        Sidebar


核心原则：

    LLM = Event Director

    World Engine = Reality / Rules

    SQLite = Persistent Memory

    Personality System = Character Evolution

    Scheduler = Time

    Sprite System = Body

    React = Renderer / UI

============================================================
五、最重要的设计原则
============================================================

必须严格遵守：

    LLM 不能直接修改数据库。

LLM 只能：

    observe
    reason
    propose

World Engine 才能：

    validate
    mutate
    persist

也就是说：

    LLM
      ↓
    Event Proposal
      ↓
    Validation
      ↓
    World Engine
      ↓
    SQLite

而不是：

    LLM
      ↓
    SQLite

============================================================
六、8×9 Sprite Sheet
============================================================

第一版最基本功能：

    用户可以导入自己的 8×9 Sprite Sheet。

例如：

    1536 × 1872

    columns = 8
    rows = 9

因此：

    frame width = 192
    frame height = 208

但：

绝对不要把尺寸写死。

程序必须：

    自动读取图片尺寸
    根据 rows / columns 计算 frame size
    支持 PNG
    支持透明背景
    验证图片尺寸

默认：

    rows = 9
    columns = 8

但是必须允许修改。

============================================================
七、Sprite Animation
============================================================

建立独立：

    Sprite Animation System

支持：

    idle
    walking
    thinking
    happy
    sad
    sleepy
    surprised
    working
    important_event
    social
    celebrating

动画配置必须可编辑。

例如：

{
    "sprite": {
        "path": "assets/pets/default/sprite.png",
        "columns": 8,
        "rows": 9
    },

    "animations": {
        "idle": [0,1,2,3,4,5],
        "walking": [8,9,10,11,12,13],
        "thinking": [16,17,18,19],
        "happy": [24,25,26,27],
        "sleepy": [32,33,34,35]
    }
}

支持：

    fps
    loop
    one-shot
    ping-pong

事件发生时：

    Event
      ↓
    Character State
      ↓
    Animation State

例如：

    important_event
        →
    important_event animation

============================================================
八、Desktop Window
============================================================

桌宠窗口必须：

    transparent
    borderless
    always-on-top

支持：

    拖动
    缩放
    多显示器
    保存窗口位置
    保存窗口大小
    系统托盘
    显示/隐藏 Sidebar
    Settings
    Exit

Windows 和 macOS 行为必须尽可能一致。

桌宠默认：

    只显示角色。

Sidebar：

    显示在角色旁边。

============================================================
九、Sidebar
============================================================

桌宠旁边拥有：

    Chronicle Sidebar

例如：

    ┌────────────────────────────┐
    │                            │
    │          PET               │
    │                            │
    │          👧                │
    │                            │
    └────────────────────────────┘
                 │
                 ▼
    ┌──────────────────────────────┐
    │ Chronicle                    │
    ├──────────────────────────────┤
    │ 18:42                        │
    │ 她去了图书馆，在窗边坐了一会。 │
    │                              │
    │ 16:20                        │
    │ 她在回家的路上遇到了 Aoi。     │
    │                              │
    │ ⭐ Important Event            │
    │ 她完成了本周的阅读目标。        │
    └──────────────────────────────┘

默认只显示：

    最近 10 条

允许 Settings 修改：

    5
    10
    20
    30

但：

    Sidebar 显示数量 != 历史数据数量。

历史必须永久保存。

============================================================
十、Event
============================================================

事件分为：

    normal_event
    social_event
    activity_event
    weather_event
    discovery_event
    item_event
    skill_event
    relationship_event
    important_event
    milestone_event
    level_up
    no_event

普通事件：

    1～2句话。

目标：

    20～80 中文字符。

不要生成：

    长篇小说
    大段剧情
    500 字故事

============================================================
十一、重要事件频率
============================================================

这是新的重要要求。

平均：

    每天约 2 次重要事件。

也就是说：

    ~2 important events / real day

但是：

绝对不能写成：

    每 12 小时机械触发一次。

必须使用：

    probability
    cooldown
    context
    goals
    relationships
    elapsed time
    recent events
    character development

共同决定。

建议：

    每 4～8 小时进入一次重要事件候选窗口。

但是：

    不一定触发。

系统根据：

    今日已经发生的重要事件数量
    最近一次重要事件时间
    角色目标
    NPC关系
    当前剧情
    普通事件数量
    当前时间
    世界状态

计算：

    important_event_probability

目标统计：

    约 2 次 / 天

允许：

    某天 1 次

或者：

    某天 3 次

但是长期平均：

    ~2/day

不要追求机械精确。

============================================================
十二、重要事件必须“有意义”
============================================================

重要事件不能只是：

    “她今天吃了一个蛋糕。”

这种事件不应该成为重要事件。

重要事件应该导致：

    Character Growth
    Relationship Change
    New Person
    New Item
    New Skill
    Goal Progress
    Location Discovery
    Personality Development
    Long-term Memory
    Milestone

例如：

    她第一次认识 Aoi。

这是重要事件。

然后：

    Aoi 成为熟悉的人。

然后：

    两人一起完成一件事情。

然后：

    两人的关系提升。

然后：

    Aoi 送给她一本书。

这些事件形成：

    Causal Chain

============================================================
十三、人物设定文件
============================================================

这是本项目的核心功能之一。

必须建立：

    character/character.md

或者：

    world/character.md

推荐：

    character/character.md

这个文件保存角色的长期人物设定。

例如：

# Character

## Basic

Name:
Aoi

Age:
18

Role:
Student

## Personality

- quiet
- curious
- kind
- slightly shy

## Likes

- books
- rain
- cats

## Dislikes

- crowded places

## Habits

- reads before sleeping
- tends to observe before speaking

## Values

- friendship
- curiosity
- learning

## Goals

- become better at drawing
- make close friends

============================================================
十四、人物设定不能完全固定
============================================================

非常重要：

    character.md 不是完全静态的 Prompt。

它应该是：

    Initial Personality
        +
    Life Experiences
        +
    Important Events
        +
    Relationships
        +
    Reflection
        ↓
    Evolving Personality

也就是说：

角色的人格会逐渐形成。

例如初始：

    shy
    cautious
    curious

发生重要事件：

    她第一次主动帮助别人。

之后：

    更愿意主动帮助别人。

再发生：

    她因为帮助别人获得了积极反馈。

之后：

    confidence 增加。

最终：

    shy
    ↓
    shy but increasingly confident

============================================================
十五、人物设定文件的内容
============================================================

建立：

    character/character.md

至少包含：

    Basic Information
    Appearance
    Personality
    Likes
    Dislikes
    Habits
    Values
    Goals
    Fears
    Strengths
    Weaknesses
    Relationships
    Important Memories
    Personality Development

例如：

# Personality Development

## Current Traits

- quiet
- curious
- kind
- increasingly confident

## Developed Traits

- became more willing to initiate conversations
- became more comfortable around close friends

## Personality Evidence

- 2026-09-02:
  Helped Aoi with a difficult problem.

- 2026-09-03:
  Voluntarily invited Aoi to study together.

============================================================
十六、人物设定文件更新机制
============================================================

不是每一个事件都更新 character.md。

只有：

    Important Event

或者：

    Personality-Relevant Event

才允许更新。

例如：

普通：

    “她吃了午饭。”

不更新。

重要：

    “她第一次主动向陌生人搭话。”

可以更新。

重要：

    “她因为一次失败变得更加谨慎。”

可以更新。

重要：

    “她和 Aoi 一起完成了一件困难的事情。”

可以更新。

============================================================
十七、根据共同经历补全性格
============================================================

这是整个项目最重要的高级机制之一。

角色性格必须根据：

    Shared Experiences

逐渐补全。

例如：

角色：

    Aoi

第一次：

    在图书馆遇到。

第二次：

    一起学习。

第三次：

    一起躲雨。

第四次：

    Aoi 在角色困难时帮助她。

这些共同经历形成：

    Shared Memory

系统可以逐渐推断：

    Aoi is becoming an important friend.

角色性格：

    more trusting
    more socially comfortable
    more willing to share thoughts

============================================================
十八、Shared Experience System
============================================================

数据库增加：

    shared_experiences

例如：

{
    "id": "shared_001",
    "characters": [
        "main_character",
        "aoi"
    ],
    "event_ids": [
        "event_120",
        "event_134",
        "event_141"
    ],
    "summary":
        "Aoi and the character studied together several times.",
    "relationship_impact": 12
}

只有多个相关事件积累之后：

    才形成长期关系变化。

不要：

    第一次见面
    relationship +50

应该：

    多次共同经历
        ↓
    Relationship gradually changes

============================================================
十九、Personality Reflection
============================================================

建立：

    prompts/personality_reflection.md

定期触发。

例如：

    每 3～7 天

或者：

    积累 3～5 个重要事件

进行一次：

    Personality Reflection

输入：

    character.md
    recent important events
    shared experiences
    NPC relationships
    goals
    previous personality development

LLM 输出：

    proposed personality changes

例如：

{
    "changes": [
        {
            "trait": "confidence",
            "change": "+1",
            "reason":
                "She repeatedly initiated conversations with Aoi."
        }
    ]
}

但是：

LLM 不能直接覆盖 character.md。

必须：

    LLM Proposal
        ↓
    Personality Validator
        ↓
    World Engine
        ↓
    Update character.md

============================================================
二十、性格不能瞬间改变
============================================================

必须防止：

    一个事件
        →
    Personality 完全改变

例如：

原本：

    shy = 8/10

一次事件不能变成：

    shy = 2/10

应该：

    shy = 8
        ↓
    7.5

或者：

    confidence 3
        ↓
    confidence 4

逐渐变化。

建议使用：

    trait score

例如：

    confidence: 35
    sociability: 42
    curiosity: 78
    kindness: 82

范围：

    0～100

每次重要事件：

    delta -5 ~ +5

特殊重大事件：

    delta -10 ~ +10

但必须经过 World Engine 限制。

============================================================
二十一、Character Development
============================================================

人物成长应该同时来自：

    XP
    Skills
    Relationships
    Memories
    Personality

因此：

    Level

只是角色成长的一部分。

不能把：

    Level 10

理解为：

    人格成熟。

============================================================
二十二、NPC
============================================================

至少：

    3～5 个 NPC

NPC 必须拥有：

    id
    name
    personality
    goals
    location
    memories
    relationship

例如：

{
    "id": "aoi",
    "name": "Aoi",
    "personality": [
        "quiet",
        "kind",
        "observant"
    ],
    "goals": [
        "improve drawing"
    ]
}

============================================================
二十三、重要人物系统
============================================================

并不是所有 NPC 都必须成为重要人物。

建立：

    Important Character

机制。

当主角：

    多次遇到某 NPC

或者：

    发生重要共同事件

或者：

    Relationship 超过阈值

该 NPC 可以升级为：

    Important Character

例如：

    relationship >= 30

或者：

    shared experiences >= 2

则：

    进入重要人物候选。

如果确认：

    写入人物设定文件。

例如：

# Important People

## Aoi

Role:
Close friend

Personality:
Quiet, kind, observant

Relationship:
Friend

Shared Experiences:
- Studied together
- Walked home together in the rain
- Helped each other with assignments

First Met:
2026-09-02

Important Memories:
...

============================================================
二十四、人物文件分层
============================================================

推荐：

character/
├── character.md
├── personality.md
├── relationships.md
└── important_people/
    ├── aoi.md
    ├── yuki.md
    └── ...

其中：

    character.md

保存：

    核心人物设定。

    personality.md

保存：

    当前性格
    性格变化
    性格证据。

    relationships.md

保存：

    关系摘要。

    important_people/*.md

保存：

    重要人物。

============================================================
二十五、文件和数据库的关系
============================================================

SQLite：

    Source of Truth

Markdown：

    Human-readable Memory / Character Profile

也就是说：

    SQLite
        =
    完整结构化状态

    Markdown
        =
    AI 可读 + 用户可编辑的人物记忆

必须保证：

    两者可以同步。

============================================================
二十六、人物文件必须可以人工修改
============================================================

用户可以直接修改：

    character/character.md
    character/personality.md
    character/important_people/*.md

程序下一次生成事件时：

    必须重新读取这些文件。

用户可以通过修改文件：

    改变角色性格
    增加设定
    增加人物
    删除设定

例如用户写：

    “她非常害怕狗。”

之后：

    LLM 生成事件时必须考虑这个设定。

============================================================
二十七、World Rules
============================================================

建立：

    world/rules.md

例如：

# World Rules

## Time

World time follows real local time.

## Character

The character must behave consistently with her personality.

Personality can gradually evolve through meaningful experiences.

## Memory

Important events must be remembered.

Repeated shared experiences can strengthen relationships.

## Important People

People who repeatedly interact with the character may become important.

## Events

Normal events should be short.

Important events should be meaningful.

Average important event frequency:

    approximately 2 per real day

This is an average target, not a strict timer.

## Growth

XP and level are controlled by World Engine.

## Personality

Personality changes must be gradual.

A single ordinary event cannot completely change personality.

## Consistency

Persistent world state has higher priority than LLM output.

## LLM

LLM proposes events.

LLM cannot directly mutate persistent state.

============================================================
二十八、真实时间
============================================================

World Time：

    默认完全同步真实时间。

例如：

真实：

    18:42

World：

    18:42

不要默认时间加速。

支持：

    real_time_mode

未来可以扩展：

    accelerated_time

但第一版：

    1 real minute
        =
    1 world minute

============================================================
二十九、角色日程
============================================================

角色应该有自然生活节奏。

例如：

    07:00
        wake

    08:00
        school

    12:00
        lunch

    13:00
        study

    16:30
        free

    18:00
        home

    22:30
        sleep

但是：

不能完全写死。

实际行为由：

    schedule
    personality
    weather
    location
    energy
    goals
    recent events
    world rules

共同决定。

============================================================
三十、Event Scheduler
============================================================

不要每分钟调用 LLM。

使用：

    background scheduler

正常事件：

    每 20～90 分钟检查。

重要事件：

    每 4～8 小时进入候选窗口。

目标：

    平均约 2 次重要事件 / 天。

但是：

    不是强制每 12 小时一次。

必须允许：

    no_event

============================================================
三十一、Event Candidate System
============================================================

首先由 World Engine 生成候选：

    study
    rest
    food
    social
    travel
    weather
    discovery
    item
    NPC
    relationship
    goal
    milestone

根据：

    current_time
    location
    weather
    personality
    energy
    relationships
    goals
    cooldown
    previous events

计算候选分数。

然后交给 LLM。

============================================================
三十二、LLM Event Director
============================================================

建立：

    prompts/event_director.md

LLM 需要看到：

    World Rules
    Character Profile
    Personality
    Current World
    Current Time
    Weather
    Recent Events
    Important Memories
    Important People
    NPC Relationships
    Goals
    Shared Experiences
    Event Candidates

LLM 的角色：

    Event Director

不是：

    God

============================================================
三十三、LLM 输出格式
============================================================

必须使用结构化 JSON。

普通事件：

{
    "type": "normal_event",

    "summary":
        "她去了图书馆，在窗边坐了一会儿。",

    "importance": 0.2,

    "location":
        "library",

    "effects": {
        "energy": -3,
        "happiness": 2,
        "xp": 5
    }
}

重要事件：

{
    "type": "important_event",

    "summary":
        "她第一次主动邀请 Aoi 一起学习。",

    "importance": 0.9,

    "participants": [
        "main_character",
        "aoi"
    ],

    "effects": {
        "xp": 40,
        "relationship": {
            "target": "aoi",
            "delta": 5
        }
    },

    "memory": {
        "important": true
    },

    "personality_signal": {
        "trait": "confidence",
        "delta": 3,
        "reason":
            "She initiated the interaction herself."
    }
}

没有事件：

{
    "type": "no_event"
}

============================================================
三十四、Event Validation
============================================================

LLM 输出必须经过：

    JSON Parse
    Schema Validation
    World Rule Validation
    State Validation
    Cooldown Validation

例如：

LLM：

    relationship_delta = 500

必须：

    clamp / reject

LLM：

    level = 999

必须：

    reject

LLM：

    create legendary sword

如果：

    world rules 不允许

必须：

    reject

============================================================
三十五、World Engine
============================================================

World Engine 负责：

    load state
    update time
    schedule events
    validate events
    apply events
    update character
    update personality
    update NPC
    update relationships
    update inventory
    update skills
    update XP
    calculate level
    create memories
    update character files
    persist state

============================================================
三十六、XP
============================================================

例如：

    Level 1:
        100 XP

    Level 2:
        250 XP

    Level 3:
        450 XP

XP 规则：

    World Engine 控制。

LLM 只能建议：

    xp_delta

不能直接：

    level = 10

============================================================
三十七、Inventory
============================================================

支持：

    item

包含：

    id
    name
    description
    quantity
    rarity

例如：

    umbrella
    notebook
    book
    flower
    gift

重要物品：

    进入长期记忆。

============================================================
三十八、Skills
============================================================

支持：

    reading
    cooking
    drawing
    focus
    music

技能：

    id
    name
    level
    experience

Skill Level：

    World Engine 控制。

============================================================
三十九、Goals
============================================================

支持长期目标：

    Improve Focus
    Make a Friend
    Read 5 Books
    Learn Drawing

Goal：

    id
    description
    progress
    target
    completed

重要事件：

    可以推进 Goal。

Goal 完成：

    可以触发重要事件。

============================================================
四十、Relationship
============================================================

关系：

    -100 ~ 100

阶段：

    stranger
    acquaintance
    friend
    close_friend

关系变化：

    gradual

不能：

    第一次见面
    +50

应该：

    多次共同经历
        ↓
    relationship gradually changes

============================================================
四十一、Shared Experience
============================================================

实现：

    Shared Experience Graph

核心概念：

    Character A
        ↕
    Event
        ↕
    Character B

多个事件：

    Event A
    Event B
    Event C

如果都包含：

    A
    B

则形成：

    Shared Experience

例如：

    Aoi + Character

共同：

    认识
    学习
    避雨
    帮助
    聊天

系统总结：

    “Aoi and the character have gradually become close through repeated shared experiences.”

这条总结：

    写入长期记忆。

============================================================
四十二、Personality Evolution
============================================================

角色性格必须由：

    Initial Personality
        +
    Important Events
        +
    Shared Experiences
        +
    Relationship Development
        +
    Goals
        +
    Failures
        +
    Successes

共同形成。

例如：

初始：

    confidence = 30

经历：

    第一次主动帮助别人

变化：

    confidence = 33

经历：

    第二次主动与 Aoi 交流

变化：

    confidence = 36

经历：

    成功完成困难目标

变化：

    confidence = 40

最后：

    Personality：

    shy but increasingly confident

============================================================
四十三、Personality Reflection
============================================================

建立：

    prompts/personality_reflection.md

触发：

    每 3～7 天

或者：

    累积 3～5 个重要事件

输入：

    character.md
    personality.md
    important events
    shared experiences
    relationships
    goals

LLM 输出：

{
    "changes": [
        {
            "trait": "confidence",
            "old_value": 35,
            "delta": 3,
            "reason":
                "Repeatedly initiated interactions with Aoi."
        }
    ]
}

World Engine 验证后：

    更新 personality.md

============================================================
四十四、重要人物文件
============================================================

当 NPC 满足：

    relationship threshold

或者：

    shared experience threshold

或者：

    important event participant

则可以升级为：

    Important Person

创建：

    character/important_people/aoi.md

内容：

# Aoi

## Basic

Name:
Aoi

## Personality

quiet
kind
observant

## Relationship

friend

## First Met

2026-09-02

## Shared Experiences

- Studied together.
- Walked home together in the rain.
- Helped each other with assignments.

## Important Memories

...

## Relationship Development

...

============================================================
四十五、重要人物不是一次生成完
============================================================

重要人物文件应该持续完善。

第一次：

    Aoi

只知道：

    quiet
    kind

之后：

    共同经历增加

系统发现：

    Aoi likes drawing.

再加入：

    likes drawing

之后：

    Aoi is uncomfortable in crowds.

再加入：

    dislikes crowded places

因此：

    Important Person Profile

也是逐渐生成的。

============================================================
四十六、人物性格“补全”机制
============================================================

这里的“补全”不是：

    LLM 随便脑补。

必须有：

    Evidence

每个新性格特征必须尽可能对应事件证据。

例如：

    Trait:
        caring

Evidence:

    event_120:
        Helped Aoi when she was struggling.

    event_134:
        Brought an umbrella for Aoi.

因此：

    caring +2

而不是：

    LLM 觉得她很温柔
        →
    直接加入温柔。

============================================================
四十七、Personality Evidence
============================================================

数据库保存：

    personality_evidence

例如：

{
    "character_id": "main",
    "trait": "confidence",
    "delta": 3,
    "event_id": "event_120",
    "reason":
        "Initiated a conversation without being prompted."
}

这样未来：

    Personality Reflection

可以追溯：

    为什么性格发生变化。

============================================================
四十八、Memory 分层
============================================================

不要把所有历史一次性发送给 LLM。

分成：

    Recent Events
    Important Memories
    Shared Experiences
    Important People
    Personality
    Goals

例如：

Recent：

    最近 10 条。

Important：

    最近 20～50 条。

Long-term：

    character.md

NPC：

    每个人自己的摘要。

============================================================
四十九、SQLite
============================================================

SQLite 是：

    Source of Truth

至少包含：

    world_state
    characters
    personality_traits
    personality_evidence
    events
    memories
    npcs
    relationships
    shared_experiences
    inventory
    skills
    goals
    important_people

============================================================
五十、Markdown Memory
============================================================

Markdown 是：

    Human-readable
    AI-readable
    User-editable

包括：

    world/rules.md

    character/character.md

    character/personality.md

    character/relationships.md

    character/important_people/*.md

这些文件必须在事件生成前读取。

============================================================
五十一、文件同步
============================================================

如果：

    World Engine

修改了人物：

必须同步：

    SQLite

和：

    Markdown

如果用户手动修改：

    Markdown

下一次：

    Event Director

必须读取最新内容。

如果发现：

    Markdown
    SQLite

不一致：

以明确规则解决。

推荐：

    用户可编辑 Markdown
        →
    启动时 / 定期 reload
        →
    更新结构化 state

但是：

    不允许静默覆盖用户手动修改。

============================================================
五十二、API
============================================================

本项目：

    不绑定 OpenAI
    不绑定 Claude
    不绑定 Gemini
    不绑定 OpenCode
    不绑定 Codex

用户自己填写：

    Base URL
    API Key
    Model

目标：

    OpenAI-compatible API

默认：

    POST /chat/completions

例如：

    Base URL:
    https://example.com/v1

    Model:
    qwen3-max

    API Key:
    sk-xxxx

============================================================
五十三、API Settings
============================================================

Settings：

    ┌─────────────────────────────┐
    │ AI Provider                 │
    │                             │
    │ Base URL                    │
    │ [https://example.com/v1   ] │
    │                             │
    │ Model                       │
    │ [qwen3-max                ] │
    │                             │
    │ API Key                     │
    │ [••••••••••••••••         ] │
    │                             │
    │ [ Test Connection ]         │
    │                             │
    │ [ Save ]                    │
    └─────────────────────────────┘

支持：

    Test Connection

错误：

    timeout
    401
    403
    429
    500
    invalid JSON

都不能导致程序崩溃。

============================================================
五十四、API Key 安全
============================================================

绝对禁止：

    hardcode API key

绝对禁止：

    commit API key

绝对禁止：

    log API key

Windows：

    优先 Windows Credential Manager

macOS：

    优先 Keychain

Tauri 可以使用：

    OS credential storage

如果暂时实现不了：

至少：

    API key 不进入 Git
    config 中只保存 encrypted/secure reference
    日志中完全隐藏

============================================================
五十五、LLM Provider 抽象
============================================================

不要把：

    OpenAI-compatible API

写死在 World Engine。

建立：

    LLMProvider trait/interface

例如：

    trait LlmProvider

包含：

    generate()
    test_connection()

以后可以扩展：

    OpenAICompatible
    LocalModel
    OtherProvider

第一版只需要：

    OpenAICompatible

============================================================
五十六、Event Scheduler
============================================================

Scheduler 后台运行。

正常事件：

    20～90 min

重要事件：

    4～8 h candidate window

目标：

    ~2 important events/day

Scheduler 必须考虑：

    application startup
    system sleep
    computer wake
    timezone change

例如：

电脑睡眠 8 小时：

不要在恢复时：

    一次性生成 8 个事件。

应该：

    根据时间差决定是否生成少量回顾性事件。

============================================================
五十七、离线模式
============================================================

如果 API 不可用：

    World 不能停止。

可以使用：

    deterministic fallback events

例如：

    “她安静地休息了一会儿。”

    “她在房间里整理了一下东西。”

    “她看了一会儿书。”

但：

    离线模式不能产生重大人物关系变化。

重大成长事件：

    API 恢复后再进行。

============================================================
五十八、Event Cooldown
============================================================

防止：

    重复事件

例如：

    去便利店

发生后：

    6h cooldown

类似事件：

    降低概率。

但是：

如果存在：

    causal reason

可以突破 cooldown。

例如：

    第一次去便利店

后来：

    “因为 Aoi 让她帮忙买东西”

这就是合理的重复。

============================================================
五十九、事件因果链
============================================================

必须支持：

    Event A
        ↓
    Event B
        ↓
    Event C

例如：

    A：
    第一次认识 Aoi

    B：
    再次遇到 Aoi

    C：
    一起学习

    D：
    关系成为 Friend

    E：
    Aoi 送给她一本书

    F：
    她开始喜欢阅读

最终：

    F

不能与：

    A～E

完全无关。

============================================================
六十、重要事件示例
============================================================

示例：

09:10

    她第一次遇到了 Aoi。

这是：

    important_event

产生：

    new_person

之后：

17:30

    她在图书馆又遇到了 Aoi。

这是：

    social_event

之后：

第二天：

    她主动邀请 Aoi 一起学习。

这是：

    important_event

产生：

    relationship +5

之后：

    shared_experience

之后：

    Aoi 成为 important person。

之后：

    character/personality.md

加入：

    “She is becoming more comfortable initiating conversations.”

============================================================
六十一、Character Status
============================================================

点击桌宠：

显示：

    Name

    Level
    XP

    Mood
    Energy

    Personality

    Skills

    Inventory

    Relationships

例如：

    Aoi

    Lv.3
    XP 240 / 450

    Mood 72
    Energy 61

    Personality:
        Curious
        Kind
        Increasingly confident

    Friend:
        Aoi ❤️ 42

============================================================
六十二、Important Event UI
============================================================

重要事件必须明显区别于普通事件。

例如：

    ⭐ IMPORTANT EVENT

并触发：

    important animation

    sidebar highlight

    subtle particle effect

但是：

    不要弹出非常打扰用户的窗口。

============================================================
六十三、Pet Autonomous Behavior
============================================================

即使没有事件：

角色也应该有轻微自主行为：

    idle
    walking
    looking around
    sitting
    sleeping
    thinking

事件：

    reading
        →
    reading animation

    happy
        →
    happy animation

    important event
        →
    celebration animation

============================================================
六十四、低资源设计
============================================================

桌宠可能：

    一天运行十几个小时。

因此：

idle CPU：

    必须非常低。

不要：

    高频轮询 SQLite

不要：

    每帧 React state update

不要：

    每分钟请求 LLM

Scheduler：

    timer based

State：

    event-driven

============================================================
六十五、工程目录
============================================================

建议：

ai-world-pet/

├── src-tauri/
│
│   ├── src/
│   │   ├── main.rs
│   │
│   ├── world/
│   │   ├── engine.rs
│   │   ├── state.rs
│   │   ├── rules.rs
│   │   ├── scheduler.rs
│   │   └── time.rs
│   │
│   ├── events/
│   │   ├── event.rs
│   │   ├── candidate.rs
│   │   ├── validator.rs
│   │   ├── executor.rs
│   │   └── cooldown.rs
│   │
│   ├── character/
│   │   ├── character.rs
│   │   ├── personality.rs
│   │   ├── evidence.rs
│   │   ├── inventory.rs
│   │   ├── skills.rs
│   │   ├── goals.rs
│   │   ├── relationships.rs
│   │   ├── shared_experience.rs
│   │   └── important_people.rs
│   │
│   ├── memory/
│   │   ├── memory.rs
│   │   ├── summarizer.rs
│   │   └── markdown.rs
│   │
│   ├── llm/
│   │   ├── provider.rs
│   │   ├── openai_compatible.rs
│   │   ├── client.rs
│   │   ├── schema.rs
│   │   └── prompts.rs
│   │
│   ├── storage/
│   │   ├── db.rs
│   │   ├── migrations.rs
│   │   └── repository.rs
│   │
│   └── commands/
│       └── mod.rs
│
├── src/
│
│   ├── components/
│   │   ├── Pet/
│   │   ├── EventSidebar/
│   │   ├── CharacterPanel/
│   │   ├── ImportantEvent/
│   │   ├── Settings/
│   │   └── Inventory/
│   │
│   ├── sprite/
│   │   ├── SpriteSheet.ts
│   │   └── AnimationController.ts
│   │
│   ├── store/
│   │
│   ├── App.tsx
│   └── main.tsx
│
├── assets/
│   └── pets/
│       └── default/
│           └── sprite.png
│
├── character/
│   ├── character.md
│   ├── personality.md
│   ├── relationships.md
│   └── important_people/
│
├── world/
│   ├── rules.md
│   ├── world.json
│   ├── locations.json
│   └── npcs.json
│
├── prompts/
│   ├── event_director.md
│   ├── important_event.md
│   ├── personality_reflection.md
│   └── memory_summary.md
│
├── config/
│   └── config.example.json
│
├── data/
│
├── tests/
│
├── architecture.md
├── README.md
└── package.json

============================================================
六十六、数据模型
============================================================

至少实现：

    WorldState

    Character

    PersonalityTrait

    PersonalityEvidence

    Event

    Memory

    NPC

    Relationship

    SharedExperience

    ImportantPerson

    Item

    Skill

    Goal

============================================================
六十七、Event 数据结构
============================================================

例如：

{
    "id": "event_001",

    "timestamp":
        "2026-09-02T18:42:00",

    "world_time":
        "18:42",

    "type":
        "important_event",

    "summary":
        "她第一次主动邀请 Aoi 一起学习。",

    "importance":
        0.92,

    "location":
        "library",

    "participants": [
        "main_character",
        "aoi"
    ],

    "effects": {
        "xp": 40
    },

    "memory": {
        "important": true
    },

    "causes": [
        "event_023"
    ],

    "personality_evidence": [
        {
            "trait": "confidence",
            "delta": 3,
            "reason":
                "She initiated the interaction."
        }
    ]
}

============================================================
六十八、Important Event 生成逻辑
============================================================

重要事件评分可以类似：

    importance_score =

        goal_progress
        +
        relationship_opportunity
        +
        unresolved_story
        +
        time_since_last_important_event
        +
        character_state
        +
        random_factor
        -
        recent_important_event_penalty

当：

    score > threshold

才允许：

    important event

并且：

    daily important event count

会影响概率。

例如：

今天已经：

    2 次

则：

    后续重要事件概率显著下降。

但不是绝对禁止。

============================================================
六十九、重要事件平均每天 2 次
============================================================

最终统计：

    important_events / real_day

目标：

    ≈ 2

不是：

    exactly 2

允许：

    0
    1
    2
    3
    4

但长期：

    mean ≈ 2/day

必须把这个逻辑写入：

    Scheduler

而不是写进 Prompt 让 LLM 自己决定。

============================================================
七十、Memory Compression
============================================================

长期运行后：

    events

可能达到：

    几千条
    几万条

因此必须支持：

    memory summarization

例如：

每积累：

    50 events

生成：

    memory summary

保存：

    long-term memory

但：

    原始 events

仍然保留在 SQLite。

============================================================
七十一、启动恢复
============================================================

应用启动：

    load SQLite
        ↓
    load world
        ↓
    load character files
        ↓
    sync state
        ↓
    start scheduler
        ↓
    start pet animation

不能：

    每次启动重新生成角色。

============================================================
七十二、系统睡眠恢复
============================================================

Windows/macOS：

如果电脑：

    sleep

然后：

    wake

必须检测：

    last_update_time

例如：

睡眠：

    8h

恢复：

    不生成 8 个小时的每小时事件。

而应该：

    根据规则生成 0～2 个合理的“期间经历”。

例如：

    “她在你离开的这段时间里安静地休息了。”

重要事件：

    不应该因为睡眠而瞬间爆发。

============================================================
七十三、配置
============================================================

config/config.example.json：

{
    "llm": {
        "base_url": "https://example.com/v1",
        "model": "your-model"
    },

    "pet": {
        "sprite": {
            "path": "assets/pets/default/sprite.png",
            "columns": 8,
            "rows": 9
        }
    },

    "events": {
        "normal_check_min_minutes": 20,
        "normal_check_max_minutes": 90,

        "important_window_min_hours": 4,
        "important_window_max_hours": 8,

        "target_important_events_per_day": 2,

        "sidebar_max_events": 10
    },

    "world": {
        "real_time": true
    }
}

API Key：

    不放在这个文件。

============================================================
七十四、Settings
============================================================

Settings 至少：

    AI Provider

    Base URL

    Model

    API Key

    Test Connection

    Normal Event Frequency

    Important Event Target

    Sidebar Event Count

    Pet Scale

    Animation FPS

    Real-time Mode

    Start at Login

    Export World

    Import World

    Reset World

============================================================
七十五、Import / Export
============================================================

支持：

    Export World

导出：

    Character
    Personality
    Personality Evidence
    Events
    Memories
    NPC
    Relationships
    Shared Experiences
    Inventory
    Skills
    Goals

格式：

    JSON

同时允许导出：

    Markdown Character Files

============================================================
七十六、Reset World
============================================================

提供：

    Reset World

必须二次确认。

提示：

    This will delete all character progress,
    memories, relationships and event history.

============================================================
七十七、测试
============================================================

必须建立自动化测试。

至少：

    Sprite indexing

    Animation

    Event JSON parsing

    Event validation

    Event cooldown

    Important event probability

    Daily important event balancing

    XP calculation

    Level up

    Inventory

    Relationship

    Shared Experience

    Personality Evolution

    Personality Evidence

    Important Person detection

    Markdown synchronization

    World Rules

    World Time

    Scheduler

    SQLite persistence

    LLM failure

    API timeout

    API invalid response

    Sleep/Wake recovery

特别测试：

    App restart

必须保证：

    Character state 不丢失。

============================================================
七十八、README
============================================================

README 必须包含：

    Project Overview

    Architecture

    Windows Setup

    macOS Setup

    Development

    Build

    Release

    Sprite Format

    Character Files

    World Rules

    Custom LLM API

    API Key

    Event System

    Personality Evolution

    Important People

    Memory

    SQLite

    Import / Export

    Troubleshooting

============================================================
七十九、开发流程
============================================================

严格按照：

STEP 1

检查当前目录。

STEP 2

检查：

    Node
    npm/pnpm
    Rust
    Cargo
    Tauri

STEP 3

检查当前操作系统。

STEP 4

阅读：

    agent-pet
    agent-terrarium
    agents-in-the-office
    hermes-quest

STEP 5

建立：

    architecture.md

STEP 6

创建：

    Tauri v2 + React + Rust

STEP 7

完成：

    transparent pet

STEP 8

完成：

    8×9 sprite

STEP 9

完成：

    Sidebar

STEP 10

完成：

    SQLite

STEP 11

完成：

    World Engine

STEP 12

完成：

    Character

STEP 13

完成：

    Character Files

STEP 14

完成：

    Personality System

STEP 15

完成：

    Event Scheduler

STEP 16

完成：

    Custom LLM API

STEP 17

完成：

    Event Director

STEP 18

完成：

    Event Validation

STEP 19

完成：

    Important Event

STEP 20

完成：

    Shared Experience

STEP 21

完成：

    Important People

STEP 22

完成：

    Personality Evolution

STEP 23

完成：

    XP / Level / Inventory / Skills

STEP 24

完成：

    Memory

STEP 25

完成：

    Sleep/Wake recovery

STEP 26

测试。

STEP 27

编译 Windows / macOS target。

STEP 28

修复所有错误。

============================================================
八十、代码质量
============================================================

禁止：

    所有逻辑写进 App.tsx。

禁止：

    所有世界逻辑写在 React。

禁止：

    LLM 直接写 SQLite。

禁止：

    API Key hardcode。

禁止：

    每秒轮询。

禁止：

    每分钟调用 LLM。

禁止：

    用随机字符串代替真正的 Persistent State。

必须：

    Rust
        =
    World Engine

    SQLite
        =
    Persistent State

    Markdown
        =
    Human-readable Character Memory

    React
        =
    UI

============================================================
八十一、最终世界循环
============================================================

最终运行：

    Real Time
        ↓
    World State Update
        ↓
    Character State
        ↓
    NPC State
        ↓
    Scheduler
        ↓
    Candidate Events
        ↓
    LLM Event Director
        ↓
    Event Proposal
        ↓
    Validation
        ↓
    World Engine
        ↓
    Event Persistence
        ↓
    Memory Update
        ↓
    Shared Experience
        ↓
    Relationship Update
        ↓
    Personality Evidence
        ↓
    Personality Reflection
        ↓
    Character Profile Update
        ↓
    UI
        ├── Pet Animation
        └── Chronicle Sidebar

============================================================
八十二、最终用户体验
============================================================

第一次启动：

    用户导入 8×9 Sprite Sheet。

然后：

    设置 Base URL
    设置 Model
    设置 API Key

点击：

    Test Connection

成功。

然后：

    世界正式开始运行。

------------------------------------------------------------
第一天
------------------------------------------------------------

08:00

    她醒来。

08:20

    “她整理了一下书包，准备出门。”

12:10

    “她和同学一起吃了午饭。”

15:40

    “她在图书馆遇到了一个叫 Aoi 的女孩。”

这是：

    ⭐ Important Event

系统：

    创建 Aoi

    保存事件

    建立第一次关系

    写入 Important Memory

------------------------------------------------------------

晚上
------------------------------------------------------------

18:30

    她再次遇到 Aoi。

19:10

    两人一起学习。

系统发现：

    Shared Experience

但不立即大幅改变人格。

------------------------------------------------------------
第二天
------------------------------------------------------------

16:20

    她主动邀请 Aoi 一起学习。

这是：

    ⭐ Important Event

系统：

    Relationship +5

    Shared Experience +1

    Personality Evidence:

        confidence +3

------------------------------------------------------------
几天之后
------------------------------------------------------------

Personality Reflection：

系统发现：

    她多次主动和 Aoi 交流。

于是：

    confidence
        35 → 40

character/personality.md：

    “She has gradually become more confident
     when interacting with people she trusts.”

------------------------------------------------------------
更长时间以后
------------------------------------------------------------

Aoi 成为：

    Important Person

生成：

    character/important_people/aoi.md

里面记录：

    personality
    relationship
    shared experiences
    important memories
    relationship development

最终：

    这个人物不是一开始由 LLM 完整生成的。

而是：

    通过生活
        ↓
    事件
        ↓
    共同经历
        ↓
    关系
        ↓
    记忆
        ↓
    性格变化

逐渐形成。

============================================================
八十三、最终核心理念
============================================================

请不要把这个项目理解成：

    AI + Desktop Pet

而应该理解成：

    Persistent Character Simulation

核心是：

    Body
        +
    Character
        +
    World
        +
    Time
        +
    Memory
        +
    Relationships
        +
    Shared Experiences
        +
    Personality Evolution
        +
    LLM

最终目标：

    用户每天打开电脑，

    桌宠都还是“同一个人”。

她记得：

    昨天发生了什么。

她记得：

    谁帮助过她。

她知道：

    自己喜欢什么。

她会：

    因为过去的经历逐渐改变。

她和某个人：

    有共同经历。

因此：

    她们的关系会自然发展。

她会：

    获得物品
    学会技能
    完成目标
    获得 XP
    升级
    形成新的性格特征。

而所有这些：

    都不会因为程序重启而消失。

============================================================
八十四、最重要的约束
============================================================

再次强调：

    不要做成随机故事生成器。

    不要做成聊天机器人。

    不要做成简单的 RPG UI。

    不要让 LLM 直接控制世界。

    不要让每个事件都改变人格。

    不要让重要事件机械地每 12 小时发生。

    不要让角色每天重置。

    不要让 NPC 每次重新生成。

    不要把所有记忆塞进 Prompt。

应该建立：

    World Engine
        +
    Persistent Memory
        +
    Event System
        +
    Character Model
        +
    Personality Evolution
        +
    Shared Experience
        +
    Important People
        +
    Custom LLM

最终成为一个：

    长期生活在 Windows / macOS 桌面上的
    持久化 AI Character World。

现在开始实际创建工程。

不要只输出设计文档。

直接：

    创建文件
    编写代码
    安装依赖
    运行测试
    编译
    修复错误
    启动应用
    验证功能

直到得到一个真正可以运行的完整工程。