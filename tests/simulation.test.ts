import { describe, expect, it } from 'vitest';
import { applyEvent, createOfflineEvent, initialState } from '../src/simulation';
describe('world simulation',()=>{
 it('creates deterministic offline events without major effects',()=>{const e=createOfflineEvent(initialState);expect(e.type).toBe('normal_event');expect(e.participants).toEqual(['main']);});
 it('applies XP and clamps energy',()=>{const s={...initialState,energy:1};const n=applyEvent(s,createOfflineEvent(s));expect(n.energy).toBe(0);expect(n.events.length).toBe(s.events.length+1);});
 it('starts without seeded relationships or events',()=>{expect(initialState.npcs).toEqual([]);expect(initialState.events).toEqual([]);});
});
