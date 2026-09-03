import { useEffect, useRef, useState, type MouseEvent } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { emit, listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { ChevronRight, Cog, Download, Heart, History, MapPin, Package, RefreshCw, Sparkles, X } from 'lucide-react';
import { initialState } from './simulation';
import { animationsForGrid, AnimationController, type AnimationState } from './sprite/AnimationController';
import { frameFor } from './sprite/SpriteSheet';
import type { EventRecord, PetState, WorldSnapshot } from './types';

type SettingsState = {
  baseUrl: string;
  model: string;
  apiKey: string;
  language: 'zh' | 'en';
  characterName: string;
  sidebarEvents: number;
  fps: number;
  realTime: boolean;
  characterDescription: string;
  characterExperiences: string;
  characterTags: string;
  characterInterests: string;
  characterBehavior: string;
  characterStartLocation: string;
  characterItems: string;
  characterSkills: string;
  characterAvatarFrame: number;
  characterEnergy?: number;
  characterMood?: number;
  characterHealth?: number;
  characterIntelligence?: number;
  characterFriendship?: number;
  characterCuriosity?: number;
  characterCreativity?: number;
  characterCourage?: number;
  spriteColumns: number;
  spriteRows: number;
  spriteData: string;
  spriteAtlasWidth: number;
  spriteAtlasHeight: number;
  spriteFrameWidth: number;
  spriteFrameHeight: number;
};
type LogEntry = { time: string; level: 'info' | 'error'; message: string };
type ResizeDirection = 'East' | 'North' | 'NorthEast' | 'NorthWest' | 'South' | 'SouthEast' | 'SouthWest' | 'West';
type SettingsTab = 'provider' | 'character' | 'sprite' | 'runtime' | 'reset';

function appendLog(level: LogEntry['level'], message: string) {
  const entry = { time: new Date().toISOString(), level, message };
  try {
    const previous = JSON.parse(localStorage.getItem('aoi-world-logs') || '[]') as LogEntry[];
    localStorage.setItem('aoi-world-logs', JSON.stringify([...previous, entry].slice(-100)));
  } catch {}
  (level === 'error' ? console.error : console.info)(`[aoi-world] ${message}`);
}

function Progress({ value, color = '#d66b62' }: { value: number; color?: string }) {
  return <div className="progress"><span style={{ width: `${Math.min(100, value)}%`, background: color }} /></div>;
}

function EventItem({ event }: { event: EventRecord }) {
  const important = event.type === 'important_event';
  return (
    <article className={`event ${important ? 'important' : ''}`}>
      <div className="event-time">
        {important && <Sparkles size={13} />}
        {event.timestamp}
        <span>{event.location}</span>
      </div>
      <p>{event.summary}</p>
      {important && <div className="event-tag">IMPORTANT EVENT <ChevronRight size={12} /></div>}
    </article>
  );
}

function PetStage({
  state,
  spriteUrl,
  frame,
  atlasSize,
  spriteGrid,
  onClick,
  onResize,
}: {
  state: PetState;
  spriteUrl: string;
  frame: number;
  atlasSize: { width: number; height: number };
  spriteGrid: { columns: number; rows: number };
  onClick: () => void;
  onResize: (direction: ResizeDirection, event: MouseEvent<HTMLSpanElement>) => void;
}) {
  const drag = () => { void getCurrentWindow().startDragging().catch(() => undefined); };
  const frameRect = spriteUrl ? frameFor({ width: atlasSize.width, height: atlasSize.height, columns: spriteGrid.columns, rows: spriteGrid.rows }, frame) : undefined;
  const style = spriteUrl && frameRect ? {
    width: frameRect.width,
    height: frameRect.height,
    backgroundImage: `url(${spriteUrl})`,
    backgroundSize: `${atlasSize.width}px ${atlasSize.height}px`,
    backgroundPosition: `${-frameRect.x}px ${-frameRect.y}px`,
  } : undefined;

  return (
    <section className="pet-stage">
      <div className="pet-drag-zone" data-tauri-drag-region onMouseDownCapture={event => { event.preventDefault(); drag(); }} onMouseDown={drag}>
        <button className={`sprite-pet ${spriteUrl ? 'has-sprite' : ''}`} onMouseDown={event => { event.preventDefault(); event.stopPropagation(); }} onClick={event => { event.currentTarget.blur(); onClick(); }} aria-label={`打开 ${state.name} 窗口`}>
          {spriteUrl ? <span className="sprite-frame" style={style} /> : <><div className="pet-shadow" /><div className="pet-hair" /><div className="pet-face"><i /><i /><b /></div><div className="pet-body" /><div className="pet-dress" /></>}
        </button>
      </div>
      {(['North', 'South', 'East', 'West', 'NorthEast', 'NorthWest', 'SouthEast', 'SouthWest'] as ResizeDirection[]).map(direction => (
        <span key={direction} className={`pet-resize-handle ${direction.toLowerCase()}`} onMouseDown={event => onResize(direction, event)} />
      ))}
    </section>
  );
}

function Chronicle({ state, onAiGenerate }: { state: PetState; onAiGenerate: () => void }) {
  return (
    <section className="chronicle">
      <div className="section-title">
        <div><span className="eyebrow">REAL-TIME MEMORY</span><h3>Chronicle</h3></div>
        <div className="chronicle-actions">
          <button className="outline-action" onClick={onAiGenerate} title="AI 立即生成一个事件"><Sparkles size={14} />AI 立即生成</button>
        </div>
      </div>
      <div className="event-list">{state.events.slice(0, 8).map(event => <EventItem event={event} key={event.id} />)}</div>
    </section>
  );
}

function RelationshipsView({ state }: { state: PetState }) {
  return <section className="detail-view"><div className="section-title"><div><span className="eyebrow">SOCIAL GRAPH</span><h3>人际</h3></div></div><div className="detail-list">{state.npcs.map(npc => <article className="detail-row" key={npc.id}><div className="person-avatar">{npc.avatar}</div><div><strong>{npc.name}</strong><span>{npc.role}</span></div><b className="detail-score"><Heart size={13} /> {npc.relationship}</b></article>)}</div></section>;
}

function PersonalityView({ state }: { state: PetState }) {
  return <section className="detail-view"><div className="section-title"><div><span className="eyebrow">EVOLVING CHARACTER</span><h3>性格</h3></div></div><div className="trait-list">{state.traits.map(trait => <div className="trait" key={trait.name}><span>{trait.name}</span><Progress value={trait.score} color={trait.color} /><b>{trait.score}</b></div>)}</div></section>;
}

function InventoryView({ state }: { state: PetState }) {
  return <section className="detail-view"><div className="section-title"><div><span className="eyebrow">COLLECTED OBJECTS</span><h3>物品</h3></div></div><div className="detail-list">{state.inventory.map(item => <article className="detail-row" key={item.name}><div className="item-icon"><Package size={16} /></div><div><strong>{item.name}</strong><span>{item.detail}</span></div></article>)}</div></section>;
}

function CharacterView({ state, settings, onChange, spriteUrl, atlasSize, onFrameChange }: { state: PetState; settings: SettingsState; onChange: (next: SettingsState) => void; spriteUrl: string; atlasSize: { width: number; height: number }; onFrameChange: (frame: number) => void }) {
  const [editing, setEditing] = useState(false);
  const update = (patch: Partial<SettingsState>) => onChange({ ...settings, ...patch });
  const split = (value: string) => value.split(/[,\n，、]/).map(item => item.trim()).filter(Boolean);
  const rect = spriteUrl ? frameFor({ width: atlasSize.width, height: atlasSize.height, columns: settings.spriteColumns, rows: settings.spriteRows }, settings.characterAvatarFrame) : undefined;
  const avatarStyle = rect ? { width: rect.width, height: rect.height, backgroundImage: `url(${spriteUrl})`, backgroundSize: `${atlasSize.width}px ${atlasSize.height}px`, backgroundPosition: `-${rect.x}px -${rect.y}px` } : undefined;
  const stats = [['能量', settings.characterEnergy ?? state.energy], ['心情', settings.characterMood ?? state.mood], ['体力', settings.characterHealth ?? state.health], ['智力', settings.characterIntelligence ?? state.intelligence], ['好奇心', settings.characterCuriosity ?? state.curiosity], ['社交', settings.characterFriendship ?? state.friendship]];
  return <section className="detail-view character-view">
    <div className="section-title"><div><span className="eyebrow">CHARACTER PROFILE</span><h3>人物</h3></div><button className="outline-action" onClick={() => setEditing(value => !value)}>{editing ? '完成' : '编辑'}</button></div>
    <div className="character-hero"><div className="character-avatar" style={avatarStyle}>{!avatarStyle && settings.characterName.slice(0, 1)}</div><div><h4>{settings.characterName || state.name}</h4><p>{settings.characterDescription || '还没有角色简介。'}</p><div className="tag-list">{split(settings.characterTags).map(tag => <span key={tag}>{tag}</span>)}</div></div></div>
    {!editing ? <><div className="profile-grid">{stats.map(([label, value]) => <div className="profile-stat" key={String(label)}><span>{label}</span><b>{value}</b><Progress value={Number(value)} /></div>)}</div><div className="profile-sections"><div><strong>兴趣与喜好</strong><p>{settings.characterInterests || '未设置'}</p></div><div><strong>行为倾向</strong><p>{settings.characterBehavior || '未设置'}</p></div><div><strong>初始地点</strong><p>{settings.characterStartLocation || state.location}</p></div><div><strong>背景故事</strong><p>{settings.characterExperiences || '未设置'}</p></div><div><strong>初始物品</strong><p>{split(settings.characterItems).join('、') || '无'}</p></div><div><strong>初始技能</strong><p>{split(settings.characterSkills).join('、') || '无'}</p></div></div></> : <div className="character-editor">
      <label>角色名称<input value={settings.characterName} onChange={event => update({ characterName: event.target.value })} /></label>
      <label>性格标签<input value={settings.characterTags} onChange={event => update({ characterTags: event.target.value })} placeholder="好奇, 温柔, 害羞" /></label>
      <label>兴趣与喜好<textarea value={settings.characterInterests} onChange={event => update({ characterInterests: event.target.value })} /></label>
      <label>行为倾向<textarea value={settings.characterBehavior} onChange={event => update({ characterBehavior: event.target.value })} /></label>
      <label>初始地点<input value={settings.characterStartLocation} onChange={event => update({ characterStartLocation: event.target.value })} /></label>
      <div className="settings-grid">{[['characterEnergy', '能量'], ['characterMood', '心情'], ['characterHealth', '体力'], ['characterIntelligence', '智力'], ['characterCuriosity', '好奇心'], ['characterFriendship', '社交'], ['characterCreativity', '创造力'], ['characterCourage', '勇气']].map(([key, label]) => <label key={key}>{label}<input type="number" min="0" max="100" value={settings[key as keyof SettingsState] as number} onChange={event => update({ [key]: Number(event.target.value) } as Partial<SettingsState>)} /></label>)}</div>
      <label>背景故事 / 角色简介<textarea value={settings.characterExperiences} onChange={event => update({ characterExperiences: event.target.value })} /></label>
      <label>初始物品<input value={settings.characterItems} onChange={event => update({ characterItems: event.target.value })} placeholder="笔记本, 雨伞" /></label>
      <label>初始技能<input value={settings.characterSkills} onChange={event => update({ characterSkills: event.target.value })} placeholder="阅读, 绘画" /></label>
      <label>头像帧<input type="number" min="0" max={settings.spriteColumns * settings.spriteRows - 1} value={settings.characterAvatarFrame} onChange={event => { const frame = Math.max(0, Number(event.target.value)); update({ characterAvatarFrame: frame }); onFrameChange(frame); }} /></label>
    </div>}
    <div className="profile-dimensions"><strong>五维成长属性</strong><div className="profile-grid">{[['智力', settings.characterIntelligence ?? state.intelligence], ['好奇心', settings.characterCuriosity ?? state.curiosity], ['社交', settings.characterFriendship ?? state.friendship], ['创造力', settings.characterCreativity ?? state.creativity], ['勇气', settings.characterCourage ?? state.courage]].map(([label, value]) => <div className="profile-stat" key={String(label)}><span>{label}</span><b>{value}</b><Progress value={Number(value)} /></div>)}</div></div>
  </section>;
}

function MapView({ state }: { state: PetState }) {
  const locations = state.knownLocations.length ? state.knownLocations : [{ name: state.location, description: '', exploration: 0, rarity: 'common' }];
  const eventLocations = new Set(state.events.slice(0, 8).map(event => event.location).filter(Boolean));
  return <section className="detail-view map-view">
    <div className="section-title"><div><span className="eyebrow">WORLD LOCATIONS</span><h3>地图</h3></div><span className="map-current"><MapPin size={13} /> 当前：{state.location}</span></div>
    <div className="location-list">{locations.map(location => <article className={`location-row ${location.name === state.location ? 'current' : ''}`} key={location.name}><div className="location-icon"><MapPin size={16} /></div><div><strong>{location.name}</strong><span>{location.description || '尚无场景描述'}</span><small>探索度 {location.exploration}% · {location.rarity}</small></div>{location.name === state.location && <b>当前所在</b>}{eventLocations.has(location.name) && <em>事件发生地</em>}</article>)}</div>
    <div className="map-event-note"><strong>最近事件场景</strong>{state.events.slice(0, 5).map(event => <p key={event.id}><span>{event.timestamp}</span>{event.location || '未知地点'}：{event.summary}</p>)}</div>
  </section>;
}

function SettingsPanel({ settings, onChange, onClose, onTest, onExport, onReset, onSprite, onLogs }: {
  settings: SettingsState;
  onChange: (next: SettingsState) => void;
  onClose: () => void;
  onTest: () => void;
  onExport: () => void;
  onReset: () => void;
  onSprite: (file: File) => void;
  onLogs: () => void;
}) {
  const [tab, setTab] = useState<SettingsTab>('provider');
  const update = (patch: Partial<SettingsState>) => onChange({ ...settings, ...patch });
  return (
    <div className="modal-backdrop" onMouseDown={onClose}>
      <section className="settings-panel" onMouseDown={event => event.stopPropagation()}>
        <header><div><span className="eyebrow">WORLD CONFIGURATION</span><h2>Settings</h2></div><button className="icon-button" onClick={onClose}><X size={18} /></button></header>
        <nav className="settings-tabs">
          {([['provider', 'API / Model'], ['character', 'Character'], ['sprite', 'Sprite / Animation'], ['runtime', 'Runtime'], ['reset', 'Reset']] as const).map(([key, label]) => <button key={key} className={tab === key ? 'active' : ''} onClick={() => setTab(key)}>{label}</button>)}
        </nav>
        {tab === 'provider' && <>
          <label>Base URL<input value={settings.baseUrl} onChange={event => update({ baseUrl: event.target.value })} placeholder="https://example.com/v1" /></label>
          <label>Model<input value={settings.model} onChange={event => update({ model: event.target.value })} placeholder="your-model" /></label>
          <label>API Key<input type="password" value={settings.apiKey} onChange={event => update({ apiKey: event.target.value })} /></label>
          <label>Output language<select value={settings.language} onChange={event => update({ language: event.target.value as 'zh' | 'en' })}><option value="zh">中文优先</option><option value="en">English</option></select></label>
          <div className="settings-actions settings-page-actions"><button className="outline-action" onClick={onTest}><RefreshCw size={14} /> TEST CONNECTION</button></div>
        </>}
        {tab === 'character' && <>
          <label>Character name<input value={settings.characterName} onChange={event => update({ characterName: event.target.value })} placeholder="Character name" /></label>
          <label>Personality description<textarea value={settings.characterDescription} onChange={event => update({ characterDescription: event.target.value })} placeholder="Describe personality, habits and preferences." /></label>
          <label>Experiences and background<textarea value={settings.characterExperiences} onChange={event => update({ characterExperiences: event.target.value })} placeholder="Describe important experiences and memories." /></label>
        </>}
        {tab === 'sprite' && <>
          <label className="settings-section-title">Sprite sheet</label>
          <div className="settings-grid">
            <label>Columns<select value={settings.spriteColumns} onChange={event => { const columns = Number(event.target.value); update({ spriteColumns: columns, spriteFrameWidth: settings.spriteAtlasWidth / columns }); }}><option value="8">8 columns</option><option value="10">10 columns</option></select></label>
            <label>Rows<select value={settings.spriteRows} onChange={event => { const rows = Number(event.target.value); update({ spriteRows: rows, spriteFrameHeight: settings.spriteAtlasHeight / rows }); }}><option value="9">9 rows</option><option value="10">10 rows</option></select></label>
          </div>
          <label className="outline-action sprite-picker">IMPORT PNG SPRITE<input hidden type="file" accept="image/png" onChange={event => event.target.files?.[0] && onSprite(event.target.files[0])} /></label>
          <label>Animation FPS<input type="number" min="0.5" max="10" step="0.5" value={settings.fps} onChange={event => update({ fps: Number(event.target.value) })} /></label>
        </>}
        {tab === 'runtime' && <>
          <label>Chronicle events<select value={settings.sidebarEvents} onChange={event => update({ sidebarEvents: Number(event.target.value) })}><option value="5">5</option><option value="8">8</option><option value="10">10</option><option value="20">20</option></select></label>
          <label className="check-row"><input type="checkbox" checked={settings.realTime} onChange={event => update({ realTime: event.target.checked })} /> Real-time mode</label>
          <div className="settings-actions settings-page-actions"><button className="outline-action" onClick={onLogs}>OPEN LOGS</button><button className="outline-action" onClick={onExport}><Download size={14} /> EXPORT</button></div>
        </>}
        {tab === 'reset' && <div className="reset-page"><h3>Reset Character World</h3><p>Restore the initial character, five starting locations, one friend, one backpack and an empty event history. API, model and key settings remain unchanged.</p><button className="primary-action reset-button" onClick={onReset}><RefreshCw size={14} /> RESET CHARACTER WORLD</button></div>}
        <div className="settings-actions">
          <button className="primary-action" onClick={onClose}>SAVE</button>
        </div>
      </section>
    </div>
  );
}

function LogsPanel({ close }: { close: () => void }) {
  const [logs, setLogs] = useState<LogEntry[]>(() => { try { return JSON.parse(localStorage.getItem('aoi-world-logs') || '[]'); } catch { return []; } });
  const clear = () => { localStorage.removeItem('aoi-world-logs'); setLogs([]); };
  return <div className="modal-backdrop" onMouseDown={close}><section className="settings-panel log-panel" onMouseDown={event => event.stopPropagation()}><header><div><span className="eyebrow">DIAGNOSTICS</span><h2>Logs</h2></div><button className="icon-button" onClick={close}><X size={18} /></button></header><div className="log-list">{logs.slice().reverse().map((entry, index) => <div className={`log-entry ${entry.level}`} key={`${entry.time}-${index}`}><time>{entry.time}</time><p>{entry.message}</p></div>)}</div><div className="settings-actions"><button className="text-action" onClick={clear}>CLEAR</button><button className="primary-action" onClick={close}>CLOSE</button></div></section></div>;
}

export default function App() {
  const [state, setState] = useState<PetState>(() => { try { return JSON.parse(localStorage.getItem('aoi-world-state') || 'null') || initialState; } catch { return initialState; } });
  const [settings, setSettings] = useState<SettingsState>(() => { try { return { baseUrl: '', model: '', language: 'zh', characterName: initialState.name, sidebarEvents: 8, fps: 8, realTime: true, apiKey: '', characterDescription: '', characterExperiences: '', characterTags: '好奇, 温柔', characterInterests: '', characterBehavior: '', characterStartLocation: initialState.location, characterItems: '', characterSkills: '', characterAvatarFrame: 0, characterHealth: 100, characterIntelligence: 50, characterFriendship: 0, characterCuriosity: 50, spriteColumns: 8, spriteRows: 9, spriteData: '', spriteAtlasWidth: 768, spriteAtlasHeight: 936, spriteFrameWidth: 96, spriteFrameHeight: 104, ...JSON.parse(localStorage.getItem('aoi-world-settings') || '{}') }; } catch { return { baseUrl: '', model: '', apiKey: '', language: 'zh', characterName: initialState.name, sidebarEvents: 8, fps: 8, realTime: true, characterDescription: '', characterExperiences: '', characterTags: '好奇, 温柔', characterInterests: '', characterBehavior: '', characterStartLocation: initialState.location, characterItems: '', characterSkills: '', characterAvatarFrame: 0, characterHealth: 100, characterIntelligence: 50, characterFriendship: 0, characterCuriosity: 50, spriteColumns: 8, spriteRows: 9, spriteData: '', spriteAtlasWidth: 768, spriteAtlasHeight: 936, spriteFrameWidth: 96, spriteFrameHeight: 104 }; } });
  const [running, setRunning] = useState(true);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [logsOpen, setLogsOpen] = useState(false);
  const [toast, setToast] = useState('');
  const [spriteUrl, setSpriteUrl] = useState(() => { try { return JSON.parse(localStorage.getItem('aoi-world-settings') || '{}').spriteData || ''; } catch { return ''; } });
  const [spriteFrame, setSpriteFrame] = useState(0);
  const animationRef = useRef(new AnimationController(animationsForGrid(settings.spriteColumns, settings.spriteRows)));
  const generatingRef = useRef(false);
  const [chronicleView, setChronicleView] = useState<'事件' | '人际' | '人物' | '地图' | '物品' | '设置'>('事件');
  const windowLabel = getCurrentWindow().label;
  const isChronicleWindow = windowLabel === 'chronicle';

  const notify = (message: string) => {
    appendLog('info', message);
    setToast(message);
    window.setTimeout(() => setToast(''), 2400);
  };

  const handleWindowDrag = (event: MouseEvent<HTMLDivElement>) => {
    if ((event.target as HTMLElement).closest('button,input,textarea,select')) return;
    void getCurrentWindow().startDragging().catch(() => undefined);
  };
  const handleWindowResize = (direction: ResizeDirection, event: MouseEvent<HTMLSpanElement>) => {
    event.stopPropagation();
    void getCurrentWindow().startResizeDragging(direction).catch(error => appendLog('error', `resize failed: ${String(error)}`));
  };

  useEffect(() => { localStorage.setItem('aoi-world-state', JSON.stringify(state)); }, [state]);
  useEffect(() => { localStorage.setItem('aoi-world-settings', JSON.stringify(settings)); }, [settings]);
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    void (async () => {
      try {
        const snapshot = await invoke<WorldSnapshot>('get_world');
        setState(current => ({ ...snapshot, name: settings.characterName.trim() || current.name || snapshot.name }));
        appendLog('info', 'backend connected');
        unlisten = await listen<WorldSnapshot>('world-updated', event => {
          appendLog('info', `world-updated events=${event.payload.events.length}`);
          setState(event.payload);
        });
      } catch (error) {
        appendLog('error', `backend unavailable: ${String(error)}`);
      }
    })();
    return () => unlisten?.();
  }, []);
  useEffect(() => {
    const name = settings.characterName.trim();
    if (name) setState(current => current.name === name ? current : { ...current, name });
  }, [settings.characterName]);
  useEffect(() => {
    if (!spriteUrl || !running) return;
    const animations = animationsForGrid(settings.spriteColumns, settings.spriteRows);
    const configuredFps = Math.max(0.5, Math.min(10, Number(settings.fps) || 8));
    Object.values(animations).forEach(animation => {
      animation.minFps = configuredFps;
      animation.maxFps = configuredFps;
    });
    animationRef.current = new AnimationController(animations);
    let frameId = 0;
    const started = performance.now();
    const animate = (now: number) => {
      const source = state.animation;
      const animation: AnimationState = source === 'idle' ? 'Idle' : source === 'social' ? 'Band' : source === 'celebrating' ? 'Special' : source === 'thinking' ? 'Robot' : source === 'happy' ? 'Walk' : 'Idle';
      setSpriteFrame(animationRef.current.tick(animation, now - started));
      frameId = requestAnimationFrame(animate);
    };
    frameId = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(frameId);
  }, [spriteUrl, running, settings.fps, settings.spriteColumns, settings.spriteRows, state.animation]);
  useEffect(() => { let unlisten: (() => void) | undefined; void listen<{ data: string }>('sprite-updated', event => { setSpriteUrl(event.payload.data); setSpriteFrame(0); }).then(stop => { unlisten = stop; }); return () => unlisten?.(); }, []);
  useEffect(() => { if (!isChronicleWindow) return; let unlisten: (() => void) | undefined; void listen('open-settings', () => setSettingsOpen(true)).then(stop => { unlisten = stop; }); return () => unlisten?.(); }, [isChronicleWindow]);

  const generateAiEvent = async () => {
    if (generatingRef.current) return;
    if (!settings.baseUrl.trim() || !settings.model.trim()) {
      const message = '请先填写 Base URL 和 Model';
      appendLog('error', message);
      notify(message);
      return;
    }
    generatingRef.current = true;
    appendLog('info', 'starting AI event generation');
    try {
      setState(await invoke<WorldSnapshot>('generate_event', {
        baseUrl: settings.baseUrl,
        model: settings.model,
        apiKey: settings.apiKey || null,
        language: settings.language,
        characterContext: `Name: ${settings.characterName}\nTags: ${settings.characterTags}\nInterests: ${settings.characterInterests}\nBehavior: ${settings.characterBehavior}\nStart location: ${settings.characterStartLocation}\nStats: energy=${settings.characterEnergy ?? state.energy}, mood=${settings.characterMood ?? state.mood}, health=${settings.characterHealth ?? state.health}, intelligence=${settings.characterIntelligence ?? state.intelligence}, curiosity=${settings.characterCuriosity ?? state.curiosity}, social=${settings.characterFriendship ?? state.friendship}, creativity=${settings.characterCreativity ?? state.creativity}, courage=${settings.characterCourage ?? state.courage}\nInitial items: ${settings.characterItems}\nInitial skills: ${settings.characterSkills}\nPersonality: ${settings.characterDescription}\nExperiences: ${settings.characterExperiences}`,
      }));
      notify('AI event generated');
    } catch (error) {
      const message = `AI event failed: ${String(error)}`;
      appendLog('error', message);
      notify(message);
    } finally {
      generatingRef.current = false;
    }
  };

  useEffect(() => {
    if (!isChronicleWindow || !running) return;
    const timer = window.setInterval(() => { void generateAiEvent(); }, 600000);
    return () => window.clearInterval(timer);
  }, [isChronicleWindow, running, settings.baseUrl, settings.model, settings.apiKey, settings.language, settings.characterDescription, settings.characterExperiences]);

  const resetWorld = async () => {
    if (!window.confirm('Reset the character world? API, model and key settings will be kept.')) return;
    try {
      await invoke('reset_world');
      const snapshot = await invoke<WorldSnapshot>('get_world');
      setState(snapshot);
      setSpriteUrl('');
      setSpriteFrame(0);
      setSettings(current => ({
        ...current,
        characterName: 'Aoi',
        characterDescription: '',
        characterExperiences: '',
        characterTags: '好奇, 温柔',
        characterInterests: '',
        characterBehavior: '',
        characterStartLocation: '家',
        characterItems: '书包',
        characterSkills: '',
        characterAvatarFrame: 0,
        characterEnergy: 100,
        characterMood: 100,
        characterHealth: 100,
        characterIntelligence: 10,
        characterFriendship: 10,
        characterCuriosity: 10,
        characterCreativity: 10,
        characterCourage: 10,
        spriteData: '',
      }));
      notify('Character world reset');
    } catch (error) {
      appendLog('error', `reset failed: ${String(error)}`);
      notify('Reset failed');
    }
  };

  const exportWorld = () => {
    const link = document.createElement('a');
    link.href = URL.createObjectURL(new Blob([JSON.stringify({ exportedAt: new Date().toISOString(), state }, null, 2)], { type: 'application/json' }));
    link.download = 'aoi-world-export.json';
    link.click();
    notify('世界数据已导出');
  };

  const testProvider = async () => {
    if (!settings.baseUrl.trim() || !settings.model.trim()) { notify('请填写 Base URL 和 Model'); return; }
    try {
      const result = await invoke<string>('test_provider', { baseUrl: settings.baseUrl, model: settings.model, apiKey: settings.apiKey || null });
      appendLog('info', `provider test success: ${result}`);
      notify(result);
    } catch (error) {
      const message = `Provider test failed: ${String(error)}`;
      appendLog('error', message);
      notify(message);
    }
  };

  const toggleChronicle = async () => { try { await invoke('toggle_chronicle'); } catch { notify('事件栏窗口不可用'); } };

  const importSprite = (file: File) => {
    if (file.type !== 'image/png') { notify('精灵图必须是 PNG'); return; }
    const image = new Image();
    image.onload = () => {
      if (image.width < settings.spriteColumns || image.height < settings.spriteRows) { notify('精灵图尺寸太小，无法切分当前网格'); return; }
      const reader = new FileReader();
      reader.onload = () => {
        const data = String(reader.result);
        const size = { width: image.width / settings.spriteColumns, height: image.height / settings.spriteRows };
        setSpriteUrl(data);
        setSpriteFrame(0);
        setSettings(current => ({ ...current, spriteData: data, spriteAtlasWidth: image.width, spriteAtlasHeight: image.height, spriteFrameWidth: size.width, spriteFrameHeight: size.height }));
        void emit('sprite-updated', { data, width: size.width, height: size.height });
        notify('精灵图已替换并保存');
      };
      reader.readAsDataURL(file);
    };
    image.src = URL.createObjectURL(file);
  };

  const chronicleContent = chronicleView === '事件'
    ? <Chronicle state={state} onAiGenerate={generateAiEvent} />
    : chronicleView === '人际'
      ? <RelationshipsView state={state} />
      : chronicleView === '人物'
        ? <CharacterView state={state} settings={settings} onChange={setSettings} spriteUrl={spriteUrl} atlasSize={{ width: settings.spriteAtlasWidth, height: settings.spriteAtlasHeight }} onFrameChange={setSpriteFrame} />
        : chronicleView === '地图'
          ? <MapView state={state} />
        : chronicleView === '物品'
          ? <InventoryView state={state} />
          : <SettingsPanel settings={settings} onChange={setSettings} onClose={() => setChronicleView('事件')} onTest={testProvider} onExport={exportWorld} onReset={resetWorld} onSprite={importSprite} onLogs={() => setLogsOpen(true)} />;

  if (isChronicleWindow) {
    return (
      <div className="chronicle-window" data-tauri-drag-region onMouseDown={handleWindowDrag}>
        <div className="chronicle-shell">
          <nav className="chronicle-nav">
            {([['事件', History], ['人际', Heart], ['人物', Sparkles], ['地图', MapPin], ['物品', Package], ['设置', Cog]] as const).map(([label, Icon]) => <button className={chronicleView === label ? 'active' : ''} onClick={() => setChronicleView(label)} title={label} key={label}><Icon size={16} /><span>{label}</span></button>)}
          </nav>
          <main className="chronicle-content">{chronicleContent}</main>
        </div>
        {logsOpen && <LogsPanel close={() => setLogsOpen(false)} />}
        {settingsOpen && <SettingsPanel settings={settings} onChange={setSettings} onClose={() => setSettingsOpen(false)} onTest={testProvider} onExport={exportWorld} onReset={resetWorld} onSprite={importSprite} onLogs={() => setLogsOpen(true)} />}
        {toast && <div className="toast"><Sparkles size={14} />{toast}</div>}
      </div>
    );
  }

  return <div className="pet-only" data-tauri-drag-region onMouseDown={handleWindowDrag}><PetStage state={state} spriteUrl={spriteUrl} frame={spriteFrame} atlasSize={{ width: settings.spriteAtlasWidth, height: settings.spriteAtlasHeight }} spriteGrid={{ columns: settings.spriteColumns, rows: settings.spriteRows }} onClick={toggleChronicle} onResize={handleWindowResize} />{toast && <div className="toast"><Sparkles size={14} />{toast}</div>}</div>;
}
