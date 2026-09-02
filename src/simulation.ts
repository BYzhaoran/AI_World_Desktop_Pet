import type { EventRecord, PetState } from './types';

const seedEvents: EventRecord[] = [
  { id: 'event-141', timestamp: '18:42', type: 'important_event', summary: '她在图书馆主动邀请 Aoi 一起学习。', location: '图书馆', importance: .94, participants: ['main', 'aoi'], causes: ['event-134'] },
  { id: 'event-140', timestamp: '16:20', type: 'social_event', summary: 'Aoi 在窗边和她分享了最近画的雨景。', location: '图书馆', importance: .58, participants: ['main', 'aoi'], causes: ['event-128'] },
  { id: 'event-134', timestamp: '14:05', type: 'activity_event', summary: '她整理了笔记，完成了今天的阅读计划。', location: '房间', importance: .32, participants: ['main'], causes: [] },
  { id: 'event-128', timestamp: '12:10', type: 'normal_event', summary: '她在窗边安静地吃了午饭，外面开始下雨。', location: '房间', importance: .2, participants: ['main'], causes: [] },
  { id: 'event-121', timestamp: '09:10', type: 'important_event', summary: '她第一次在图书馆遇到了 Aoi。', location: '图书馆', importance: .89, participants: ['main', 'aoi'], causes: [] },
];

export const initialState: PetState = { name: 'Aoi', level: 3, xp: 284, nextXp: 450, mood: 72, energy: 61, location: '图书馆', weather: '小雨 · 18°C', status: '正在阅读', animation: 'idle', traits: [{name:'好奇',score:78,color:'#f0a44b'},{name:'善良',score:82,color:'#d66b62'},{name:'自信',score:42,color:'#5c9a9b'},{name:'专注',score:66,color:'#7582b6'}], skills: [{name:'阅读',level:4,xp:72},{name:'绘画',level:2,xp:38},{name:'专注',level:3,xp:61}], inventory: [{name:'旧书签',detail:'Aoi 送的纪念品',icon:'bookmark'},{name:'雨伞',detail:'透明的蓝色雨伞',icon:'umbrella'},{name:'笔记本',detail:'记录着她的想法',icon:'notebook'}], goals: [{name:'读完 5 本书',progress:3,target:5},{name:'学会画画',progress:42,target:100},{name:'成为更好的朋友',progress:68,target:100}], npcs: [{id:'aoi',name:'Aoi',role:'同学 · 朋友',relationship:42,stage:'friend',avatar:'A'},{id:'yuki',name:'Yuki',role:'邻居 · 熟人',relationship:18,stage:'acquaintance',avatar:'Y'},{id:'ren',name:'Ren',role:'图书管理员',relationship:9,stage:'acquaintance',avatar:'R'}], events: seedEvents };

export function createOfflineEvent(state: PetState): EventRecord {
  const options = ['她安静地翻了几页书。', '她在房间里整理了一下东西。', '她望着窗外的雨，想起了一个温柔的下午。'];
  return { id: `event-${Date.now()}`, timestamp: new Date().toLocaleTimeString('zh-CN',{hour:'2-digit',minute:'2-digit'}), type: 'normal_event', summary: options[state.events.length % options.length], location: state.location, importance: .2, participants: ['main'], causes: state.events[0] ? [state.events[0].id] : [] };
}

export function applyEvent(state: PetState, event: EventRecord): PetState { const xp = event.type === 'important_event' ? 40 : 5; return {...state, xp: state.xp + xp >= state.nextXp ? state.xp + xp - state.nextXp : state.xp + xp, level: state.xp + xp >= state.nextXp ? state.level + 1 : state.level, mood: Math.min(100, state.mood + (event.type === 'important_event' ? 5 : 1)), energy: Math.max(0, state.energy - 3), events: [event, ...state.events]}; }
