import type { EventRecord, PetState } from './types';

export const initialState: PetState = { name: 'Aoi', level: 1, xp: 0, nextXp: 100, mood: 70, energy: 70, health: 100, intelligence: 50, friendship: 0, curiosity: 50, creativity: 50, courage: 50, money: 0, location: 'Home', weather: 'Clear', status: 'Resting', animation: 'idle', traits: [{name:'好奇',score:50,color:'#f0a44b'},{name:'善良',score:50,color:'#d66b62'},{name:'自信',score:50,color:'#5c9a9b'},{name:'专注',score:50,color:'#7582b6'}], skills: [], inventory: [], goals: [], npcs: [], knownLocations: [{name:'Home',description:'A quiet place to rest.',exploration:35,rarity:'common'}], events: [], eventThreads: [] };

export function createOfflineEvent(state: PetState): EventRecord {
  const options = ['她安静地翻了几页书。', '她在房间里整理了一下东西。', '她望着窗外的雨，想起了一个温柔的下午。'];
  return { id: `event-${Date.now()}`, timestamp: new Date().toLocaleTimeString('zh-CN',{hour:'2-digit',minute:'2-digit'}), type: 'normal_event', summary: options[state.events.length % options.length], location: state.location, importance: .2, participants: ['main'], causes: state.events[0] ? [state.events[0].id] : [] };
}

export function applyEvent(state: PetState, event: EventRecord): PetState { const xp = event.type === 'important_event' ? 40 : 5; return {...state, xp: state.xp + xp >= state.nextXp ? state.xp + xp - state.nextXp : state.xp + xp, level: state.xp + xp >= state.nextXp ? state.level + 1 : state.level, mood: Math.min(100, state.mood + (event.type === 'important_event' ? 5 : 1)), energy: Math.max(0, state.energy - 3), events: [event, ...state.events]}; }
