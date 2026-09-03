export type EventType = 'normal_event' | 'social_event' | 'activity_event' | 'weather_event' | 'discovery_event' | 'item_event' | 'skill_event' | 'relationship_event' | 'important_event' | 'milestone_event' | 'level_up' | 'no_event';
export type PetMood = 'idle' | 'thinking' | 'happy' | 'sleepy' | 'social' | 'celebrating';
export interface EventRecord { id: string; timestamp: string; type: EventType; summary: string; location: string; importance: number; participants: string[]; causes: string[]; }
export type EventThreadStatus = 'planned' | 'active' | 'paused' | 'completed' | 'interrupted' | 'failed' | 'abandoned';
export interface EventEffects { energy?: number; mood?: number; health?: number; intelligence?: number; friendship?: number; curiosity?: number; creativity?: number; courage?: number; money?: number; exploration?: number; xp?: number; item?: unknown; }
export interface EventProgress { id: string; threadId: string; timestamp: string; summary: string; progress: number; state: EventThreadStatus; effects?: EventEffects | null; }
export interface EventThread { id: string; title: string; summary: string; type: EventType; startTime: string; lastUpdateTime: string; endTime: string | null; estimatedDuration: number; actualDuration: number | null; status: EventThreadStatus; progress: number; importance: number; location: string; participants: string[]; updates: EventProgress[]; }
export interface Trait { name: string; score: number; color: string; }
export interface Npc { id: string; name: string; role: string; relationship: number; stage: string; avatar: string; personality: string; favoriteItem: string; homeLocation: string; }
export interface Location { name: string; description: string; exploration: number; rarity: string; }
export interface WorldSnapshot extends PetState {
  worldTime: string;
  lastUpdate: string;
  importantToday: number;
  nextNormalCheck: number | null;
  memoryContext: string;
  health: number;
  intelligence: number;
  friendship: number;
  curiosity: number;
  creativity: number;
  courage: number;
  money: number;
  knownLocations: Location[];
  dayCount: number;
  totalPlayTime: number;
  currentBehavior: string;
}
export interface PetState { name: string; level: number; xp: number; nextXp: number; mood: number; energy: number; health: number; intelligence: number; friendship: number; curiosity: number; creativity: number; courage: number; money: number; location: string; weather: string; status: string; animation: PetMood; traits: Trait[]; skills: {name: string; level: number; xp: number}[]; inventory: {name: string; detail: string; icon: string}[]; goals: {name: string; progress: number; target: number}[]; npcs: Npc[]; knownLocations: Location[]; events: EventRecord[]; eventThreads: EventThread[]; }
