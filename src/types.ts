export type EventType = 'normal_event' | 'social_event' | 'activity_event' | 'important_event' | 'milestone_event' | 'no_event';
export type PetMood = 'idle' | 'thinking' | 'happy' | 'sleepy' | 'social' | 'celebrating';
export interface EventRecord { id: string; timestamp: string; type: EventType; summary: string; location: string; importance: number; participants: string[]; causes: string[]; }
export interface Trait { name: string; score: number; color: string; }
export interface Npc { id: string; name: string; role: string; relationship: number; stage: string; avatar: string; }
export interface PetState { name: string; level: number; xp: number; nextXp: number; mood: number; energy: number; location: string; weather: string; status: string; animation: PetMood; traits: Trait[]; skills: {name: string; level: number; xp: number}[]; inventory: {name: string; detail: string; icon: string}[]; goals: {name: string; progress: number; target: number}[]; npcs: Npc[]; events: EventRecord[]; }
