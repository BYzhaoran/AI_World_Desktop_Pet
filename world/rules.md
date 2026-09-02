# World Rules

## Time
World time follows real local time. One real minute equals one world minute.

## Events
Normal events are short. Important events are meaningful and may create memories, relationships, items, skills, goal progress, or personality evidence. The scheduler uses probability, cooldown, context, elapsed time, and daily balance, targeting approximately two important events per real day without a fixed timer.

## Authority
The LLM is an Event Director. It may observe and propose structured JSON only. The World Engine validates, mutates, and persists state.

## Personality
Personality changes are gradual, clamped to small deltas, and require evidence from meaningful events or repeated shared experiences.

## Persistence
SQLite is the structured source of truth. Markdown files are human-readable and user-editable memory. The latest Markdown is reloaded before event generation and is never silently overwritten.

## Offline
When the provider is unavailable, deterministic normal events may continue. Offline events cannot create major relationship or personality changes.
