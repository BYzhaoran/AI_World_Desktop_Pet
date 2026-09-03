export type AnimationState = 'Idle' | 'Walk' | 'Run' | 'Robot' | 'Band' | 'Special';
export type AnimationName = AnimationState;
export interface AnimationConfig { frames: number[]; minFps: number; maxFps: number; loop: boolean; pingPong?: boolean; cooldownMs?: number; minDurationMs?: number; maxDurationMs?: number; }

export function animationsForGrid(columns: number, rows: number): Record<AnimationState, AnimationConfig> {
  const rowFrames = (from: number, to: number) => Array.from({ length: Math.max(0, to - from) }, (_, row) => Array.from({ length: columns }, (_, column) => (from + row) * columns + column)).flat();
  const ranges: Record<AnimationState, [number, number]> = rows >= 10
    ? { Idle: [0, 2], Walk: [2, 3], Run: [3, 4], Robot: [4, 6], Band: [6, 8], Special: [8, 10] }
    : { Idle: [0, 1], Walk: [1, 2], Run: [2, 3], Robot: [3, 4], Band: [4, 5], Special: [5, rows] };
  return {
    Idle: { frames: rowFrames(...ranges.Idle), minFps: 8, maxFps: 10, loop: true, pingPong: true },
    Walk: { frames: rowFrames(...ranges.Walk), minFps: 12, maxFps: 16, loop: false, cooldownMs: 700 },
    Run: { frames: rowFrames(...ranges.Run), minFps: 16, maxFps: 20, loop: false, cooldownMs: 1000 },
    Robot: { frames: rowFrames(...ranges.Robot), minFps: 12, maxFps: 16, loop: false, cooldownMs: 1200 },
    Band: { frames: rowFrames(...ranges.Band), minFps: 12, maxFps: 16, loop: false, cooldownMs: 1200 },
    Special: { frames: rowFrames(...ranges.Special), minFps: 10, maxFps: 16, loop: false, cooldownMs: 1800 },
  };
}

export class AnimationController {
  private current: AnimationName = 'Idle';
  private finished = false;
  private startedAt = 0;
  private durationMs = 0;
  private frameDurationMs = 125;
  private cooldownUntil = 0;
  private selectedFps = 8;
  constructor(private readonly animations: Record<string, AnimationConfig>) {}
  tick(name: AnimationName, nowMs: number): number {
    const requested = this.animations[name] ? name : 'Idle';
    const config = this.animations[requested] ?? this.animations.Idle;
    if (!config || config.frames.length === 0) return 0;
    if (requested !== this.current && nowMs >= this.cooldownUntil) this.start(requested, config, nowMs);
    if (this.finished && requested !== 'Idle') {
      const idle = this.animations.Idle;
      if (idle) this.start('Idle', idle, nowMs);
    }
    const elapsed = Math.max(0, Math.min(nowMs - this.startedAt, this.durationMs || Infinity));
    const framePosition = Math.floor(elapsed / this.frameDurationMs);
    if (!config.loop && framePosition >= config.frames.length) {
      this.finished = true;
      return config.frames[config.frames.length - 1];
    }
    const pingPongLength = config.frames.length * 2;
    const rawPosition = config.loop ? framePosition % (config.pingPong ? pingPongLength : config.frames.length) : framePosition;
    const pingPongPosition = config.pingPong && config.frames.length > 1
      ? rawPosition < config.frames.length ? rawPosition : pingPongLength - 1 - rawPosition
      : rawPosition;
    return config.frames[Math.max(0, Math.min(config.frames.length - 1, pingPongPosition))];
  }
  reset(): void { this.current = 'Idle'; this.finished = false; this.startedAt = 0; this.durationMs = 0; this.cooldownUntil = 0; this.selectedFps = 8; this.frameDurationMs = 125; }
  private start(name: AnimationName, config: AnimationConfig, nowMs: number) {
    this.current = name;
    this.finished = false;
    this.startedAt = nowMs;
    this.selectedFps = config.minFps + Math.random() * Math.max(0, config.maxFps - config.minFps);
    this.frameDurationMs = 1000 / Math.max(0.5, this.selectedFps);
    const sequenceLength = config.pingPong ? config.frames.length * 2 : config.frames.length;
    const min = config.minDurationMs ?? (sequenceLength * 1000 / Math.max(0.5, config.maxFps));
    const max = config.maxDurationMs ?? (sequenceLength * 1000 / Math.max(0.5, config.minFps));
    this.durationMs = min + Math.random() * Math.max(0, max - min);
    this.cooldownUntil = nowMs + (config.cooldownMs ?? 0);
  }
}
