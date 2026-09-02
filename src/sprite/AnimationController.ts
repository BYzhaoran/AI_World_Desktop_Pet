export type AnimationName = 'idle' | 'walking' | 'thinking' | 'happy' | 'sad' | 'sleepy' | 'surprised' | 'working' | 'important_event' | 'social' | 'celebrating';
export interface AnimationConfig { frames: number[]; fps: number; loop: boolean; pingPong?: boolean; }
export class AnimationController {
  private elapsed = 0;
  private position = 0;
  constructor(private readonly animations: Record<string, AnimationConfig>) {}
  tick(name: AnimationName, deltaMs: number): number { const config = this.animations[name] ?? this.animations.idle; if (!config || config.frames.length === 0) return 0; this.elapsed += deltaMs; const interval = 1000 / Math.max(1, config.fps); while (this.elapsed >= interval) { this.elapsed -= interval; this.position += 1; if (this.position >= config.frames.length) this.position = config.loop ? 0 : config.frames.length - 1; } const index = config.pingPong && Math.floor(this.position / config.frames.length) % 2 ? config.frames.length - 1 - (this.position % config.frames.length) : this.position; return config.frames[index]; }
  reset(): void { this.elapsed = 0; this.position = 0; }
}
